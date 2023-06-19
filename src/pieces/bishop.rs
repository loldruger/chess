use crate::{square::Square, board::Board};

use super::Color;

#[derive(Clone, Copy)]
pub struct Bishop {
    color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &Board, coord_from: Square) -> Vec<(Square, bool)> {
        let mut valid_moves = Vec::new();
        let current_file = coord_from.get_file();
        let current_rank = coord_from.get_rank();
        
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        let mut lay = |file, rank, pierce_counter: &mut u32| {
            let position = Square::from_position((rank, file));

            if !board.is_empty(position) {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                if color != self.color {
                    valid_moves.push((position, *pierce_counter > 0));

                    if let super::Piece::K(mut king) = query {
                        king.set_checked(true);
                    }
                }
                *pierce_counter += 1;
            } else {
                if *pierce_counter < 2 {
                    valid_moves.push((position, *pierce_counter > 0));
                }
            }
        };

        for i in 1..=i32::min(current_file, 7 - current_rank) {
            lay(current_file - i, current_rank + i, &mut a);
        }

        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            lay(current_file + i, current_rank + i, &mut b);
        }

        for i in 1..=i32::min(7 - current_file, current_rank) {
            lay(current_file + i, current_rank - i, &mut c);
        }

        for i in 1..=i32::min(current_file, current_rank) {
            lay(current_file - i, current_rank - i, &mut d);
        }

        valid_moves
    }
}