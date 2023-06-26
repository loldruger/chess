mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;

pub use pawn::Pawn;
pub use knight::Knight;
pub use bishop::Bishop;
pub use rook::Rook;
pub use queen::Queen;
pub use king::King;

use crate::{board::Board, square::Square};

#[derive(PartialEq)]
pub enum MoveStatus {
    Capturable,
    Pierced,
    UnCapturable,
    EnPassant,
    Castling,
    Movable
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Piece {
    P(Pawn),
    N(Knight),
    B(Bishop),
    R(Rook),
    Q(Queen),
    K(King),
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::P(p) => p.get_color(),
            Piece::B(p) => p.get_color(),
            Piece::N(p) => p.get_color(),
            Piece::R(p) => p.get_color(),
            Piece::Q(p) => p.get_color(),
            Piece::K(p) => p.get_color(),
        }
    }

    pub fn get_valid_moves(&self, board: &mut Board, coord: Square) -> Vec<(Square, MoveStatus)> {
        match self {
            Piece::P(p) => p.get_valid_moves(board, coord),
            Piece::B(p) => p.get_valid_moves(board, coord),
            Piece::N(p) => p.get_valid_moves(board, coord),
            Piece::R(p) => p.get_valid_moves(board, coord),
            Piece::Q(p) => p.get_valid_moves(board, coord),
            Piece::K(p) => p.get_valid_moves(board, coord),
        }
    }
}

