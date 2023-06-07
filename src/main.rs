mod board;
mod pieces;
mod moves;

use board::Board;
use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use moves::{Placable, Position};

use crate::pieces::Knight;

fn main() {
    let mut board = Board::new();

    let mut pawn = Pawn::new(Color::White);
    let mut rook = Rook::new(Color::Black);
    let mut bishop = Bishop::new(Color::White);
    let mut queen = Queen::new(Color::Black);
    let mut king = King::new(Color::Black);
    let mut knight = Knight::new(Color::Black);

    bishop.set_position(Position::from_str("C4").unwrap()).ok();
    king.set_position(Position::from_str("G7").unwrap()).ok();
    knight.set_position(Position::from_str("D7").unwrap()).ok();
    queen.set_position(Position::from_str("D4").unwrap()).ok();

    board.spawn(&bishop).ok();
    board.spawn(&king).ok();
    board.spawn(&knight).ok();
    board.spawn(&queen).ok();

    // board.show_valid_move(&bishop);
    if let Some(piece) = board.get_square_info(Position::from_str("C4").unwrap()) {
        piece.get_mut().move_valid(&board, Position::from_str("D5").unwrap()).ok();
    }
    // bishop.move_valid(&board, Position::from_str("C5").unwrap());

    // let a = board.is_square_under_attack(Position::from_str("C2").unwrap(), Color::Black);
    println!("{board}");

}
