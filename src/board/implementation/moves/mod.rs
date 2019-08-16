use crate::base::bitboard::BitBoard;
use crate::base::castlezone::CastleZone;
use crate::base::Reflectable;
use crate::base::Side;
use crate::base::square::Square;
use crate::board::Board;
use crate::board::implementation::BoardImpl;
use crate::board::implementation::cache::constraints::MoveConstraints;
use crate::board::Move;
use crate::board::MoveComputeType;
use crate::pieces::Piece;

#[cfg(test)]
mod test;

mod enpassant_source;

const FILES: [BitBoard; 8] = BitBoard::FILES;


impl BoardImpl {
    pub(in crate::board::implementation) fn compute_moves_impl(
        &mut self,
        computation_type: MoveComputeType,
    ) -> Vec<Move> {
        let constraints = self.constraints(computation_type);
        let pawn_moves = self.compute_pawn_moves(&constraints);
        let nbrqk_moves = self.compute_nbrqk_moves(&constraints);
        let castle_moves = match computation_type {
            MoveComputeType::All => self.compute_castle_moves(&constraints),
            _ => Vec::with_capacity(0),
        };
        pawn_moves
            .into_iter()
            .chain(nbrqk_moves.into_iter())
            .chain(castle_moves.into_iter())
            .collect()
    }

    fn compute_nbrqk_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(40);
        let (whites, blacks) = self.sides();
        let unchecked_moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // Add standard moves for pieces which aren't pawns or king
        for piece in Piece::on_side(self.active).skip(1) {
            for location in self.pieces.locs_impl(piece) {
                let moves = unchecked_moves(piece, location) & constraints.get(location);
                dest.extend(Move::standards(piece, location, moves));
            }
        }
        dest
    }

    fn compute_pawn_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let (active_pawn, (whites, blacks)) = (Piece::pawn(self.active), self.sides());
        let compute_moves = |loc: Square| active_pawn.moves(loc, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }
        for location in enpassant {
            if constraints.get(location).contains(self.enpassant.unwrap()) {
                dest.push(Move::Enpassant(location));
            }
        }
        for location in promotion {
            let targets = compute_moves(location) & constraints.get(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_source = self.enpassant.map_or(BitBoard::EMPTY, |sq| {
            enpassant_source::squares(self.active, sq)
        });
        let promotion_rank = self.active.pawn_last_rank();
        let pawn_locs = self.locs(Piece::pawn(self.active));
        (
            pawn_locs - enpassant_source - promotion_rank,
            pawn_locs & enpassant_source,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_castle_moves(&self, constraints: &MoveConstraints) -> Vec<Move> {
        let king_constraint = constraints.get(self.king(self.active));
        let (whites, blacks) = self.sides();
        let p1 = |z: CastleZone| king_constraint.subsumes(z.uncontrolled_requirement());
        let p2 = |z: CastleZone| !(whites | blacks).intersects(z.unoccupied_requirement());
        self.castling
            .rights()
            .iter()
            .filter(|&z| p1(z) && p2(z))
            .map(Move::Castle)
            .collect()
    }
}
