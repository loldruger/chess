use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

pub struct Rook {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Rook {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        let current_pos = self.get_position();
        
        for i in 0..8 {
            if i != current_pos.get_file() {
                valid_moves.push(Position::from_tuple((i, current_pos.get_rank())));
            }
        }

        // Vertical moves
        for i in 0..8 {
            if i != current_pos.get_rank() {
                valid_moves.push(Position::from_tuple((current_pos.get_file(), i)));
            }
        }

        valid_moves
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

impl ToString for Rook {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♖".to_owned(),
            Color::White => "♜".to_owned(),
        }
    }
}