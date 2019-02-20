#[macro_use]
extern crate itertools;
//extern crate bitwise;

use crate::bitboard::BitBoard;
use crate::square::*;
use crate::square::constants::*;

mod square;
mod bitboard;
mod dir;

fn main() {
    let dirs = vec!(&dir::N);
    let board = BitBoard::new(&[D2, H3]);
    let board2 = BitBoard::new(&[A3, G7]);
    println!("{}", board | board2);
    println!("{}", board | F3);
    println!("{}", F3 | board);
    println!("{}", G1 | A8);
    let bitboard: BitBoard = vec!(A1, G5).into_iter().collect();
    println!("{}", bitboard);
//    let squares = square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", square::H1);
}

