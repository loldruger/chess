pub mod pawn;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod queen;
pub mod king;

pub use pawn::Pawn;
pub use rook::Rook;
pub use bishop::Bishop;
pub use knight::Knight;
pub use queen::Queen;
pub use king::King;

use crate::moves::Position;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy)]
pub enum Piece {
    Pawn { color: Color, pos: Position },
    Bishop { color: Color, pos: Position },
    Knight { color: Color, pos: Position },
    Rook { color: Color, pos: Position },
    Queen { color: Color, pos: Position },
    King { color: Color, pos: Position }
}