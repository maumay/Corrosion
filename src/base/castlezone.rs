use crate::base::square::constants::A1;
use crate::base::square::constants::A8;
use crate::base::square::constants::B1;
use crate::base::square::constants::B8;
use crate::base::square::constants::C1;
use crate::base::square::constants::C8;
use crate::base::square::constants::E1;
use crate::base::square::constants::E8;
use crate::base::square::constants::F1;
use crate::base::square::constants::F8;
use crate::base::square::constants::G1;
use crate::base::square::constants::G8;
use crate::base::square::constants::H1;
use crate::base::square::constants::H8;
use crate::base::square::Square;
use std::iter::FromIterator;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct CastleZone {
    i: usize,
}

impl CastleZone {
    pub fn i(&self) -> usize {
        self.i
    }

    pub fn king_source(&self) -> Square {
        unimplemented!()
    }

    pub fn king_target(&self) -> Square {
        unimplemented!()
    }

    pub fn rook_source(&self) -> Square {
        unimplemented!()
    }

    pub fn rook_target(&self) -> Square {
        unimplemented!()
    }

    pub fn lift(&self) -> CastleZoneSet {
        CastleZoneSet {data: 1usize << self.i}
    }

    pub const WK: CastleZone = CastleZone {i: 0};
    pub const WQ: CastleZone = CastleZone {i: 1};
    pub const BK: CastleZone = CastleZone {i: 2};
    pub const BQ: CastleZone = CastleZone {i: 3};

    pub const ALL: [CastleZone; 4] = [
        CastleZone::WK,
        CastleZone::WQ,
        CastleZone::BK,
        CastleZone::BQ,
    ];
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct CastleZoneSet {
    data: usize,
}

impl CastleZoneSet {
    pub fn all() -> CastleZoneSet {
        CastleZoneSet {data: 0b1111}
    }

    pub fn none() -> CastleZoneSet {
        CastleZoneSet {data: 0}
    }

    pub fn contains(self, zone: CastleZone) -> bool {
        (1usize << zone.i) & self.data != 0
    }
}

impl<'a> FromIterator<&'a CastleZone> for CastleZoneSet {
    fn from_iter<T: IntoIterator<Item=&'a CastleZone>>(iter: T) -> Self {
        CastleZoneSet{data: iter.into_iter().map(|cz| 1usize << cz.i).fold(0, |a, b| a | b)}
    }
}

impl ops::Sub<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn sub(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet {data: self.data & !rhs.data}
    }
}

impl ops::BitOr<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn bitor(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet{data: self.data | rhs.data}
    }
}

impl ops::BitAnd<CastleZoneSet> for CastleZoneSet {
    type Output = CastleZoneSet;

    fn bitand(self, rhs: CastleZoneSet) -> Self::Output {
        CastleZoneSet{data: self.data & rhs.data}
    }
}


#[cfg(test)]
mod set_test {
    use super::*;

    #[test]
    fn test_all() {
        let all = CastleZoneSet::all();
        for &zone in &CastleZone::ALL {
            assert!(all.contains(zone));
        }
    }

    #[test]
    fn test_none() {
        let none = CastleZoneSet::none();
        for &zone in &CastleZone::ALL {
            assert!(!none.contains(zone));
        }
    }

    #[test]
    fn test_collect() {
        let source = vec![&CastleZone::BK, &CastleZone::WK, &CastleZone::WQ, &CastleZone::BQ];
        let collected: CastleZoneSet = source.into_iter().collect();
        assert_eq!(CastleZoneSet::all(), collected);
    }

//    #[test]
//    fn test_add() {
//        let mut set = CastleZoneSet::none();
//        assert!(!set.contains(&CastleZone::BK));
//        set.add(&CastleZone::BK);
//        assert!(set.contains(&CastleZone::BK));
//    }
//
//    #[test]
//    fn test_remove() {
//        let mut set = CastleZoneSet::all();
//        assert!(set.contains(&CastleZone::WQ));
//        set.remove(&CastleZone::WQ);
//        assert!(!set.contains(&CastleZone::WQ));
//    }
}
