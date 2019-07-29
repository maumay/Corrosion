use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::Board;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::board::ReversalData;
use crate::pieces::Piece;

mod see;
mod tables;
mod values;

/// Extension of the Board trait which adds a static evaluation function.
///
pub trait EvalBoard: Board {
    /// The static evaluation function assigns a score to this exact
    /// position at the point of time it is called. It does not take
    /// into account potential captures/recaptures etc. It must follow
    /// the rule that 'a higher score is best for the active side'. That
    /// is if it is white to move next then a high positive score indicates
    /// a favorable position for white and if it is black to move a high
    /// positive score indicates a favorable position for black.
    fn static_eval(&self) -> i32;
}

pub struct SimpleEvalBoard<B: Board> {
    mid_eval: i32,
    end_eval: i32,
    phase: usize,
    board: B,
}

const PHASE_VALUES: [usize; 5] = [0, 1, 1, 2, 4];
const TOTAL_PHASE: usize = 16 * PHASE_VALUES[0]
    + 4 * (PHASE_VALUES[1] + PHASE_VALUES[2] + PHASE_VALUES[3])
    + 2 * PHASE_VALUES[4];

impl<B: Board> SimpleEvalBoard<B> {
    fn remove(&mut self, piece: Piece, location: Square) {
        self.mid_eval -= tables::midgame(piece, location) + values::midgame(piece);
        self.end_eval -= tables::endgame(piece, location) + values::endgame(piece);
        self.phase += PHASE_VALUES[piece as usize];
    }

    fn add(&mut self, piece: Piece, location: Square) {
        self.mid_eval += tables::midgame(piece, location) + values::midgame(piece);
        self.end_eval += tables::endgame(piece, location) + values::endgame(piece);
        self.phase -= PHASE_VALUES[piece as usize];
    }
}

impl<B: Board> Board for SimpleEvalBoard<B> {
    fn evolve(&mut self, action: &Move) -> ReversalData {
        match action {
            &Move::Standard(moving, src, target) => {
                self.remove(moving, src);
                self.add(moving, target);
                self.piece_at(target)
                    .map(|taken| self.remove(taken, target));
            }
            &Move::Promotion(source, target, promoting) => {
                let pawn = Piece::pawn(self.active());
                self.remove(pawn, source);
                self.add(promoting, target);
                self.piece_at(target)
                    .map(|taken| self.remove(taken, target));
            }
            &Move::Enpassant(source) => {
                let active_pawn = Piece::pawn(self.active());
                let passive_pawn = active_pawn.reflect();
                let enpassant = self.enpassant_square().unwrap();
                let removal_square = match self.active() {
                    Side::White => enpassant >> 8,
                    Side::Black => enpassant << 8,
                };
                self.remove(active_pawn, source);
                self.add(active_pawn, enpassant);
                self.remove(passive_pawn, removal_square);
            }
            &Move::Castle(zone) => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
                self.remove(rook, r_src);
                self.add(rook, r_target);
                self.remove(king, k_src);
                self.add(king, k_target);
            }
        };
        self.board.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: ReversalData) {
        match action {
            &Move::Standard(moving, src, target) => {
                self.remove(moving, target);
                self.add(moving, src);
                discards
                    .discarded_piece
                    .map(|taken| self.add(taken, target));
            }
            &Move::Promotion(source, target, promoting) => {
                let pawn = Piece::pawn(self.active());
                self.add(pawn, source);
                self.remove(promoting, target);
                discards
                    .discarded_piece
                    .map(|taken| self.add(taken, target));
            }
            &Move::Enpassant(source) => {
                let active_pawn = Piece::pawn(self.active());
                let passive_pawn = active_pawn.reflect();
                let enpassant = self.enpassant_square().unwrap();
                let removal_square = match self.active() {
                    Side::White => enpassant >> 8,
                    Side::Black => enpassant << 8,
                };
                self.add(active_pawn, source);
                self.remove(active_pawn, enpassant);
                self.add(passive_pawn, removal_square);
            }
            &Move::Castle(zone) => {
                let (rook, r_src, r_target) = zone.rook_data();
                let (king, k_src, k_target) = zone.king_data();
                self.add(rook, r_src);
                self.remove(rook, r_target);
                self.add(king, k_src);
                self.remove(king, k_target);
            }
        };
        self.board.devolve(action, discards)
    }

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.board.compute_moves(computation_type)
    }

    fn hash(&self) -> u64 {
        self.board.hash()
    }

    fn active(&self) -> Side {
        self.board.active()
    }

    fn enpassant_square(&self) -> Option<Square> {
        self.board.enpassant_square()
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.board.castle_status(side)
    }

    fn piece_locations(&self, piece: Piece) -> BitBoard {
        self.board.piece_locations(piece)
    }

    fn king_location(&self, side: Side) -> Square {
        self.board.king_location(side)
    }

    fn whites_blacks(&self) -> (BitBoard, BitBoard) {
        self.board.whites_blacks()
    }

    fn piece_at(&self, location: Square) -> Option<Piece> {
        self.board.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.board.half_move_clock()
    }

    fn game_counter(&self) -> usize {
        self.board.game_counter()
    }
}

impl<B: Board> EvalBoard for SimpleEvalBoard<B> {
    fn static_eval(&self) -> i32 {
        let phase: i32 = ((self.phase * 256 + TOTAL_PHASE / 2) / TOTAL_PHASE) as i32;
        let (mid, end) = (self.mid_eval, self.end_eval);
        let eval = ((mid * (256 - phase)) + end * phase) / 256;
        match self.active() {
            Side::White => eval,
            Side::Black => -eval,
        }
    }
}
