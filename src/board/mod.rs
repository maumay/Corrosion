use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::pieces::Piece;
use crate::board::hashcache::HashCache;
use crate::board::piecetracker::PieceTracker;
use crate::board::castletracker::CastleTracker;
use crate::base::Side;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::pieces::PieceRef;

pub mod hash;
//pub mod tables;// To be removed
pub mod evolve;
pub mod moves;


mod piecetracker;
mod castletracker;
mod hashcache;

#[cfg(test)]
mod testutils;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    hashes: HashCache,
    pieces: PieceTracker,
    castling: CastleTracker,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
}

impl Board {
    fn switch_side(&mut self) {
        self.active = self.active.other();
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache, returning the overwritten value.
    fn update_hash(&mut self) {
        let next_hash = self.pieces.hash()
            ^ self.castling.hash()
            ^ hash::side_feature(self.active)
            ^ self.enpassant.map_or(0u64, |x| hash::enpassant_feature(x));
        self.hashes.push_head(next_hash)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReversalData {
    discarded_rights: CastleZoneSet,
    discarded_piece: Option<PieceRef>,
    discarded_enpassant: Option<Square>,
    discarded_hash: u64,
    discarded_clock: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Standard(PieceRef, Square, Square),
    Enpassant(Square),
    Promotion(Square, Square, PieceRef),
    Castle(CastleZone),
}

impl Move {
    pub fn standard(moving_piece: PieceRef, source: Square, target: Square) -> Move {
        Move::Standard(moving_piece, source, target)
    }

    pub fn enpassant(source: Square) -> Move {
        Move::Enpassant(source)
    }

    pub fn promotion(source: Square, target: Square, piece: PieceRef) -> Move {
        Move::Promotion(source, target, piece)
    }

    pub fn castle(zone: CastleZone) -> Move {
        Move::Castle(zone)
    }
}


