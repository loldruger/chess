use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Bishop(Color);

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self(color)
    }

    pub fn get_color(&self) -> Color {
        self.0
    }
}

impl Placable for Bishop {
    fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        let mut valid_moves = Vec::new();

        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;

        // Top-right to bottom-left diagonal moves
        for i in 1..=i32::min(current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file - i) as usize, (current_rank + i) as usize));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file + i) as usize, (current_rank + i) as usize));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            let position = Square::from_position(((current_file + i) as usize, (current_rank - i) as usize));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            let position = Square::from_position(((current_file - i) as usize, (current_rank - i) as usize));
            if board.is_empty(position) {
                valid_moves.push(position);
            } else {
                break;
            }
        }


        valid_moves
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        todo!()
    }

    fn get_color(&self) -> Color {
        self.0
    }


}
