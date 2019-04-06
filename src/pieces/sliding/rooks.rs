use std::iter::repeat;

use crate::base::bitboard::BitBoard;
use crate::base::square::constants::SQUARES;
use crate::base::square::Square;
use crate::pieces::BlackRook;
use crate::pieces::Piece;
use crate::pieces::WhiteRook;

use super::{compute_control, compute_powerset, compute_rook_index, rook_dirs, ROOK_MASKS};

/// Piece trait implementation for the white rook singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for WhiteRook {
    fn index(&self) -> usize {
        3
    }

    fn id(&self) -> &'static str {
        "wr"
    }

    fn control(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_rook_index(location, white | black)]
    }

    fn moves(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) - white
    }

    fn attacks(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) & black
    }
}

/// Piece trait implementation for the black rook singleton struct.
/// The move database is cached in the static memory and the code for
/// that is at the bottom of this file.
impl Piece for BlackRook {
    fn index(&self) -> usize {
        9
    }

    fn id(&self) -> &'static str {
        "br"
    }

    fn control(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        MOVES[location.i as usize][compute_rook_index(location, white | black)]
    }

    fn moves(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
        self.control(location, white, black) - black
    }

    fn attacks(&self, location: Square, white: BitBoard, black: BitBoard) -> BitBoard {
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
    let dirs = rook_dirs();
    for (&sq, bb) in izip!(SQUARES.iter(), ROOK_MASKS.iter().map(|&m| BitBoard(m))) {
        let dest_size = 1 << bb.size();
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(dest_size).collect();
        for occ_var in compute_powerset(&bb.into_iter().collect()) {
            let index = compute_rook_index(sq, occ_var);
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

    use super::{compute_move_database, compute_rook_index, Moves};

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(&moves);
        //test_case_two(&moves);
    }

    fn test_case_one(moves: &Moves) {
        let (sq, occ) = (D3, D1 | D5 | D6 | G3 | C3 | A6 | H8);
        let expected = D2 | D1 | D4 | D5 | E3 | F3 | G3 | C3;
        assert_eq!(expected, moves[sq.i as usize][compute_rook_index(sq, occ)])
    }
}
