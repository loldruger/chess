mod board;
mod pieces;
mod moves;

use board::Board;
use pieces::{Color, Pawn, Rook, Bishop, Queen};
use moves::{Placable, Position};

fn main() {
    let mut board = Board::new();

    let mut pawn = Pawn::new(Color::White);
    let mut rook = Rook::new(Color::Black);
    let mut bishop = Bishop::new(Color::White);
    let mut queen = Queen::new(Color::Black);

    rook.set_position(Position::from_str("B4").unwrap()).unwrap();
    pawn.set_position(Position::from_str("A2").unwrap()).unwrap();
    bishop.set_position(Position::from_str("C3").unwrap()).unwrap();
    queen.set_position(Position::from_str("D5").unwrap()).unwrap();

    board.spawn(&pawn).ok();
    board.spawn(&rook).ok();
    board.spawn(&queen).ok();

    board.show_valid_move(&queen);

    println!("{}", board);

}
