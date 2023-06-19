use crate::{square::Square, board::Board};

use super::Color;

#[derive(Clone, Copy)]
pub struct Rook {
    color: Color,
}

impl Rook {
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
                    if let super::Piece::K(mut king) = query {
                        king.set_checked(true);
                    }
                    valid_moves.push((position, *pierce_counter > 0));

                }
                *pierce_counter += 1;

            } else {
                if *pierce_counter < 2 {
                    valid_moves.push((position, *pierce_counter > 0));
                }
            }
        };

        for file in (current_file + 1)..8 {
            lay(file, current_rank, &mut a);
        }
    
        // Horizontal moves to the left
        for file in (0..current_file).rev() {
            lay(file, current_rank, &mut b);
        }
    
        // Vertical moves upwards
        for rank in (current_rank + 1)..8 {
            lay(current_file, rank, &mut c);
        }
    
        // Vertical moves downwards
        for rank in (0..current_rank).rev() {
            lay(current_file, rank, &mut d);
        }

        valid_moves
    }
}