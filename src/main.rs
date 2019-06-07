extern crate rand;

use rand::thread_rng;
use rand::prelude::SliceRandom;

mod puzzle;

use puzzle::{Piece, Puzzle, Solver};

fn main() {
    let mut pieces = vec![
        Piece::from("GBRYGRYB"),
        Piece::from("RBYGRYGB"),
        Piece::from("RGYGBRYB"),
        Piece::from("RBGYRGYB"),
        Piece::from("YRGRBYGB"),

        Piece::from("GRBRYGBY"),
        Piece::from("GRYRBGYB"),
        Piece::from("BRGRYBGY"),
        Piece::from("BRGYBGYR"),
        Piece::from("BRYRGBYG"),

        Piece::from("RGBGYRBY"),
        Piece::from("BGYGRBYR"),
        Piece::from("GRYBGBRY"),
        Piece::from("BGRGYBRY"),
        Piece::from("YGBGRYBR"),

        Piece::from("YGRGBYRB"),
        Piece::from("GBYRGYRB"),
        Piece::from("YBRGYRGB"),
        Piece::from("YBGRYGRB"),
        Piece::from("RYBYGRBG"),
        
        Piece::from("RYGYBRGB"),
        Piece::from("BYRYGBRG"),
        Piece::from("GYBYRGBR"),
        Piece::from("BYGYRBGR"),
        Piece::from("GYRYBGRB")
    ];

    // let mut rng = thread_rng();
    // pieces.shuffle(&mut rng);

    let puzzle = Puzzle::new(5, 5, pieces);
    let mut solver = Solver::new(puzzle);
    let results = solver.solve();

    if results {
        println!("{}", solver.get_puzzle());
    }
}
