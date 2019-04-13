use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;

pub mod hash;
pub mod tables;

#[derive(Debug, Clone, PartialEq)]
struct PieceTracker {
    boards: Vec<BitBoard>,
    hash: u64,
    //    mid_eval: i32, lets completely detach evaluation
    //    end_eval: i32,
}

impl PieceTracker {
    pub fn contains(&self, piece: &dyn Piece, location: Square) -> bool {
        self.locations(piece).contains(location)
    }

    pub fn locations(&self, piece: &dyn Piece) -> BitBoard {
        self.boards[piece.index()]
    }

    pub fn whites(&self) -> BitBoard {
        (&self.boards).into_iter().take(6).map(|x| *x).collect()
    }

    pub fn blacks(&self) -> BitBoard {
        (&self.boards).into_iter().skip(6).map(|x| *x).collect()
    }

    pub fn add(&mut self, piece: &dyn Piece, location: Square) {
        debug_assert!(!self.boards[piece.index()].contains(location));
        self.perform_xor(piece, location);
    }

    pub fn remove(&mut self, piece: &dyn Piece, location: Square) {
        debug_assert!(self.boards[piece.index()].contains(location));
        self.perform_xor(piece, location);
    }

    fn perform_xor(&mut self, piece: &dyn Piece, location: Square) {
        self.boards[piece.index()] ^= location;
        self.hash ^= hash::piece_feature(piece, location);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter;
    use crate::base::square::constants::C5;
    use crate::pieces::BlackKnight;
    use crate::base::square::constants::E3;
    use crate::pieces::WhitePawn;

    /// We test with a simple setup of one white pawn at E3 and one
    /// black knight at C5.
    #[test]
    fn test() {
        let mut tracker = init_pawn_and_knight();
        tracker.remove(&WhitePawn, E3);
        assert_eq!(init_knight(), tracker);
        tracker.add(&WhitePawn, E3);
        assert_eq!(init_pawn_and_knight(), tracker);
        tracker.remove(&BlackKnight, C5);
        assert_eq!(init_pawn(), tracker);
        tracker.remove(&WhitePawn, E3);
        assert_eq!(init_empty(), tracker);
        tracker.add(&WhitePawn, E3);
        tracker.add(&BlackKnight, C5);
        assert_eq!(init_pawn_and_knight(), tracker);
    }

    fn init_pawn_and_knight() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[0] = E3.lift();
        boards[7] = C5.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(&WhitePawn, E3) ^ hash::piece_feature(&BlackKnight, C5)
        }
    }

    fn init_pawn() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[0] = E3.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(&WhitePawn, E3),
        }
    }

    fn init_knight() -> PieceTracker {
        let mut boards = init_empty_boards();
        boards[7] = C5.lift();
        PieceTracker {
            boards,
            hash: hash::piece_feature(&BlackKnight, C5),
        }
    }

    fn init_empty() -> PieceTracker {
        PieceTracker {
            boards: init_empty_boards(),
            hash: 0u64,
        }
    }

    fn init_empty_boards() -> Vec<BitBoard> {
        iter::repeat(BitBoard::EMPTY).take(12).collect()
    }
}
