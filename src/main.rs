use std::io;

use crate::{pieces::{Piece, Pawn, Color, Bishop, King, Rook, Knight, Queen}, square::Square};

mod pieces;
mod square;
mod board;
mod game;

fn main() {
    let mut game = game::GameManager::new();

    // game.get_board_mut().spawn(Square::A1, Piece::B(Bishop::new(Color::White))).ok();
    // game.get_board_mut().spawn(Square::B6, Piece::Q(Queen::new(Color::White))).ok();
    game.get_board_mut().spawn(Square::D5, Piece::K(King::new(Color::Black))).ok();
    game.get_board_mut().spawn(Square::D1, Piece::R(Rook::new(Color::White))).ok();
    game.get_board_mut().spawn(Square::D2, Piece::P(Pawn::new(Color::Black))).ok();
    game.get_board_mut().spawn(Square::D3, Piece::P(Pawn::new(Color::Black))).ok();
    game.get_board_mut().spawn(Square::H5, Piece::R(Rook::new(Color::White))).ok();

    print!("{}", game.get_board());

    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        match game.get_turn() {
            Color::White => print!("White's turn, "),
            Color::Black => print!("Black's turn, "),
        }

        println!("Select a piece: ");
        stdin.read_line(&mut user_input).expect("Failed to read line");
        let coord = Square::from_str(&user_input[0..2]);

        if coord.is_none() {
            println!("invalid input");
            user_input.clear();
            continue;
        }

        if game.select_piece(coord.unwrap()).is_none() {
            println!("invalid piece");
            user_input.clear();
            continue;
        }

        print!("{}", game.get_board());
        user_input.clear();
        
        loop {
            println!("move the piece to: ");
            stdin.read_line(&mut user_input).unwrap();
            if game.move_piece(coord.unwrap(), Square::from_str(&user_input[0..2]).unwrap()).is_err() {
                user_input.clear();
                continue;
            };
    
            user_input.clear();
            break;
        }

        print!("{}", game.get_board());
    }
}
