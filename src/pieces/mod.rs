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

#[derive(Clone, Copy, PartialEq)]
pub enum MoveStatus {
    None,
    Capturable { by_color: Color, activated: bool },
    Threaten { by_color: Color, activated: bool },
    Pierced  { by_color: Color, activated: bool },
    EnPassant { by_color: Color, activated: bool },
    Castling { by_color: Color, activated: bool },
    Movable { by_color: Color, activated: bool },
}

#[derive(Clone, Debug)]
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

    pub fn get_coord(&self) -> Square {
        match self {
            Piece::P(p) => p.get_coord(),
            Piece::B(p) => p.get_coord(),
            Piece::N(p) => p.get_coord(),
            Piece::R(p) => p.get_coord(),
            Piece::Q(p) => p.get_coord(),
            Piece::K(p) => p.get_coord(),
        }
    }

    pub fn set_coord(&mut self, coord: Square) {
        match self {
            Piece::P(p) => p.set_coord(coord),
            Piece::B(p) => p.set_coord(coord),
            Piece::N(p) => p.set_coord(coord),
            Piece::R(p) => p.set_coord(coord),
            Piece::Q(p) => p.set_coord(coord),
            Piece::K(p) => p.set_coord(coord),
        }
    }

    pub fn move_to(&mut self, board: &mut Board, coord_to: Square) -> Result<(), &str> {
        match self {
            Piece::P(p) => p.move_to(board, coord_to),
            Piece::B(p) => p.move_to(board, coord_to),
            Piece::N(p) => p.move_to(board, coord_to),
            Piece::R(p) => p.move_to(board, coord_to),
            Piece::Q(p) => p.move_to(board, coord_to),
            Piece::K(p) => p.move_to(board, coord_to),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}