use crate::board::implementation::MutBoardImpl;
use crate::board::Discards;
use crate::board::Move;
use crate::board::Move::*;
use myopic_core::castlezone::CastleZone;
use myopic_core::castlezone::CastleZoneSet;
use myopic_core::pieces::Piece;
use myopic_core::reflectable::Reflectable;
use myopic_core::{Side, Square};

#[cfg(test)]
mod test;

type D = Discards;

/// Implementation of board evolution/devolution via some given Move
/// instance which is assumed to be legal for this board.
impl MutBoardImpl {
    /// Public API for evolving a board. All that is required is a reference to
    /// a move which is assumed to be legal. The information required to reverse
    /// this same move is returned and the board is mutated to the next state.
    pub fn evolve(&mut self, action: &Move) -> D {
        match action {
            &Standard(piece, source, target) => self.evolve_s(piece, source, target),
            &Castle(zone) => self.evolve_c(zone),
            &Enpassant(source, _) => self.evolve_e(source),
            &Promotion(source, target, piece) => self.evolve_p(source, target, piece),
        }
    }

    /// Public API for devolving a move, the information lost at evolve time is
    /// required as an input here to recover the lost state exactly.
    pub fn devolve(&mut self, action: &Move, discards: D) {
        match action {
            &Standard(piece, source, target) => self.devolve_s(piece, source, target, discards),
            &Castle(zone) => self.devolve_c(zone, discards),
            &Enpassant(source, _) => self.devolve_e(source, discards),
            &Promotion(source, target, piece) => self.devolve_p(source, target, piece, discards),
        }
    }

    fn evolve_s(&mut self, piece: Piece, source: Square, target: Square) -> D {
        let discarded_piece = self.pieces.erase_square(target);
        let discarded_rights = self.castling.remove_rights(source | target);
        let rev_data = self.create_rev_data(discarded_piece, discarded_rights);
        self.pieces.toggle_piece(piece, &[source, target]);
        self.clock = if discarded_piece.is_some() || piece.is_pawn() { 0 } else { self.clock + 1 };
        self.enpassant = MutBoardImpl::compute_enpassant(source, target, piece);
        self.switch_side_update_hash_clear_cache();
        rev_data
    }

    fn switch_side_update_hash_clear_cache(&mut self) {
        self.switch_side();
        self.update_hash();
        self.clear_cache();
    }

    fn create_rev_data(&self, piece: Option<Piece>, rights: CastleZoneSet) -> D {
        Discards {
            piece: piece,
            rights: rights,
            enpassant: self.enpassant,
            hash: self.history.tail(),
            half_move_clock: self.clock,
        }
    }

    fn devolve_s(&mut self, piece: Piece, source: Square, target: Square, discards: D) {
        self.switch_side();
        self.pieces.toggle_piece(piece, &[target, source]);
        match discards.piece {
            Some(discarded) => self.pieces.toggle_piece(discarded, &[target]),
            _ => (),
        };
        self.replace_metadata_erase_cache(discards);
    }

    fn evolve_c(&mut self, zone: CastleZone) -> D {
        let discarded_rights = self.castling.set_status(self.active, zone);
        let rev_data = self.create_rev_data(None, discarded_rights);
        self.toggle_castle_pieces(zone);
        self.enpassant = None;
        self.clock += 1;
        self.switch_side_update_hash_clear_cache();
        rev_data
    }

    fn devolve_c(&mut self, zone: CastleZone, discards: D) {
        self.switch_side();
        self.toggle_castle_pieces(zone);
        self.castling.clear_status(self.active);
        self.replace_metadata_erase_cache(discards);
    }

    fn toggle_castle_pieces(&mut self, zone: CastleZone) {
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.toggle_piece(rook, &[r_source, r_target]);
        self.pieces.toggle_piece(king, &[k_source, k_target]);
    }

    fn evolve_e(&mut self, source: Square) -> D {
        let discarded_piece = Piece::pawn(self.active.reflect());
        let rev_data = self.create_rev_data(Some(discarded_piece), CastleZoneSet::NONE);
        self.toggle_enpassant_pieces(source, self.enpassant.unwrap());
        self.enpassant = None;
        self.clock = 0;
        self.switch_side_update_hash_clear_cache();
        rev_data
    }

    fn devolve_e(&mut self, source: Square, discards: D) {
        self.switch_side();
        self.toggle_enpassant_pieces(source, discards.enpassant.unwrap());
        self.replace_metadata_erase_cache(discards);
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

    fn evolve_p(&mut self, source: Square, target: Square, promotion_result: Piece) -> D {
        let discarded_piece = self.pieces.erase_square(target);
        let rev_data = self.create_rev_data(discarded_piece, CastleZoneSet::NONE);
        let moved_pawn = Piece::pawn(self.active);
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(promotion_result, &[target]);
        self.enpassant = None;
        self.clock = 0;
        self.switch_side_update_hash_clear_cache();
        rev_data
    }

    fn devolve_p(&mut self, source: Square, target: Square, piece: Piece, discards: D) {
        self.switch_side();
        let moved_pawn = Piece::pawn(self.active);
        self.pieces.toggle_piece(moved_pawn, &[source]);
        self.pieces.toggle_piece(piece, &[target]);
        match discards.piece {
            Some(p) => self.pieces.toggle_piece(p, &[target]),
            _ => (),
        };
        self.replace_metadata_erase_cache(discards);
    }

    fn replace_metadata_erase_cache(&mut self, discards: D) {
        self.castling.add_rights(discards.rights);
        self.history.pop_head(discards.hash);
        self.enpassant = discards.enpassant;
        self.clock = discards.half_move_clock;
        self.clear_cache();
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
