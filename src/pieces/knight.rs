use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Knight(Color);

impl Knight {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Placable for Knight {
    fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        todo!()
        // let mut valid_move = Vec::new();

        // let current_file = self.position.get_file();
        // let current_rank = self.position.get_rank();
    
        // let knight_moves = [
        //     ( 2,  1),
        //     ( 2, -1),
        //     (-2,  1),
        //     (-2, -1),
        //     ( 1,  2),
        //     ( 1, -2),
        //     (-1,  2),
        //     (-1, -2),
        // ];
    
        // for &(file_offset, rank_offset) in &knight_moves {
        //     let target_file = current_file as i32 + file_offset;
        //     let target_rank = current_rank as i32 + rank_offset;
    
        //     if target_file >= 0 && target_file < 8 && target_rank >= 0 && target_rank < 8 {
        //         let position = Square::from_position((target_file, target_rank));
        //         if board.is_empty(position) {
        //             valid_move.push(position);
        //         }
        //     }
        // }
        
        // valid_move
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        todo!()
    }

    fn get_color(&self) -> Color {
        self.0
    }
}
