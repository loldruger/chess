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
        todo!()
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        todo!()
    }

    fn get_position(&self) -> Position {
        self.position
    }
}
