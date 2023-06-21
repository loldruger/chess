use std::io;

use crate::{pieces::{Piece, Pawn, Color, Bishop, King, Rook, Knight, Queen}, square::Square};

mod pieces;
mod square;
mod board;
mod game;

fn main() {
    let mut game = game::GameManager::new();

    // game.get_board_mut().spawn(Square::A2, Piece::B(Bishop::new(Color::Black))).ok();
    // game.get_board_mut().spawn(Square::B6, Piece::Q(Queen::new(Color::White))).ok();
    game.get_board_mut().spawn(Square::E3, Piece::K(King::new(Color::White))).ok();
    // game.get_board_mut().spawn(Square::E6, Piece::P(Pawn::new(Color::White))).ok();
    // game.get_board_mut().spawn(Square::D2, Piece::P(Pawn::new(Color::White))).ok();
    game.get_board_mut().spawn(Square::E7, Piece::R(Rook::new(Color::Black))).ok();
    game.get_board_mut().spawn(Square::A4, Piece::R(Rook::new(Color::Black))).ok();
    game.get_board_mut().spawn(Square::D6, Piece::K(King::new(Color::Black))).ok();

    print!("{}", game.get_board());

    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {

        // match game.get_state() {
        //     game::GameState::Playing { by_color } => {
        //         if game.get_board().is_king_checked(by_color) {
        //             game.state = game::GameState::InCheck { by_color: *by_color };
        //         }
        //     },
        //     game::GameState::InCheck { by_color } => {
        //         if game.get_board().is_king_checked(by_color) {
        //             game.state = game::GameState::Checkmate { by_color: *by_color };
        //         } else {
        //             game.state = game::GameState::Playing { by_color: *by_color };
        //         }
        //     },
        //     game::GameState::Checkmate { by_color } => {
        //         println!("Checkmate! {} wins!", by_color.opposite());
        //         break;
        //     },
        //     game::GameState::Promoting { by_color } => {
        //         println!("Promoting {}'s pawn", by_color);
        //         break;
        //     },
        //     game::GameState::Stalemate => {
        //         println!("Stalemate!");
        //         break;
        //     },
        // }



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
