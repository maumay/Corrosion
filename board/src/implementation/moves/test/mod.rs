use std::collections::btree_set::BTreeSet;

use crate::implementation::MutBoardImpl;
use crate::Move;
use crate::MoveComputeType;
use crate::MutBoard;
use myopic_core::*;

type MoveSet = BTreeSet<Move>;

#[cfg(test)]
mod misc;
#[cfg(test)]
mod szukstra_tal;

type ExpectedMoves = Vec<(MoveComputeType, MoveSet)>;

#[derive(Debug, Clone)]
struct TestCase {
    board: &'static str,
    // We use stacked vectors so we can more easily
    // write collections of moves shorthand.
    expected_all: Vec<Vec<Move>>,
    expected_attacks_checks: Vec<Vec<Move>>,
    expected_attacks: Vec<Vec<Move>>,
}

fn s(piece: Piece, src: BitBoard, targets: BitBoard) -> Vec<Move> {
    Move::standards(piece, src.first().unwrap(), targets).collect()
}

fn p(side: Side, src: BitBoard, targets: BitBoard) -> Vec<Move> {
    Move::promotions(side, src.first().unwrap(), targets).collect()
}

fn e(src: BitBoard, enpassant_target: Square) -> Vec<Move> {
    src.iter()
        .map(|s| Move::Enpassant(s, enpassant_target))
        .collect()
}

fn c(zones: CastleZoneSet) -> Vec<Move> {
    zones.iter().map(Move::Castle).collect()
}

fn flatten(moves: &Vec<Vec<Move>>) -> MoveSet {
    moves
        .iter()
        .flat_map(|xs| xs.iter().map(|mv| mv.clone()))
        .collect()
}

fn convert_moves(case: &TestCase) -> ExpectedMoves {
    vec![
        (MoveComputeType::All, flatten(&case.expected_all)),
        (MoveComputeType::Attacks, flatten(&case.expected_attacks)),
        (
            MoveComputeType::AttacksChecks,
            flatten(&case.expected_attacks_checks),
        ),
    ]
}

fn execute_test(case: TestCase) {
    let board = crate::fen_position(case.board).unwrap();
    let moves = convert_moves(&case);
    let ref_board = board.reflect();
    let ref_moves: Vec<_> = moves.iter().map(|(t, mvs)| (*t, mvs.reflect())).collect();
    execute_test_impl(board, moves);
    execute_test_impl(ref_board, ref_moves);
}

fn execute_test_impl(mut board: MutBoardImpl, moves: ExpectedMoves) {
    for (computation_type, expected_moves) in moves.into_iter() {
        let actual_moves: MoveSet = board.compute_moves(computation_type).into_iter().collect();
        assert_eq!(
            expected_moves.clone(),
            actual_moves.clone(),
            "Differences for {:?} are: {:?}",
            computation_type,
            compute_difference(expected_moves, actual_moves)
        );
    }
}

fn compute_difference(left: MoveSet, right: MoveSet) -> (MoveSet, MoveSet) {
    (
        left.clone().difference(&right).map(|m| m.clone()).collect(),
        right.clone().difference(&left).map(|m| m.clone()).collect(),
    )
}
