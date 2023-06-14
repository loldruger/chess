mod board;
mod pieces;
mod moves;
mod game;
mod square;

use std::io;

use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use crate::{pieces::{Knight, Piece}, square::Square};


fn main() {
    let mut game = game::GameManager::new();

    // game.get_board_mut().spawn(Piece::P(Pawn::new(Color::White)), Square::A8).ok();
    // game.get_board_mut().spawn(Piece::P(Pawn::new(Color::White)), Square::A3).ok();

    game.get_board_mut().spawn(Piece::R(Rook::new(Color::Black)), Square::A1).ok();
    game.get_board_mut().spawn(Piece::R(Rook::new(Color::Black)), Square::H1).ok();
    // game.get_board_mut().spawn(Piece::B(Bishop::new(Color::White)), Square::C4).ok();
    game.get_board_mut().spawn(Piece::N(Knight::new(Color::White)), Square::F3).ok();
    // game.get_board_mut().spawn(Piece::Q(Queen::new(Color::Black)), Square::F1).ok();
    game.get_board_mut().spawn(Piece::K(King::new(Color::Black)), Square::E1).ok();

    // bishop.get_position(game.get_board());
    println!("{}", game.get_board());

    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        if game.get_turn() == Color::White {
            print!("White's turn: ");
        } else {
            print!("Black's turn: ");
        }

        println!("Select a piece at: ");
        stdin.read_line(&mut user_input).unwrap();
        let coord = Square::from_str(&user_input[0..2]);

        if coord.is_none() {
            println!("invalid input");
            user_input.clear();
            continue;
        }

        if game.select_piece(coord.unwrap()).is_err() {
            println!("invalid piece");
            user_input.clear();
            continue;
        }

        user_input.clear();
        println!("{}", game.get_board());

        loop {
            println!("move the piece to: ");
            stdin.read_line(&mut user_input).unwrap();
            if game.move_piece(Square::from_str(&user_input[0..2]).unwrap()).is_err() {
                println!("invalid move");
                user_input.clear();
                continue;
            } else {
                break;
            }
        }
        // game.reset_threaten();
        user_input.clear();
        println!("{}", game.get_board());
    }
    
}
