use crate::base::dir::Dir;
use crate::base::dir::N;
use crate::base::dir::S;
use crate::base::bitboard::BitBoard;

pub mod bitboard;
pub mod square;
pub mod dir;
pub mod castlezone;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::White => Side::Black,
            _ => Side::Black
        }
    }

    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::White => N,
            Side::Black => S,
        }
    }

    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[1],
            Side::Black => BitBoard::RANKS[6],
        }
    }

    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[3],
            Side::Black => BitBoard::RANKS[4],
        }
    }

}

