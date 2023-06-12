use crate::{
    moves::{Placable}, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Rook(Color);

impl Rook {
    pub fn new(color: Color) -> Self {
        Self(color)
    }

    pub fn get_color(&self) -> Color {
        self.0
    }
}

impl Placable for Rook {
    fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        let mut valid_moves = Vec::new();
        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as usize;
        let current_rank = current_rank as usize;
        
        for file in (current_file + 1)..8 {
            let position = Square::from_position((file, current_rank));
            if !board.is_empty(position) && !is_threaten {
                break;
            }
            valid_moves.push(position);
        }
    
        // Horizontal moves to the left
        for file in (0..current_file).rev() {
            let position = Square::from_position((file, current_rank));
            if !board.is_empty(position) && !is_threaten {
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves upwards
        for rank in (current_rank + 1)..8 {
            let position = Square::from_position((current_file, rank));
            if !board.is_empty(position) && !is_threaten {
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves downwards
        for rank in (0..current_rank).rev() {
            let position = Square::from_position((current_file, rank));
            if !board.is_empty(position) && !is_threaten {
                break;
            }
            valid_moves.push(position);
        }

        valid_moves
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        // self.position
        todo!()
    }

    fn get_color(&self) -> Color {
        self.0
    }
}
