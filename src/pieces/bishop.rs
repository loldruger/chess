use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Bishop {
    color: Color,
    is_threatened: bool,
}
impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            is_threatened: false,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn is_threatened(&self) -> bool {
        self.is_threatened
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }
}

impl Placable for Bishop {
    fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threatening: bool) -> Vec<Square> {
        let mut valid_moves = Vec::new();

        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;

        // Top-right to bottom-left diagonal moves
        for i in 1..=i32::min(current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file - i) as usize, (current_rank + i) as usize));
            if !board.is_empty(position) && !is_threatening {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file + i) as usize, (current_rank + i) as usize));
            if !board.is_empty(position) && !is_threatening {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            let position = Square::from_position(((current_file + i) as usize, (current_rank - i) as usize));
            if !board.is_empty(position) && !is_threatening {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            let position = Square::from_position(((current_file - i) as usize, (current_rank - i) as usize));
            if !board.is_empty(position) && !is_threatening {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }


        valid_moves
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        todo!()
    }

    fn get_color(&self) -> Color {
        self.color
    }


}
