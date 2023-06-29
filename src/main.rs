mod pieces;
mod square;
mod board;
mod game;

use std::io;

use pieces::Piece::{P, N, B, R, Q, K};
use square::Square;
use board::Board;
use game::GameManager;

use crate::pieces::{Piece, Pawn, Color, Rook, King};

fn main() {
    let mut game = GameManager::new();
    game.get_board_mut().spawn(K(King::new(Color::Black)), Square::B7).ok();
    game.get_board_mut().spawn(P(Pawn::new(Color::White)), Square::B6).ok();
    // game.get_board_mut().spawn(R(Rook::new(Color::Black)), Square::B4).ok();

    // game.get_board_mut().clear_marks();
    // board.move_piece(Square::A1, Square::A2).ok();
    print!("{}", game.get_board());

    let mut user_input = String::new();
    let stdin = io::stdin();
    loop {
        match &game.get_state() {
            game::GameState::Playing { turn } => {
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
            
                    if game.select_piece(coord.unwrap()).is_err() {
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
            },
            game::GameState::InCheck { by_color } => {
                
            },
            game::GameState::Promoting { pawn } => {
                // loop {
                //     println!("select a piece to promote to: (Q, R, B, N) ");
                //     stdin.read_line(&mut user_input).unwrap();

                //     match user_input.trim() {
                //         "Q" | "q" => { pawn.try_into_queen(); break; },
                //         "R" | "r" => { pawn.try_into_rook(); break; },
                //         "B" | "b" => { pawn.try_into_bishop(); break; },
                //         "N" | "n" => { pawn.try_into_knight(); break; },
                //         _ => {
                //             println!("invalid input");
                //             user_input.clear();
                //             continue;
                //         }
                //     };
                // }

                // user_input.clear();
                
            },
        }
    }
}