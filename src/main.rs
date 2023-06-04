mod board;
mod pieces;
mod moves;

use board::Board;
use pieces::{Color, Pawn, Rook, Bishop};
use moves::{Placable, Position};

fn main() {
    let mut board = Board::new();

    let mut pawn = Pawn::new(Color::White);
    let mut rook = Rook::new(Color::Black);
    let mut bishop = Bishop::new(Color::White);

    rook.set_position(Position::from_str("B4").unwrap()).unwrap();
    pawn.set_position(Position::from_str("A3").unwrap()).unwrap();
    bishop.set_position(Position::from_str("C3").unwrap()).unwrap();

    board.spawn(&pawn).ok();
    board.spawn(&rook).ok();
    board.show_valid_move(&pawn);

    let a = pawn.get_valid_moves(&board);

    a;

}
