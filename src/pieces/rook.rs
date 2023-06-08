use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

#[derive(Clone, Copy)]
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

    fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        todo!()
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();
        
        for file in (current_file + 1)..8 {
            let position = Position::from_tuple((file, current_rank));
            if !board.is_empty(position) {
                break;
            }
            valid_moves.push(position);
        }
    
        // Horizontal moves to the left
        for file in (0..current_file).rev() {
            let position = Position::from_tuple((file, current_rank));
            if !board.is_empty(position) {
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves upwards
        for rank in (current_rank + 1)..8 {
            let position = Position::from_tuple((current_file, rank));
            if !board.is_empty(position) {
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves downwards
        for rank in (0..current_rank).rev() {
            let position = Position::from_tuple((current_file, rank));
            if !board.is_empty(position) {
                break;
            }
            valid_moves.push(position);
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
