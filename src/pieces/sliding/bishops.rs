use std::iter::repeat;

use crate::base::bitboard::BitBoard;
use crate::base::square::constants::SQUARES;
use crate::base::square::Square;
use crate::pieces::BlackBishop;
use crate::pieces::Piece;
use crate::pieces::WhiteBishop;

use super::{bishop_dirs, compute_bishop_index, compute_control, compute_powerset, BISHOP_MASKS};

/// Piece trait implementation for the white bishop singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for WhiteBishop {
    fn control(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_bishop_index(location, white | black)]
    }

    fn moves(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) - white
    }

    fn attacks(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) & black
    }
}

/// Piece trait implementation for the black bishop singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for BlackBishop {
    fn control(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_bishop_index(location, white | black)]
    }

    fn moves(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) - black
    }

    fn attacks(self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) & white
    }
}

/// Implementation and tests for the static magic move database.
///
type Moves = Vec<Vec<BitBoard>>;

lazy_static! {
    static ref MOVES: Moves = compute_move_database();
}

fn compute_move_database() -> Moves {
    let mut dest = Vec::with_capacity(64);
    let dirs = bishop_dirs();
    for (&sq, bb) in izip!(SQUARES.iter(), BISHOP_MASKS.iter().map(|&m| BitBoard(m))) {
        let dest_size = 1 << bb.size();
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(dest_size).collect();
        for occ_var in compute_powerset(&bb.into_iter().collect()) {
            let index = compute_bishop_index(sq, occ_var);
            if sq_dest[index] == BitBoard::ALL {
                sq_dest[index] = compute_control(sq, occ_var, &dirs);
            }
        }
        dest.push(sq_dest);
    }
    dest
}

#[cfg(test)]
mod test {
    use crate::base::square::constants::*;

    use super::{compute_bishop_index, compute_move_database, Moves};

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(&moves);
        test_case_two(&moves);
    }

    fn test_case_one(moves: &Moves) {
        let (sq, occ) = (D3, E2 | B1 | F5 | H7 | D4);
        let expected = E2 | C2 | B1 | C4 | B5 | A6 | E4 | F5;
        assert_eq!(
            expected,
            moves[sq.i as usize][compute_bishop_index(sq, occ)]
        )
    }

    fn test_case_two(moves: &Moves) {
        let (sq, occ) = (H5, D1 | E2 | G6 | C5 | F6 | A1 | A4 | D2);
        let expected = G4 | F3 | E2 | G6;
        assert_eq!(
            expected,
            moves[sq.i as usize][compute_bishop_index(sq, occ)]
        )
    }
}
