use std::io::repeat;

use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::castlezone::CastleZoneSet;
use crate::base::dir::N;
use crate::base::dir::S;
use crate::base::square::Square::A1;
use crate::base::square::Square::A8;
use crate::base::square::Square::E1;
use crate::base::square::Square::E8;
use crate::base::square::Square::H1;
use crate::base::square::Square::H8;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::hash;
use crate::board::Board;
use crate::board::Move;
use crate::board::Move::*;
use crate::board::ReversalData;
use crate::{pieces, pieces::Piece};
use crate::base::Reflectable;

#[cfg(test)]
mod test;

type RD = ReversalData;

/// Implementation of board evolution/devolution via some given Move
/// instance which is assumed to be legal for this board.
impl Board {
    /// Public API for evolving a board. All that is required is a reference to
    /// a move which is assumed to be legal. The information required to reverse
    /// this same move is returned and the board is mutated to the next state.
    pub fn evolve(&mut self, action: &Move) -> RD {
        match action {
            Standard(piece, source, target) => self.evolve_s(*piece, *source, *target),
            Castle(zone) => self.evolve_c(*zone),
            Enpassant(source) => self.evolve_e(*source),
            Promotion(source, target, piece) => self.evolve_p(*source, *target, *piece),
        }
    }

    /// Public API for devolving a move, the information lost at evolve time is
    /// required as an input here to recover the lost state exactly.
    pub fn devolve(&mut self, action: &Move, discards: RD) {
        match action {
            Standard(piece, source, target) => self.devolve_s(*piece, *source, *target, discards),
            Castle(zone) => self.devolve_c(*zone, discards),
            Enpassant(source) => self.devolve_e(*source, discards),
            Promotion(source, target, piece) => self.devolve_p(*source, *target, *piece, discards),
        }
    }

    fn evolve_s(&mut self, piece: Piece, source: Square, target: Square) -> RD {
        let discarded_piece = self.pieces.erase_square(target);
        let discarded_rights = self.castling.remove_rights(source | target);
        let rev_data = self.create_rev_data(discarded_piece, discarded_rights);
        self.pieces.toggle_piece(piece, &[source, target]);
        self.clock = if discarded_piece.is_some() || piece.is_pawn() {
            0
        } else {
            self.clock + 1
        };
        self.enpassant = Board::compute_enpassant(source, target, piece);
        self.switch_side_and_update_hash();
        rev_data
    }

    fn switch_side_and_update_hash(&mut self) {
        self.switch_side();
        self.update_hash();
    }

    fn create_rev_data(
        &self,
        discarded_piece: Option<Piece>,
        discarded_rights: CastleZoneSet,
    ) -> RD {
        ReversalData {
            discarded_piece,
            discarded_rights,
            discarded_enpassant: self.enpassant,
            discarded_hash: self.hashes.tail(),
            discarded_clock: self.clock,
        }
    }

    fn devolve_s(&mut self, piece: Piece, source: Square, target: Square, discards: RD) {
        self.switch_side();
        self.pieces.toggle_piece(piece, &[target, source]);
        match discards.discarded_piece {
            Some(discarded) => self.pieces.toggle_piece(discarded, &[target]),
            _ => (),
        };
        self.replace_metadata(discards);
    }

    fn evolve_c(&mut self, zone: CastleZone) -> RD {
        let discarded_rights = self.castling.set_status(self.active, zone);
        let rev_data = self.create_rev_data(None, discarded_rights);
        self.toggle_castle_pieces(zone);
        self.enpassant = None;
        self.clock += 1;
        self.switch_side_and_update_hash();
        rev_data
    }

    fn devolve_c(&mut self, zone: CastleZone, discards: RD) {
        self.switch_side();
        self.toggle_castle_pieces(zone);
        self.castling.clear_status(self.active);
        self.replace_metadata(discards);
    }

    fn toggle_castle_pieces(&mut self, zone: CastleZone) {
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
    }

    fn evolve_e(&mut self, source: Square) -> RD {
        let discarded_piece = pieces::pawn(self.active.reflect());
        let rev_data = self.create_rev_data(Some(discarded_piece), CastleZoneSet::NONE);
        self.toggle_enpassant_pieces(source, self.enpassant.unwrap());
        self.enpassant = None;
        self.clock = 0;
        self.switch_side_and_update_hash();
        rev_data
    }

    fn devolve_e(&mut self, source: Square, discards: RD) {
        self.switch_side();
        self.toggle_enpassant_pieces(source, discards.discarded_enpassant.unwrap());
        self.replace_metadata(discards);
    }

    fn toggle_enpassant_pieces(&mut self, source: Square, enpassant: Square) {
        let active = self.active;
        let (active_pawn, passive_pawn) = match active {
            Side::White => (Piece::WP, Piece::BP),
            _ => (Piece::BP, Piece::WP),
        };
        let removal_square = enpassant.next(active.pawn_dir().reflect()).unwrap();
        self.pieces.toggle_piece(active_pawn, &[source, enpassant]);
        self.pieces.toggle_piece(passive_pawn, &[removal_square]);
    }

    fn evolve_p(&mut self, source: Square, target: Square, promotion_result: Piece) -> RD {
        let discarded_piece = self.pieces.erase_square(target);
        let rev_data = self.create_rev_data(discarded_piece, CastleZoneSet::NONE);
        let moved_pawn = pieces::pawn(self.active);
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(promotion_result, &[target]);
        self.enpassant = None;
        self.clock = 0;
        self.switch_side_and_update_hash();
        rev_data
    }

    fn devolve_p(&mut self, source: Square, target: Square, piece: Piece, discards: RD) {
        self.switch_side();
        let moved_pawn = pieces::pawn(self.active);
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(piece, &[target]);
        match discards.discarded_piece {
            Some(p) => self.pieces.toggle_piece(p, &[target]),
            _ => (),
        };
        self.replace_metadata(discards);
    }

    fn replace_metadata(&mut self, discards: RD) {
        self.castling.add_rights(discards.discarded_rights);
        self.hashes.pop_head(discards.discarded_hash);
        self.enpassant = discards.discarded_enpassant;
        self.clock = discards.discarded_clock;
    }

    /// Determines the enpassant square for the next board state given a
    /// piece which has just moved from the source to the target.
    fn compute_enpassant(source: Square, target: Square, piece: Piece) -> Option<Square> {
        if piece.is_pawn() {
            let side = piece.side();
            if side.pawn_first_rank().contains(source) && side.pawn_third_rank().contains(target) {
                source.next(side.pawn_dir())
            } else {
                None
            }
        } else {
            None
        }
    }
}
