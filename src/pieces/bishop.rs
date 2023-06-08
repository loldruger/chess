use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Bishop {
    color: Color,
    position: Position,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
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

    fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        if self.get_valid_moves(board).iter().any(|&x| x == position) {
            self.position = position;

            return Ok(());
        }

        Err(())
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();

        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();

        // Top-right to bottom-left diagonal moves
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
