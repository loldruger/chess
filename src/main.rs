mod board;
mod pieces;
mod moves;
mod game;
mod square;

use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use crate::{pieces::{Knight, Piece}, square::Square};


fn main() {
    let mut game = game::GameManager::new();

    // game.get_board_mut().spawn(Piece::P(Pawn::new(Color::White)), Square::A8).ok();
    // game.get_board_mut().spawn(Piece::P(Pawn::new(Color::White)), Square::A3).ok();

    game.get_board_mut().spawn(Piece::R(Rook::new(Color::Black)), Square::A1).ok();
    game.get_board_mut().spawn(Piece::R(Rook::new(Color::Black)), Square::H1).ok();
    // game.get_board_mut().spawn(Piece::B(Bishop::new(Color::White)), Square::C4).ok();
    game.get_board_mut().spawn(Piece::N(Knight::new(Color::Black)), Square::F2).ok();
    game.get_board_mut().spawn(Piece::Q(Queen::new(Color::Black)), Square::F1).ok();
    game.get_board_mut().spawn(Piece::K(King::new(Color::Black)), Square::E1).ok();

    // bishop.get_position(game.get_board());
    println!("{}", game.get_board());

    game.select_piece(Square::E1).and_then(|_| {
        game.move_piece(Square::C1).ok();
        
        Ok(())
    }).ok();

    // game.select_piece(Square::D1).and_then(|_| {
    //     game.move_piece(Square::E1).ok();
    //     Ok(())
    // }).ok();

    // game.select_piece(Square::E1).and_then(|_| {

    //     Ok(())
    // }).ok();
    // game.select_piece(Square::D4).and_then(|_| {
    //     // game.move_piece(Square::B3).ok();

    //     Ok(())
    // }).ok();

    println!("{}", game.get_board());
    
}
