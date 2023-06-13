use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
    is_threatened: bool,
}
impl King {
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

impl Placable for King {
    fn get_valid_moves(&self, board: &mut Board, coord: Square, _: bool) -> Vec<Square> {
        let mut valid_move = Vec::new();

        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;

        let direction = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];
        
        for (file, rank) in direction {
            let dest_file = current_file + file;
            let dest_rank = current_rank + rank;

            if dest_file >= 0 && dest_file < 8 && dest_rank >= 0 && dest_rank < 8 {
                if !board.is_threatened(Square::from_position((dest_file, dest_rank)), self.color) {
                    valid_move.push(Square::from_position((dest_file, dest_rank)));
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

