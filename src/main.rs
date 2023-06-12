mod board;
mod pieces;
mod moves;
mod game;
mod square;

use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use crate::{pieces::{Knight, Piece}, square::Square};


fn main() {
    let mut game = game::GameManager::new();

    let pawn = Pawn::new(Color::White);
    let rook = Rook::new(Color::Black);
    let bishop = Bishop::new(Color::White);
    let queen = Queen::new(Color::Black);
    let king = King::new(Color::Black);
    let knight = Knight::new(Color::Black);

    game.get_board_mut().spawn(Piece::B(bishop), Square::C2).ok();
    game.get_board_mut().spawn(Piece::R(rook), Square::G2).ok();
    // game.get_board_mut().spawn(Piece::K(king), Square::A2).ok();
    // game.get_board_mut().spawn(Piece::N(knight), Square::A3).ok();
    // game.get_board_mut().spawn(Piece::Q(queen), Square::A8).ok();

    // bishop.get_position(game.get_board());
    println!("{}", game.get_board());
    
    game.select_piece(Square::G2).and_then(|_| {
        // game.move_piece(Square::B3).ok();

        Ok(())
    }).ok();

    println!("{}", game.get_board());

}
