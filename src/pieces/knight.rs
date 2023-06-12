use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Knight {
    color: Color,
    is_threatened: bool,
}
impl Knight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            is_threatened: false,
        }
    }

    pub fn is_threatened(&self) -> bool {
        self.is_threatened
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }
}

impl Placable for Knight {
    fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threatening: bool) -> Vec<Square> {
        let mut valid_move = Vec::new();

        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;

        let knight_moves = [
            ( 2,  1), ( 2, -1), (-2,  1), (-2, -1),
            ( 1,  2), ( 1, -2), (-1,  2), (-1, -2),
        ];
    
        for &(file_offset, rank_offset) in &knight_moves {
            let target_file = current_file as i32 + file_offset;
            let target_rank = current_rank as i32 + rank_offset;
    
            if target_file >= 0 && target_file < 8 && target_rank >= 0 && target_rank < 8 {
                let position = Square::from_position((target_file as usize, target_rank as usize));
                if board.is_empty(position) {
                    valid_move.push(position);
                } else {
                    let query = board.get_piece(position).unwrap();
                    let color = query.get_color();

                    board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                }
            }
        }
        
        valid_move
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        todo!()
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
