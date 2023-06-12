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

use crate::{moves::Placable, board::Board, square::Square};

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White
}



#[derive(Clone, Copy)]
pub enum Piece {
    P(Pawn),
    B(Bishop),
    N(Knight),
    R(Rook),
    Q(Queen),
    K(King)
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

    pub fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threatened: bool) -> Vec<Square> {
        match self {
            Piece::P(p) => p.get_valid_moves(board, coord, is_threatened),
            Piece::B(p) => p.get_valid_moves(board, coord, is_threatened),
            Piece::N(p) => p.get_valid_moves(board, coord, is_threatened),
            Piece::R(p) => p.get_valid_moves(board, coord, is_threatened),
            Piece::Q(p) => p.get_valid_moves(board, coord, is_threatened),
            Piece::K(p) => p.get_valid_moves(board, coord, is_threatened),
        }
    }

    pub fn is_threatened(&self) -> bool {
        match self {
            Piece::P(p) => p.is_threatened(),
            Piece::B(p) => p.is_threatened(),
            Piece::N(p) => p.is_threatened(),
            Piece::R(p) => p.is_threatened(),
            Piece::Q(p) => p.is_threatened(),
            Piece::K(p) => p.is_threatened(),
        }
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        match self {
            Piece::P(p) => p.set_threatened(is_threatened),
            Piece::B(p) => p.set_threatened(is_threatened),
            Piece::N(p) => p.set_threatened(is_threatened),
            Piece::R(p) => p.set_threatened(is_threatened),
            Piece::Q(p) => p.set_threatened(is_threatened),
            Piece::K(p) => p.set_threatened(is_threatened),
        }
    }
}