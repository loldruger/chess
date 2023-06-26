use crate::{board::Board, square::Square};

use super::{Color, MoveStatus};

#[derive(Clone, Copy)]
pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &mut Board, coord_from: Square) -> Vec<(Square, MoveStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_file();
        let current_rank = coord_from.get_rank();
    
        let knight_moves = [
            ( 2,  1), ( 2, -1), (-2,  1), (-2, -1),
            ( 1,  2), ( 1, -2), (-1,  2), (-1, -2),
        ];
    
        for &(file_offset, rank_offset) in &knight_moves {
            let target_file = current_file + file_offset;
            let target_rank = current_rank + rank_offset;
    
            if target_file >= 0 && target_file < 8 && target_rank >= 0 && target_rank < 8 {
                let position = Square::from_position((target_rank, target_file));
                
                if !board.is_empty(position) {
                    let query = board.get_piece_mut(position).unwrap();
                    let color = query.get_color();
    
                    if color != self.color {
                        valid_moves.push((position, MoveStatus::Capturable));

                        if let super::Piece::K(ref mut king) = query {
                            king.set_checked(true);
                        }
                    }
                } else {
                    valid_moves.push((position, MoveStatus::Capturable));
                }
            }
        }
    
        valid_moves
    }
}