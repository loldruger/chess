use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Queen {
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

        for i in 1..=i32::min(current_file, 7 - current_rank) {
            let position = Position::from_tuple((current_file - i, current_rank + i));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            let position = Position::from_tuple((current_file + i, current_rank + i));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            let position = Position::from_tuple((current_file + i, current_rank - i));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            let position = Position::from_tuple((current_file - i, current_rank - i));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
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
