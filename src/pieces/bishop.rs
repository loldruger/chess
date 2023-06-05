use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

pub struct Bishop {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Bishop {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        // if let Some(x) = self.get_valid_moves()
        //     .iter()
        //     .find(|x| {
        //         **x == self.get_position()
        //     }) {
        //         self.position = *x;
        //         Ok(())
        //     } else {
        //         Err(())
        //     }
        self.position = position;
        Ok(())
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();

        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();

        // Top-right to bottom-left diagonal moves
        for i in 1..=i32::min(current_file, 7 - current_rank) {
            valid_moves.push(Position::from_tuple((current_file - i, current_rank + i)));
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            valid_moves.push(Position::from_tuple((current_file + i, current_rank + i)));
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            valid_moves.push(Position::from_tuple((current_file + i, current_rank - i)));
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            valid_moves.push(Position::from_tuple((current_file - i, current_rank - i)));
        }


        valid_moves
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

impl ToString for Bishop {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♗".to_owned(),
            Color::White => "♝".to_owned(),
        }
    }
}