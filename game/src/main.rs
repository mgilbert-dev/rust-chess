extern crate board;
extern crate eval;
use std::env;

use board::helpers::generate_board;
use board::Move;
use eval::get_all_legal_moves;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
        let mut board: board::Board = generate_board(board_string);
        let legal_moves: Vec<Move> = get_all_legal_moves(board);
        println!("{:?}", legal_moves);
    } else {
        let board_string = String::from(args[1].clone());
        let mut board: board::Board = generate_board(board_string);
        let legal_moves: Vec<Move> = get_all_legal_moves(board);
        println!("{:?}", legal_moves);
    }
}
