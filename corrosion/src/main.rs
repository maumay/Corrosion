#[macro_use]
extern crate itertools;

mod square;
mod squareset;
mod dir;

use self::square::Square;

fn main() {
    let dirs = vec!(&dir::N);
    println!("hi");
//    let squares = square::H1.search_one(dirs);
//    println!("{:#?}", squares);
//    println!("{}", square::H1);

}

fn some_func(mut input_ref: &Square) {
    input_ref = &square::H1;
}

fn first2(square: Square) {
    println!("{:?}", square);
}

fn first(mut square: Square) {
    square.i = 5;
    println!("{:?}", square);
}

fn second(square: &Square) {
    println!("{:?}", square);
}

fn third(square: &mut Square) {
    square.i = 5;
    println!("{:?}", square);
}
