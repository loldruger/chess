mod board;
mod pieces;
mod moves;

use board::Board;
use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use moves::{Placable, Position};

use crate::pieces::{Knight, Piece};

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

    board.spawn(Piece::B(bishop)).ok();
    board.spawn(Piece::K(king)).ok();
    board.spawn(Piece::N(knight)).ok();
    board.spawn(Piece::Q(queen)).ok();

    println!("{board}");
    
    let a = board.get_piece_mut(Position::from_str("C4").unwrap()).unwrap();
    // a.move_valid(&board, Position::from_str("C5").unwrap());
    a.set_position(Position::from_str("A1").unwrap()).ok();

    println!("{board}");

}
