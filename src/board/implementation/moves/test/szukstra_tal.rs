use super::*;

#[test]
fn black_move_eight() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | C4 | D4 | E4 | F3 | G2 | H2,
                C3 | E2,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | E5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 1,
            hash_offset: 14,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::BP, A7, A6 | A5),
            s(Piece::BP, B7, B6 | B5),
            s(Piece::BN, B8, A6 | D7),
            s(Piece::BP, C6, C5),
            s(Piece::BB, C8, D7 | E6 | F5 | G4 | H3),
            s(Piece::BP, D6, D5),
            s(Piece::BQ, D8, C7 | B6 | A5 | D7 | E8 | E7),
            s(Piece::BP, E5, D4),
            s(Piece::BN, F6, D5 | D7 | E8 | H5 | G4 | E4),
            s(Piece::BR, F8, E8),
            s(Piece::BK, G8, H8),
            s(Piece::BB, G7, H6 | H8),
            s(Piece::BP, G6, G5),
            s(Piece::BP, H7, H6 | H5),
        ],

        expected_attacks: vec![s(Piece::BP, E5, D4), s(Piece::BN, F6, E4)],
        expected_attacks_checks: vec![s(Piece::BP, E5, D4), s(Piece::BN, F6, E4)],
    });
}

#[test]
fn white_move_nine() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | E2,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | D4 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            c(CastleZone::WQ.lift()),
            s(Piece::WR, A1, B1 | C1 | D1),
            s(Piece::WP, A2, A3 | A4),
            s(Piece::WQ, B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7),
            s(Piece::WN, C3, B1 | D1 | D5 | A4 | B5),
            s(Piece::WP, C4, C5),
            s(Piece::WK, E1, D1 | D2 | F2),
            s(Piece::WN, E2, C1 | D4 | F4 | G3 | G1),
            s(Piece::WB, E3, D4 | D2 | C1 | F4 | G5 | H6 | F2 | G1),
            s(Piece::WP, E4, E5),
            s(Piece::WP, F3, F4),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1),
            s(Piece::WP, H2, H3 | H4),
        ],

        expected_attacks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WB, E3, D4),
            s(Piece::WN, E2, D4),
        ],
        expected_attacks_checks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WB, E3, D4),
            s(Piece::WN, E2, D4),
        ],
    });
}

#[test]
fn black_move_nine() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D6 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::BP, A7, A6 | A5),
            s(Piece::BP, B7, B6 | B5),
            s(Piece::BN, B8, A6 | D7),
            s(Piece::BP, C6, C5),
            s(Piece::BB, C8, D7 | E6 | F5 | G4 | H3),
            s(Piece::BP, D6, D5),
            s(Piece::BQ, D8, C7 | B6 | A5 | D7 | E8 | E7),
            s(Piece::BN, F6, D5 | D7 | E8 | H5 | G4 | E4),
            s(Piece::BR, F8, E8),
            s(Piece::BK, G8, H8),
            s(Piece::BB, G7, H6 | H8),
            s(Piece::BP, G6, G5),
            s(Piece::BP, H7, H6 | H5),
        ],

        expected_attacks: vec![s(Piece::BN, F6, E4)],
        expected_attacks_checks: vec![s(Piece::BN, F6, E4)],
    });
}

#[test]
fn white_move_ten() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | C4 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | D5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            c(CastleZone::WQ.lift()),
            s(Piece::WR, A1, B1 | C1 | D1),
            s(Piece::WP, A2, A3 | A4),
            s(Piece::WQ, B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7),
            s(Piece::WN, C3, B1 | D1 | D5 | A4 | B5 | E2),
            s(Piece::WP, C4, D5 | C5),
            s(Piece::WK, E1, D1 | D2 | F2 | E2),
            s(Piece::WN, D4, C2 | B5 | E6 | F5 | E2 | C6),
            s(Piece::WB, E3, D2 | C1 | F4 | G5 | H6 | F2 | G1),
            s(Piece::WB, F1, E2 | D3),
            s(Piece::WP, E4, D5 | E5),
            s(Piece::WP, F3, F4),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1),
            s(Piece::WP, H2, H3 | H4),
        ],

        expected_attacks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WP, E4, D5),
            s(Piece::WP, C4, D5),
            s(Piece::WN, C3, D5),
            s(Piece::WN, D4, C6),
        ],
        expected_attacks_checks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WP, E4, D5),
            s(Piece::WP, C4, D5),
            s(Piece::WN, C3, D5),
            s(Piece::WN, D4, C6),
        ],
    });
}

#[test]
fn black_move_ten() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | D5 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | C6 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::BP, A7, A6 | A5),
            s(Piece::BP, B7, B6 | B5),
            s(Piece::BN, B8, A6 | D7),
            s(Piece::BP, C6, C5 | D5),
            s(Piece::BB, C8, D7 | E6 | F5 | G4 | H3),
            s(Piece::BQ, D8, C7 | B6 | A5 | D7 | D6 | D5 | E8 | E7),
            s(Piece::BN, F6, D5 | D7 | E8 | H5 | G4 | E4),
            s(Piece::BR, F8, E8),
            s(Piece::BK, G8, H8),
            s(Piece::BB, G7, H6 | H8),
            s(Piece::BP, G6, G5),
            s(Piece::BP, H7, H6 | H5),
        ],

        expected_attacks: vec![
            s(Piece::BN, F6, E4 | D5),
            s(Piece::BP, C6, D5),
            s(Piece::BQ, D8, D5),
        ],

        expected_attacks_checks: vec![
            s(Piece::BN, F6, E4 | D5),
            s(Piece::BP, C6, D5),
            s(Piece::BQ, D8, D5),
        ],
    });
}

