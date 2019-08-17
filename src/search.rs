use std::cmp;

use crate::base::bitboard::BitBoard;
use crate::base::Reflectable;
use crate::board::Board;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::Termination;
use crate::eval;
use crate::eval::EvalBoard;
use crate::itertools;

const Q_DEPTH_CAP: i32 = -6;
const Q_CHECK_CAP: i32 = -3;

pub fn compute_best_moves<B: EvalBoard>(state: &mut B, depth: usize) -> Option<Move> {
    assert!(depth > 0);
    let mut best_move = None;
    let (mut alpha, beta) = (-eval::INFTY, eval::INFTY);
    for evolve in state.compute_moves(MoveComputeType::All) {
        let discards = state.evolve(&evolve);
        let result = -negamax(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        if result > alpha {
            alpha = result;
            best_move = Some(evolve.clone());
        }
        println!("Evaluated {:?} at {:?}", evolve, result);
    }
    best_move
}

fn negamax<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 || state.termination_status().is_some() {
        return match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            //None => state.static_eval(),
            None => quiescent(state, -eval::INFTY, eval::INFTY, -1),
        };
    }
    let mut result = -eval::INFTY;
    for evolve in state.compute_moves(MoveComputeType::All) {
        let discards = state.evolve(&evolve);
        let next_result = -negamax(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return beta;
        }
    }
    return result;
}

fn quiescent<B: EvalBoard>(state: &mut B, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    if depth == Q_DEPTH_CAP || state.termination_status().is_some() {
        return match state.termination_status() {
            Some(Termination::Loss) => eval::LOSS_VALUE,
            Some(Termination::Draw) => eval::DRAW_VALUE,
            None => state.static_eval(),
        };
    }
    // If we aren't in check then we can use the static eval as the initial
    // result under the sound assumption that there exists a move we can
    // make in the position which will improve our score. We cannot make this
    // assumption if we are in check because we will search all the moves.
    let mut result = if state.in_check() {
        -eval::INFTY
    } else {
        state.static_eval()
    };

    // Break immediately if the stand pat is greater than beta.
    if result >= beta {
        return beta;
    }
    if alpha < result {
        alpha = result;
    }

    for evolve in compute_quiescent_moves(state, depth) {
        let discards = state.evolve(&evolve);
        let next_result = -quiescent(state, -beta, -alpha, depth - 1);
        state.devolve(&evolve, discards);
        result = cmp::max(result, next_result);
        alpha = cmp::max(alpha, result);
        if alpha > beta {
            return beta;
        }
    }
    return result;
}

fn compute_quiescent_moves<B: Board>(state: &mut B, depth: i32) -> Vec<Move> {
    let mut moves = if depth > Q_CHECK_CAP {
        state.compute_moves(MoveComputeType::AttacksChecks)
    } else {
        state.compute_moves(MoveComputeType::Attacks)
    };
    let enemies = state.side(state.active().reflect());
    let attack_filter = |mv: &Move| is_attack(mv, enemies);
    let split_index = itertools::partition(&mut moves, attack_filter);
    // Score attacks using see and filter bad exchanges before sorting and
    // recombining.
    let mut attacks: Vec<_> = moves
        .iter()
        .take(split_index)
        .map(|mv| (mv, score_attack(state, mv)))
        .filter(|(_, score)| *score > 0)
        .collect();
    attacks.sort_by_key(|(_, score)| -*score);

    moves
        .iter()
        .cloned()
        .skip(split_index)
        .chain(attacks.into_iter().map(|(mv, _)| mv.clone()))
        .collect()
}

fn score_attack<B: Board>(state: &mut B, attack: &Move) -> i32 {
    match attack {
        &Move::Enpassant(_) => 10000,
        &Move::Promotion(_, _, _) => 20000,
        &Move::Standard(_, source, target) => eval::exchange_value(state, source, target),
        _ => panic!(),
    }
}

fn is_attack(query: &Move, enemies: BitBoard) -> bool {
    match query {
        &Move::Enpassant(_) => true,
        &Move::Castle(_) => false,
        &Move::Promotion(_, target, _) => enemies.contains(target),
        &Move::Standard(_, _, target) => enemies.contains(target),
    }
}
