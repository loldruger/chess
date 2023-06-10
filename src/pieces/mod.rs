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

use crate::{moves::{Position, Placable}, board::Board};

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Clone)]
pub enum Piece {
    P(Pawn),
    B(Bishop),
    N(Knight),
    R(Rook),
    Q(Queen),
    K(King)
}

impl Piece {
    pub fn get_piece_mut(&mut self) -> &mut Piece {
        match self {
            p => p
        }
    }
    pub fn set_position(&mut self, position: Position) -> Result<(), ()> {
        match self {
            Piece::P(p) => p.set_position(position),
            Piece::B(p) => p.set_position(position),
            Piece::N(p) => p.set_position(position),
            Piece::R(p) => p.set_position(position),
            Piece::Q(p) => p.set_position(position),
            Piece::K(p) => p.set_position(position),
        }
    }

    pub fn get_position(&self) -> Position {
        match self {
            Piece::P(p) => p.get_position(),
            Piece::B(p) => p.get_position(),
            Piece::N(p) => p.get_position(),
            Piece::R(p) => p.get_position(),
            Piece::Q(p) => p.get_position(),
            Piece::K(p) => p.get_position(),
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

    pub fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        match self {
            Piece::P(p) => p.get_valid_moves(board),
            Piece::B(p) => p.get_valid_moves(board),
            Piece::N(p) => p.get_valid_moves(board),
            Piece::R(p) => p.get_valid_moves(board),
            Piece::Q(p) => p.get_valid_moves(board),
            Piece::K(p) => p.get_valid_moves(board),
        }
    }
    
    pub fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        match self {
            Piece::P(p) => p.move_valid(board, position),
            Piece::B(p) => p.move_valid(board, position),
            Piece::N(p) => p.move_valid(board, position),
            Piece::R(p) => p.move_valid(board, position),
            Piece::Q(p) => p.move_valid(board, position),
            Piece::K(p) => p.move_valid(board, position),
        }
    }
}