use crate::base::bitboard::BitBoard;
use crate::base::dir::Dir;
use crate::base::dir::N;
use crate::base::dir::S;
use std::cell::Ref;

pub mod bitboard;
pub mod castlezone;
pub mod dir;
pub mod square;

/// Represents the two different teams in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Side {
    White,
    Black,
}

impl Side {
    /// Get the vertical direction in which a pawn on this side moves
    /// (north or south).
    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::White => N,
            Side::Black => S,
        }
    }

    /// Get the rank on which a pawn on this side starts the game.
    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[1],
            Side::Black => BitBoard::RANKS[6],
        }
    }

    /// Get the rank to which a pawn on this side moves to following
    /// it's special two rank first move.
    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[3],
            Side::Black => BitBoard::RANKS[4],
        }
    }

    /// Get the rank a pawn on this side must be on for it to be able
    /// to promote on it's next move.
    pub fn pawn_last_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[6],
            Side::Black => BitBoard::RANKS[1],
        }
    }
}


/// Chess is a symmetric game and this trait represents a component of
/// the game which can be reflected to it's symmetric opposite component.
pub trait Reflectable {
    fn reflect(&self) -> Self;
}

impl Reflectable for Side {
    fn reflect(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

impl<T: Reflectable> Reflectable for Vec<T> {
    fn reflect(&self) -> Self {
        self.into_iter().map(|t| t.reflect()).collect()
    }
}

impl<T: Reflectable> Reflectable for Option<T> {
    fn reflect(&self) -> Self {
        match self {
            Some(t) => Some(t.reflect()),
            _ => None,
        }
    }
}

