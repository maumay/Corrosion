use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::{Reflectable, Side};
use crate::board::implementation::cache::rays::RaySet;
use crate::board::{Board, BoardImpl};
use crate::pieces::Piece;

impl BoardImpl {
    pub fn compute_discoveries(&self) -> RaySet {
        let locs = |side: Side| self.side(side);
        let (active, passive) = (locs(self.active), locs(self.active.reflect()));
        let king_loc = self.pieces.king_location(self.active.reflect());
        let (mut discovery_rays, mut i) = (super::empty_ray_pairs(), 0);
        let mut discovers = BitBoard::EMPTY;
        for xrayer in self.compute_xrayers(king_loc) {
            let cord = BitBoard::cord(king_loc, xrayer);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 && i < super::MAX_SIZE {
                let discov_loc = ((cord & active) - xrayer).first().unwrap();
                discovery_rays[i] = Some((discov_loc, cord));
                i += 1;
                discovers |= discov_loc;
            }
        }
        RaySet { ray_points: discovers, rays: discovery_rays }
    }

    fn compute_xrayers(&self, king_loc: Square) -> BitBoard {
        let active_sliders = match self.active {
            Side::White => super::WHITE_SLIDERS,
            Side::Black => super::BLACK_SLIDERS,
        };
        let locs = |p: Piece| self.locs(p);
        active_sliders.iter().flat_map(|&p| locs(p) & p.empty_control(king_loc)).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::base::bitboard::constants::*;

    use super::*;
    use super::super::empty_ray_pairs;

    fn execute_test(fen: &'static str, expected_discoveries: RaySet) {
        let board = crate::board::from_fen(fen).unwrap();
        assert_eq!(expected_discoveries.reflect(), board.reflect().compute_discoveries());
        assert_eq!(expected_discoveries, board.compute_discoveries());
    }

    #[test]
    fn case_one() {
        let fen = "6r1/5p1k/4pP2/4N3/3PN3/6P1/2B3PK/7R w - - 1 10";
        let mut rays = empty_ray_pairs();
        rays[0] = Some((Square::E4, C2 | D3 | E4 | F5 | G6 | H7));
        rays[1] = Some((Square::H2, H1 | H2 | H3 | H4 | H5 | H6 | H7));
        let expected_pinned = RaySet {
            ray_points: E4 | H2,
            rays,
        };
        execute_test(fen, expected_pinned);
    }
}
