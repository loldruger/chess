use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
    position: Position,
    is_selected: bool,
    is_checked: bool
}

impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false,
            is_checked: false,
        }
    }
}

impl Placable for King {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        todo!()
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_move = Vec::new();

        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();

        let direction = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];
        
        for (file, rank) in direction {
            let dest_file = current_file + file;
            let dest_rank = current_rank + rank;

            if dest_file >= 0 && dest_file < 8 && dest_rank >= 0 && dest_rank < 8 {
                if !board.is_square_under_attack(Position::from_tuple((dest_file, dest_rank)), self.color) {
                    valid_move.push(Position::from_tuple((dest_file, dest_rank)));
                }
            }
        }

        valid_move
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

