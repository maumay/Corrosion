use std::cmp::max;

use crate::implementation::cache::CalculationCache;
use crate::implementation::castling::Castling;
use crate::implementation::history::History;
use crate::implementation::positions::Positions;
use crate::parse::patterns;
use crate::{Discards, FenComponent};
use crate::Move;
use crate::MoveComputeType;
use crate::MutBoard;
use crate::Termination;
use myopic_core::bitboard::BitBoard;
use myopic_core::castlezone::{CastleZone, CastleZoneSet};
use myopic_core::pieces::Piece;
use myopic_core::reflectable::Reflectable;
use myopic_core::{Side, Square};

mod cache;
mod castling;
mod evolve;
mod fen;
mod history;
mod moves;
mod positions;
#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct MutBoardImpl {
    history: History,
    pieces: Positions,
    castling: Castling,
    active: Side,
    enpassant: Option<Square>,
    clock: usize,
    cache: CalculationCache,
}

impl MutBoardImpl {
    pub(super) fn from_fen(fen: String) -> Result<MutBoardImpl, String> {
        if patterns::fen().is_match(&fen) {
            let space_split: Vec<_> = patterns::space().split(&fen).map(|s| s.to_owned()).collect();
            let pieces = positions_from_fen(&space_split[0])?;
            let active = side_from_fen(&space_split[1])?;
            let castling = rights_from_fen(&space_split[2])?;
            let enpassant = enpassant_from_fen(&space_split[3]);
            let (clock, history) = clock_history_from_fen(&fen, active)?;
            let hash = hash(&pieces, &castling, active, enpassant);
            Ok(MutBoardImpl {
                pieces,
                active,
                enpassant,
                castling,
                clock,
                history: History::new(hash, history),
                cache: CalculationCache::empty(),
            })
        } else {
            Err(fen)
        }
    }

    fn switch_side(&mut self) {
        self.active = self.active.reflect();
    }

    /// Combines the various components of the hash together and pushes the
    /// result onto the head of the cache.
    fn update_hash(&mut self) {
        self.history.push_head(hash(&self.pieces, &self.castling, self.active, self.enpassant))
    }
}

fn side_from_fen(fen: &String) -> Result<Side, String> {
    match patterns::fen_side().find(fen).map(|m| m.as_str()).ok_or(fen.clone())? {
        "w" => Ok(Side::White),
        "b" => Ok(Side::Black),
        _ => Err(fen.clone()),
    }
}

fn enpassant_from_fen(fen: &String) -> Option<Square> {
    patterns::fen_enpassant().find(fen).and_then(|m| Square::from_string(m.as_str()).ok())
}

fn positions_from_fen(fen: &String) -> Result<Positions, String> {
    let positions = patterns::fen_positions().find(&fen).map(|m| m.as_str()).ok_or(fen.clone())?;
    Positions::from_fen(String::from(positions))
}

fn rights_from_fen(fen: &String) -> Result<Castling, String> {
    let rights = patterns::fen_rights().find(&fen).map(|m| m.as_str()).ok_or(fen.clone())?;
    Castling::from_fen(String::from(rights))
}

fn clock_history_from_fen(fen: &String, active: Side) -> Result<(usize, usize), String> {
    let ints: Vec<_> =
        patterns::int().find_iter(fen).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    if ints.len() < 2 {
        Err(fen.clone())
    } else {
        let n = ints.len();
        let (clock, moves_played) = (ints[n - 2], ints[n - 1]);
        let history = 2 * (max(moves_played, 1) - 1) + (active as usize);
        Ok((clock, history))
    }
}

fn hash(pt: &Positions, ct: &Castling, active: Side, ep: Option<Square>) -> u64 {
    pt.hash()
        ^ ct.hash()
        ^ myopic_core::hash::side(active)
        ^ ep.map_or(0u64, |x| myopic_core::hash::enpassant(x))
}

impl Move {
    fn standards(moving: Piece, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().map(move |target| Move::Standard(moving, src, target))
    }

    fn promotions(side: Side, src: Square, targets: BitBoard) -> impl Iterator<Item = Move> {
        targets.into_iter().flat_map(move |target| {
            Move::promotion_targets(side)
                .iter()
                .map(move |&piece| Move::Promotion(src, target, piece))
        })
    }

    fn promotion_targets<'a>(side: Side) -> &'a [Piece; 4] {
        match side {
            Side::White => &[Piece::WQ, Piece::WR, Piece::WB, Piece::WN],
            Side::Black => &[Piece::BQ, Piece::BR, Piece::BB, Piece::BN],
        }
    }
}

#[cfg(test)]
mod fen_test {
    use crate::implementation::test::TestBoard;
    use crate::MutBoardImpl;
    use myopic_core::bitboard::constants::*;
    use myopic_core::castlezone::CastleZone;
    use myopic_core::castlezone::CastleZoneSet;
    use myopic_core::{Side, Square};

    fn test(expected: TestBoard, fen_string: String) {
        assert_eq!(MutBoardImpl::from(expected), MutBoardImpl::from_fen(fen_string).unwrap())
    }

    #[test]
    fn fen_to_board_case_1() {
        let fen = "r1br2k1/1pq1npb1/p2pp1pp/8/2PNP3/P1N5/1P1QBPPP/3R1RK1 w - - 3 19";
        let board = TestBoard {
            whites: vec![A3 | B2 | C4 | E4 | F2 | G2 | H2, C3 | D4, E2, D1 | F1, D2, G1],
            blacks: vec![A6 | B7 | D6 | E6 | F7 | G6 | H6, E7, C8 | G7, A8 | D8, C7, G8],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 3,
            active: Side::White,
            enpassant: None,
            history_count: 36,
        };
        test(board, String::from(fen));
    }

