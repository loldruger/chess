mod board;
mod pieces;
mod moves;

use std::ops::Deref;

use board::Board;
use pieces::{Color, Pawn, Rook, Bishop, Queen, King};
use moves::{Placable, Position};

fn main() {
    let mut board = Board::new();

    let mut pawn = Pawn::new(Color::White);
    let mut rook = Rook::new(Color::Black);
    let mut bishop = Bishop::new(Color::White);
    let mut queen = Queen::new(Color::Black);
    let mut king = King::new(Color::Black);

    // board.spawn_at(&mut pawn, Position::from_str("D4").unwrap()).ok();
    // board.spawn_at(&mut rook, Position::from_str("B7").unwrap()).ok();
    // board.spawn_at(&mut queen, Position::from_str("B4").unwrap()).ok();
    board.spawn_at(&mut bishop, Position::from_str("B2").unwrap()).ok();
    board.spawn_at(&mut king, Position::from_str("H8").unwrap()).ok();

    // let a = board.is_square_under_attack(Position::from_str("C2").unwrap(), Color::Black);
    println!("{board}");

}
