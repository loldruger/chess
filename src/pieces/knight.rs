use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

pub struct Knight {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Knight {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let valid_move = Vec::new();

        
        
        valid_move
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

impl ToString for Knight {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♘".to_owned(),
            Color::White => "♞".to_owned(),
        }
    }
}