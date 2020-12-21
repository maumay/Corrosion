use crate::search::eval;
use crate::search::ordering::MoveQualityEstimator;
use crate::{quiescent, EvalBoard};
use core::cmp;
use myopic_board::{Move, MoveComputeType, Termination};
use std::time::{Duration, Instant};

pub struct SearchContext {
    pub start_time: Instant,
    pub alpha: i32,
    pub beta: i32,
    pub depth_remaining: usize,
    pub precursors: Vec<Move>,
}

impl SearchContext {
    fn next_level(&self, next_alpha: i32, next_beta: i32, mv: &Move) -> SearchContext {
        let mut next_precursors = self.precursors.clone();
        next_precursors.push(mv.clone());
        SearchContext {
            start_time: self.start_time,
            alpha: next_alpha,
            beta: next_beta,
            depth_remaining: self.depth_remaining - 1,
            precursors: next_precursors,
        }
    }
}

/// Represents some object which can determine whether a search should be
/// terminated given certain context about the current state. Implementations
/// are provided for Duration (caps the search based on time elapsed), for
/// usize which represents a maximum search depth and for a pair (Duration, usize)
/// which combines both checks.
pub trait SearchTerminator {
    fn should_terminate(&self, ctx: &SearchContext) -> bool;
}

///
pub struct SearchResponse {
    // The evaluation of the position negamax was called for
    pub eval: i32,
    // The path of optimal play which led to the eval if the
    // depth was greater than zero.
    pub path: Vec<Move>,
}
impl std::ops::Neg for SearchResponse {
    type Output = SearchResponse;

    fn neg(self) -> Self::Output {
        SearchResponse {
            eval: -self.eval,
            path: self.path,
        }
    }
}
impl Default for SearchResponse {
    fn default() -> Self {
        SearchResponse {
            eval: 0,
            path: vec![],
        }
    }
}

pub struct Searcher<'a, T, B, M>
where
    T: SearchTerminator,
    B: EvalBoard,
    M: MoveQualityEstimator<B>,
{
    /// The terminator is responsible for deciding when the
    /// search is complete
    pub terminator: &'a T,
    /// The principle variation is a search optimisation which
    /// comes from "iterative deepening". The idea is that if
    /// we do a search at a lower depth then the optimal path
    /// recovered from that is a good candidate to search first
    /// in a deeper search
    pub principle_variation: &'a Vec<Move>,
    /// Used for performing an initial sort on the moves
    /// generated in each position for optimising the search
    pub move_quality_estimator: M,
    /// Placeholder to satisfy the compiler because of the 'unused'
    /// type parameter for the board
    pub board_type: std::marker::PhantomData<B>,
}

impl<T, B, M> Searcher<'_, T, B, M>
where
    T: SearchTerminator,
    B: EvalBoard,
    M: MoveQualityEstimator<B>,
{
    ///
    pub fn search(&self, root: &mut B, mut ctx: SearchContext) -> Result<SearchResponse, String> {
        if self.terminator.should_terminate(&ctx) {
            Err(format!("Terminated at depth {}", ctx.depth_remaining))
        } else if ctx.depth_remaining == 0 || root.termination_status().is_some() {
            Ok(SearchResponse {
                eval: match root.termination_status() {
                    Some(Termination::Loss) => eval::LOSS_VALUE,
                    Some(Termination::Draw) => eval::DRAW_VALUE,
                    None => quiescent::search(root, -eval::INFTY, eval::INFTY, -1),
                },
                path: vec![],
            })
        } else {
            let (mut result, mut best_path) = (-eval::INFTY, vec![]);
            for (i, evolve) in self
                .compute_moves(root, &ctx.precursors)
                .into_iter()
                .enumerate()
            {
                let discards = root.evolve(&evolve);
                #[allow(unused_assignments)]
                let mut response = SearchResponse::default();
                if i == 0 {
                    // Perform a full search immediately on the first move which
                    // we expect to be the best
                    response =
                        -self.search(root, ctx.next_level(-ctx.beta, -ctx.alpha, &evolve))?;
                } else {
                    // Search with null window under the assumption that the
                    // previous moves are better than this
                    response =
                        -self.search(root, ctx.next_level(-ctx.alpha - 1, -ctx.alpha, &evolve))?;
                    // If there is some move which can raise alpha
                    if ctx.alpha < response.eval && response.eval < ctx.beta {
                        // Then this was actually a better move and so we must
                        // perform a full search
                        response = -self
                            .search(root, ctx.next_level(-ctx.beta, -response.eval, &evolve))?;
                    }
                }
                root.devolve(&evolve, discards);

                if response.eval > result {
                    result = response.eval;
                    best_path = response.path;
                    best_path.push(evolve.clone());
                }

                ctx.alpha = cmp::max(ctx.alpha, result);
                if ctx.alpha >= ctx.beta {
                    return Ok(SearchResponse {
                        eval: ctx.beta,
                        path: vec![],
                    });
                }
            }
            Ok(SearchResponse {
                eval: result,
                path: best_path,
            })
        }
    }

    fn compute_moves(&self, board: &mut B, precursors: &Vec<Move>) -> Vec<Move> {
        let mut moves = board.compute_moves(MoveComputeType::All);
        // Make an initial heuristic sort of the moves before looking
        // for the principle variation
        moves.sort_by_cached_key(|m| -self.move_quality_estimator.estimate(board, m));
        // If we are searching along the principal variation then search the next
        // move on it first (if another move exists)
        if self.principle_variation.starts_with(precursors.as_slice()) {
            match self.principle_variation.get(precursors.len()) {
                None => {}
                Some(suggested_move) => {
                    match moves.iter().position(|m| m == suggested_move) {
                        None => {} // Some sort of debug warning?
                        Some(index) => {
                            moves.remove(index);
                            moves.insert(0, suggested_move.clone());
                        }
                    }
                }
            }
        }
        moves
    }
}

impl SearchTerminator for Duration {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        ctx.start_time.elapsed() > *self
    }
}

impl SearchTerminator for usize {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        ctx.depth_remaining > *self
    }
}

impl SearchTerminator for (Duration, usize) {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        self.0.should_terminate(ctx) || self.1.should_terminate(ctx)
    }
}
