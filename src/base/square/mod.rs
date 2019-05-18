use itertools::iterate;

use crate::base::bitboard::BitBoard;
use crate::base::dir::Dir;
use crate::base::Reflectable;

pub mod constants;
mod traits;

/// Value type representing a square on a chessboard.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Square {
    pub i: u8,
    rank: u8,
    file: u8,
}

impl Square {
    /// Retrieve a string identifier for this square.
    pub const fn name(self) -> &'static str {
        NAMES[self.i as usize]
    }

    /// Return the index of the rank on which this square resides.
    pub const fn rank_index(self) -> usize {
        self.rank as usize
    }

    /// Return the index of the file on which this square resides.
    pub const fn file_index(self) -> usize {
        self.file as usize
    }

    /// Return a bitboard representing the rank on which this square
    /// resides.
    pub fn rank(self) -> BitBoard {
        BitBoard::RANKS[self.rank as usize]
    }

    /// Return a bitboard representing the file on which this square
    /// resides.
    pub fn file(self) -> BitBoard {
        unimplemented!()
    }

    /// 'Lifts' this square to a singleton set of squares.
    pub const fn lift(self) -> BitBoard {
        BitBoard(1u64 << self.i)
    }

    /// Finds the next square on a chessboard from this square in a
    /// given direction if it exists.
    pub fn next(self, dir: Dir) -> Option<Square> {
        let new_rank = (self.rank as i8) + dir.dr;
        let new_file = (self.file as i8) + dir.df;
        if -1 < new_rank && new_rank < 8 && -1 < new_file && new_file < 8 {
            Some(constants::SQUARES[(8 * new_rank + new_file) as usize])
        } else {
            None
        }
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a set.
    pub fn search(self, dir: Dir) -> BitBoard {
        self.search_vec(dir).into_iter().collect()
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a vector where the squares are ordered in
    /// increasing distance from this square.
    pub fn search_vec(self, dir: Dir) -> Vec<Square> {
        iterate(Some(self), |op| op.and_then(|sq| sq.next(dir)))
            .skip(1)
            .take_while(|op| op.is_some())
            .map(|op| op.unwrap())
            .collect()
    }

    /// Find all squares in all directions in a given vector and
    /// returns them as a set.
    pub fn search_all(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter().flat_map(|&dir| self.search(dir)).collect()
    }

    /// Find the squares adjacent to this square in all of the
    /// given directions and returns them as a set.
    pub fn search_one(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter()
            .flat_map(|&dir| self.next(dir).into_iter())
            .collect()
    }

    const fn new(index: u8) -> Square {
        Square {
            i: index,
            rank: index / 8,
            file: index % 8,
        }
    }
}

impl Reflectable for Square {
    fn reflect(&self) -> Self {
        let (fi, ri) = (self.file_index(), self.rank_index());
        constants::SQUARES[(8 * (7 - ri) + fi) as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::base::bitboard::*;
    use crate::base::dir::*;

    use super::constants::*;

    #[test]
    fn test_rank() {
        assert_eq!(A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1, F1.rank());
        assert_eq!(A4 | B4 | C4 | D4 | E4 | F4 | G4 | H4, D4.rank());
        assert_eq!(A8 | B8 | C8 | D8 | E8 | F8 | G8 | H8, A8.rank());
    }

    #[test]
    fn test_partial_ord() {
        for i in 0..64 {
            let prev: Vec<_> = SQUARES.iter().take(i).map(|x| *x).collect();
            let next: Vec<_> = SQUARES.iter().skip(i + 1).map(|x| *x).collect();
            let pivot = SQUARES[i];

            for smaller in prev {
                assert_eq!(true, smaller < pivot);
            }

            for larger in next {
                assert_eq!(true, pivot < larger);
            }
        }
    }

    #[test]
    fn test_search() {
        assert_eq!(D3.search(S), D2 | D1);
    }

    #[test]
    fn test_search_vec() {
        assert_eq!(D3.search_vec(S), vec![D2, D1])
    }

    #[test]
    fn test_search_one() {
        assert_eq!(D3.search_one(&vec!(S, E)), D2 | E3);
        assert_eq!(A8.search_one(&vec!(N, NWW, SE)), B7.lift());
    }

    #[test]
    fn test_search_all() {
        assert_eq!(C3.search_all(&vec!(SSW, SWW, S)), B1 | A2 | C2 | C1);
    }

    #[test]
    fn test_next() {
        assert_eq!(C3.next(N), Some(C4));
        assert_eq!(C3.next(E), Some(D3));
        assert_eq!(C3.next(S), Some(C2));
        assert_eq!(C3.next(W), Some(B3));
        assert_eq!(C3.next(NE), Some(D4));
        assert_eq!(C3.next(SE), Some(D2));
        assert_eq!(C3.next(SW), Some(B2));
        assert_eq!(C3.next(NW), Some(B4));
        assert_eq!(C3.next(NNE), Some(D5));
        assert_eq!(C3.next(NEE), Some(E4));
        assert_eq!(C3.next(SEE), Some(E2));
        assert_eq!(C3.next(SSE), Some(D1));
        assert_eq!(C3.next(SSW), Some(B1));
        assert_eq!(C3.next(SWW), Some(A2));
        assert_eq!(C3.next(NWW), Some(A4));
        assert_eq!(C3.next(NNW), Some(B5));

        assert_eq!(G8.next(N), None);
        assert_eq!(H6.next(E), None);
        assert_eq!(B1.next(S), None);
        assert_eq!(A4.next(W), None);
    }
}

const NAMES: [&str; 64] = [
    "H1", "G1", "F1", "E1", "D1", "C1", "B1", "A1", "H2", "G2", "F2", "E2", "D2", "C2", "B2", "A2",
    "H3", "G3", "F3", "E3", "D3", "C3", "B3", "A3", "H4", "G4", "F4", "E4", "D4", "C4", "B4", "A4",
    "H5", "G5", "F5", "E5", "D5", "C5", "B5", "A5", "H6", "G6", "F6", "E6", "D6", "C6", "B6", "A6",
    "H7", "G7", "F7", "E7", "D7", "C7", "B7", "A7", "H8", "G8", "F8", "E8", "D8", "C8", "B8", "A8",
];
