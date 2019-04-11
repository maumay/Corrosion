#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rand;
//extern crate bitwise;

use crate::base::bitboard::{BitBoard, simple::*};
use crate::base::dir;
use crate::base::square::constants::*;
use crate::pieces::{Piece, WhitePawn};
use crate::board::hash::gen_unique;

mod base;
mod pieces;
mod board;

fn main() {
    let _dirs = vec!(&dir::N);
    let board = D2 | H3;
    let board2 = A3 | G7;
    println!("{}", board | board2);
    println!("{}", board | F3);
    println!("{}", F3 | board);
    println!("{}", G1 | A8);
    println!("{}", G1 > H1);
    let bitboard: BitBoard = vec!(A1, G5).into_iter().collect();
    println!("{}", bitboard);
//    let squares = base.square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", base.square::H1);
    println!("{}", RANKS[1]);
    println!("{}", WhitePawn.control(H3, BitBoard::EMPTY, BitBoard::EMPTY));

//    let x = pieces::pawns::BLACK_CONTROL.clone().into_iter().map(|x| x.0).collect::<Vec<_>>();
//    println!("{:?}", x);
    println!("{:?}", gen_unique(800));
    let x = board::tables::midgame_eval(&WhitePawn, D5);
}

