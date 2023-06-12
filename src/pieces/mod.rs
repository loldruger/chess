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
    Both,
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
    pub fn get_bishop(&self) -> Option<Bishop> {
        match self {
            Piece::B(p) => Some(*p),
            _ => None
        }
    }

    pub fn get_placable_mut(&mut self) -> &mut dyn Placable {
        match self {
            Piece::P(p) => p,
            Piece::B(p) => p,
            Piece::N(p) => p,
            Piece::R(p) => p,
            Piece::Q(p) => p,
            Piece::K(p) => p,
        }
    }
    pub fn get_piece_mut(&mut self) -> &mut Piece {
        match self {
            p => p
        }
    }

    pub fn get_position(&self, board: &Board) -> Option<Square> {
        match self {
            Piece::P(p) => p.get_position(board),
            Piece::B(p) => p.get_position(board),
            Piece::N(p) => p.get_position(board),
            Piece::R(p) => p.get_position(board),
            Piece::Q(p) => p.get_position(board),
            Piece::K(p) => p.get_position(board),
        }
    }

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

    pub fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        match self {
            Piece::P(p) => p.get_valid_moves(board, coord, is_threaten),
            Piece::B(p) => p.get_valid_moves(board, coord, is_threaten),
            Piece::N(p) => p.get_valid_moves(board, coord, is_threaten),
            Piece::R(p) => p.get_valid_moves(board, coord, is_threaten),
            Piece::Q(p) => p.get_valid_moves(board, coord, is_threaten),
            Piece::K(p) => p.get_valid_moves(board, coord, is_threaten),
        }
    }
}