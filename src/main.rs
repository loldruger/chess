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

    if let Some(piece) = board.get_square_info(Position::from_str("C4").unwrap()) {
        piece.move_valid(&board, Position::from_str("D5").unwrap()).ok();
    }
// ------^^^^------ cannot borrow `***piece` as mutable, as it is behind a `&` reference
// cannot borrow as mutable
    println!("{board}");

}