    #[test]
    fn fen_to_board_case_2() {
        let fen = "rnb2rk1/ppp2ppp/4pq2/8/2PP4/5N2/PP3PPP/R2QKB1R w KQ - 2 9";
        let board = TestBoard {
            whites: vec![A2 | B2 | C4 | D4 | F2 | G2 | H2, F3, F1, A1 | H1, D1, E1],
            blacks: vec![A7 | B7 | C7 | E6 | F7 | G7 | H7, B8, C8, A8 | F8, F6, G8],
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::White,
            enpassant: None,
            history_count: 16,
        };
        test(board, String::from(fen));
    }

    #[test]
    fn fen_to_board_case_3() {
        let fen = "r1bqkbnr/ppp1pppp/n7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let board = TestBoard {
            whites: vec![A2 | B2 | C2 | D2 | E5 | F2 | G2 | H2, B1 | G1, C1 | F1, A1 | H1, D1, E1],
            blacks: vec![A7 | B7 | C7 | D5 | E7 | F7 | G7 | H7, A6 | G8, C8 | F8, A8 | H8, D8, E8],
            castle_rights: CastleZoneSet::ALL,
            white_status: None,
            black_status: None,
            clock: 0,
            active: Side::White,
            enpassant: Some(Square::D6),
            history_count: 4,
        };
        test(board, String::from(fen));
    }

    #[test]
    fn fen_to_board_case_4() {
        let fen = "r6k/p5pp/p1b2qnN/8/3Q4/2P1B3/PP4PP/R5K1 b - - 2 21";
        let board = TestBoard {
            whites: vec![A2 | B2 | C3 | G2 | H2, H6, E3, A1, D4, G1],
            blacks: vec![A7 | A6 | G7 | H7, G6, C6, A8, F6, H8],
            castle_rights: CastleZoneSet::NONE,
            white_status: Some(CastleZone::WK),
            black_status: Some(CastleZone::BK),
            clock: 2,
            active: Side::Black,
            enpassant: None,
            history_count: 41,
        };
        test(board, String::from(fen));
    }
}

// Trait implementations
impl Reflectable for MutBoardImpl {
    fn reflect(&self) -> Self {
        let pieces = self.pieces.reflect();
        let castling = self.castling.reflect();
        let active = self.active.reflect();
        let enpassant = self.enpassant.reflect();
        let history_count = self.history_count();
        let hash = hash(&pieces, &castling, active, enpassant);
        MutBoardImpl {
            history: History::new(hash, history_count),
            clock: self.clock,
            pieces,
            castling,
            active,
            enpassant,
            cache: CalculationCache::empty(),
        }
    }
}

impl Reflectable for Move {
    fn reflect(&self) -> Self {
        match self {
            Move::Castle(zone) => Move::Castle(zone.reflect()),
            Move::Enpassant(s, t) => Move::Enpassant(s.reflect(), t.reflect()),
            Move::Standard(p, s, t) => Move::Standard(p.reflect(), s.reflect(), t.reflect()),
            Move::Promotion(s, t, p) => Move::Promotion(s.reflect(), t.reflect(), p.reflect()),
        }
    }
}

impl PartialEq<MutBoardImpl> for MutBoardImpl {
    fn eq(&self, other: &MutBoardImpl) -> bool {
        self.pieces == other.pieces
            && self.castling.rights() == other.castling.rights()
            && self.enpassant == other.enpassant
            && self.active == other.active
            && self.half_move_clock() == other.half_move_clock()
    }
}

impl MutBoard for MutBoardImpl {
    fn evolve(&mut self, action: &Move) -> Discards {
        self.evolve(action)
    }

    fn devolve(&mut self, action: &Move, discards: Discards) {
        self.devolve(action, discards)
    }

    fn compute_moves(&mut self, computation_type: MoveComputeType) -> Vec<Move> {
        self.compute_moves_impl(computation_type)
    }

    fn termination_status(&mut self) -> Option<Termination> {
        self.termination_status_impl()
    }

    fn in_check(&mut self) -> bool {
        self.passive_control_impl().contains(self.king(self.active))
    }

    fn side(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.pieces.whites(),
            Side::Black => self.pieces.blacks(),
        }
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        (self.pieces.side_locations(Side::White), self.pieces.side_locations(Side::Black))
    }

    fn hash(&self) -> u64 {
        self.history.head()
    }

    fn active(&self) -> Side {
        self.active
    }

    fn enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    fn castle_status(&self, side: Side) -> Option<CastleZone> {
        self.castling.status(side)
    }

    fn locs(&self, piece: Piece) -> BitBoard {
        self.pieces.locs_impl(piece)
    }

    fn king(&self, side: Side) -> Square {
        self.pieces.king_location(side)
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.pieces.piece_at(location)
    }

    fn half_move_clock(&self) -> usize {
        self.clock
    }

    fn history_count(&self) -> usize {
        self.history.position_count()
    }

    fn remaining_rights(&self) -> CastleZoneSet {
        self.castling.rights()
    }

    fn to_partial_fen(&self, cmps: &[FenComponent]) -> String {
        fen::to_fen_impl(self, cmps)
    }
}