#[test]
fn white_move_eleven() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | E4 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![
                A7 | B7 | D5 | F7 | G6 | H7,
                B8 | F6,
                C8 | G7,
                A8 | F8,
                D8,
                G8,
            ],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            c(CastleZone::WQ.lift()),
            s(Piece::WR, A1, B1 | C1 | D1),
            s(Piece::WP, A2, A3 | A4),
            s(
                Piece::WQ,
                B3,
                C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7 | C4 | D5,
            ),
            s(Piece::WN, C3, B1 | D1 | D5 | A4 | B5 | E2),
            s(Piece::WK, E1, D1 | D2 | F2 | E2),
            s(Piece::WN, D4, C2 | B5 | E6 | F5 | E2 | C6),
            s(Piece::WB, E3, D2 | C1 | F4 | G5 | H6 | F2 | G1),
            s(Piece::WB, F1, E2 | D3 | C4 | B5 | A6),
            s(Piece::WP, E4, D5 | E5),
            s(Piece::WP, F3, F4),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1),
            s(Piece::WP, H2, H3 | H4),
        ],

        expected_attacks: vec![
            s(Piece::WQ, B3, B7 | D5),
            s(Piece::WP, E4, D5),
            s(Piece::WN, C3, D5),
        ],
        expected_attacks_checks: vec![
            s(Piece::WQ, B3, B7 | D5),
            s(Piece::WP, E4, D5),
            s(Piece::WN, C3, D5),
        ],
    });
}

#[test]
fn black_move_eleven() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::Black,
            whites: vec![
                A2 | B2 | D5 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![A7 | B7 | F7 | G6 | H7, B8 | F6, C8 | G7, A8 | F8, D8, G8],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            s(Piece::BP, A7, A6 | A5),
            s(Piece::BP, B7, B6 | B5),
            s(Piece::BN, B8, A6 | D7 | C6),
            s(Piece::BB, C8, D7 | E6 | F5 | G4 | H3),
            s(Piece::BQ, D8, C7 | B6 | A5 | D7 | D6 | D5 | E8 | E7),
            s(Piece::BN, F6, D5 | D7 | E8 | H5 | G4 | E4),
            s(Piece::BR, F8, E8),
            s(Piece::BK, G8, H8),
            s(Piece::BB, G7, H6 | H8),
            s(Piece::BP, G6, G5),
            s(Piece::BP, H7, H6 | H5),
        ],

        expected_attacks: vec![s(Piece::BN, F6, D5), s(Piece::BQ, D8, D5)],

        expected_attacks_checks: vec![s(Piece::BN, F6, D5), s(Piece::BQ, D8, D5)],
    });
}

#[test]
fn white_move_twelve() {
    execute_test(TestCase {
        board: TestBoard {
            active: Side::White,
            whites: vec![
                A2 | B2 | D5 | F3 | G2 | H2,
                C3 | D4,
                E3 | F1,
                A1 | H1,
                B3,
                E1,
            ],
            blacks: vec![A7 | B7 | F7 | G6 | H7, C6 | F6, C8 | G7, A8 | F8, D8, G8],
            clock: 0,
            hash_offset: 15,
            castle_rights: CastleZoneSet::WHITE,
            white_status: None,
            black_status: Some(CastleZone::BK),
            enpassant: None,
        },

        expected_all: vec![
            c(CastleZone::WQ.lift()),
            s(Piece::WR, A1, B1 | C1 | D1),
            s(Piece::WP, A2, A3 | A4),
            s(Piece::WQ, B3, C2 | D1 | A3 | A4 | B4 | B5 | B6 | B7 | C4),
            s(Piece::WN, C3, B1 | D1 | A4 | B5 | E2 | E4),
            s(Piece::WK, E1, D1 | D2 | F2 | E2),
            s(Piece::WN, D4, C2 | B5 | E6 | F5 | E2 | C6),
            s(Piece::WB, E3, D2 | C1 | F4 | G5 | H6 | F2 | G1),
            s(Piece::WB, F1, E2 | D3 | C4 | B5 | A6),
            s(Piece::WP, D5, D6 | C6),
            s(Piece::WP, F3, F4),
            s(Piece::WP, G2, G3 | G4),
            s(Piece::WR, H1, G1),
            s(Piece::WP, H2, H3 | H4),
        ],

        expected_attacks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WN, D4, C6),
            s(Piece::WP, D5, C6),
        ],
        expected_attacks_checks: vec![
            s(Piece::WQ, B3, B7),
            s(Piece::WN, D4, C6),
            s(Piece::WP, D5, C6),
        ],
    });
}
