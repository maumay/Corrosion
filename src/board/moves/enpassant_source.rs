use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Reflectable;
use crate::base::Side;
use crate::board::moves::FILES;

/// TODO Could have adjacent files in a constant array
pub(super) fn squares(active: Side, enpassant_target: Square) -> BitBoard {
    let fi = enpassant_target.file_index() as usize;
    let adjacent_files = match fi % 7 {
        0 => {
            if fi == 0 {
                FILES[1]
            } else {
                FILES[6]
            }
        }
        _ => FILES[fi + 1] | FILES[fi - 1],
    };
    adjacent_files & active.reflect().pawn_third_rank()
}

#[cfg(test)]
mod test_enpassant_source_squares {
    use crate::base::bitboard::constants::*;
    use crate::base::square::constants;
    use crate::base::Side;

    use super::squares;

    #[test]
    fn test() {
        assert_eq!(H4 | F4, squares(Side::Black, constants::G3));
        assert_eq!(G4, squares(Side::Black, constants::H3));
        assert_eq!(B4, squares(Side::Black, constants::A3));
        assert_eq!(H5 | F5, squares(Side::White, constants::G6));
        assert_eq!(G5, squares(Side::White, constants::H6));
        assert_eq!(B5, squares(Side::White, constants::A6));
    }
}
