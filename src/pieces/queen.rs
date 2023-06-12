use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
    is_threatened: bool,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            is_threatened: false,
        }
    }

    pub fn is_threatened(&self) -> bool {
        self.is_threatened
    }

    pub fn set_vulnerable(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }
}

impl Placable for Queen {
    fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        todo!()
        // let mut valid_moves = Vec::new();
        // let current_file = self.position.get_file();
        // let current_rank = self.position.get_rank();
        
        // for file in (current_file + 1)..8 {
        //     let position = Square::from_position((file, current_rank));
        //     if !board.is_empty(position) {
        //         break;
        //     }
        //     valid_moves.push(position);
        // }
    
        // // Horizontal moves to the left
        // for file in (0..current_file).rev() {
        //     let position = Square::from_position((file, current_rank));
        //     if !board.is_empty(position) {
        //         break;
        //     }
        //     valid_moves.push(position);
        // }
    
        // // Vertical moves upwards
        // for rank in (current_rank + 1)..8 {
        //     let position = Square::from_position((current_file, rank));
        //     if !board.is_empty(position) {
        //         break;
        //     }
        //     valid_moves.push(position);
        // }
    
        // // Vertical moves downwards
        // for rank in (0..current_rank).rev() {
        //     let position = Square::from_position((current_file, rank));
        //     if !board.is_empty(position) {
        //         break;
        //     }
        //     valid_moves.push(position);
        // }

        // for i in 1..=i32::min(current_file, 7 - current_rank) {
        //     let position = Square::from_position((current_file - i, current_rank + i));
        //     if board.is_empty(position) {
        //         valid_moves.push(position);
        //     } else {
        //         break;
        //     }
        // }

        // // Top-left to bottom-right diagonal moves
        // for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
        //     let position = Square::from_position((current_file + i, current_rank + i));
        //     if board.is_empty(position) {
        //         valid_moves.push(position);
        //     } else {
        //         break;
        //     }
        // }

        // // Bottom-left to top-right diagonal moves
        // for i in 1..=i32::min(7 - current_file, current_rank) {
        //     let position = Square::from_position((current_file + i, current_rank - i));
        //     if board.is_empty(position) {
        //         valid_moves.push(position);
        //     } else {
        //         break;
        //     }
        // }

        // // Bottom-right to top-left diagonal moves
        // for i in 1..=i32::min(current_file, current_rank) {
        //     let position = Square::from_position((current_file - i, current_rank - i));
        //     if board.is_empty(position) {
        //         valid_moves.push(position);
        //     } else {
        //         break;
        //     }
        // }

        // valid_moves
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        // self.position
        todo!()
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
