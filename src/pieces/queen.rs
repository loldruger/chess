use crate::{board::Board, square::Square};

use super::{Color, MoveStatus};

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
    coord: Square,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            coord: Square::None,
        }
    }

    pub fn set_coord(&mut self, coord: Square) {
        self.coord = coord;
    }

    pub fn get_coord(&self) -> Square {
        self.coord
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &mut Board, coord_from: Square) -> Vec<(Square, MoveStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_file();
        let current_rank = coord_from.get_rank();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut e = 0;
        let mut f = 0;
        let mut g = 0;
        let mut h = 0;
        let mut is_king_pierced = false;

        let mut lay = |file, rank, pierce_counter: &mut u32| {
            let position = Square::from_position((rank, file));
            let mut capture_status = if *pierce_counter > 0 {
                MoveStatus::Pierced
            } else {
                MoveStatus::Capturable
            };

            if !board.is_empty(position) {
                let query = board.get_piece_mut(position).unwrap();
                let color = query.get_color();
                
                if color != self.color {
                    if let super::Piece::K(ref mut king) = *query {
                        king.set_checked(true);
                        is_king_pierced = true;
                    }
                    valid_moves.push((position, capture_status));
                }

                *pierce_counter += 1;
            } else {
                if *pierce_counter > 0 && !is_king_pierced {
                    capture_status = MoveStatus::Stuck;
                }

                if *pierce_counter < 2 {
                    valid_moves.push((position, capture_status));
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

        for file in (current_file + 1)..8 {
            lay(file, current_rank, &mut e);
        }
    
        // Horizontal moves to the left
        for file in (0..current_file).rev() {
            lay(file, current_rank, &mut f);
        }
    
        // Vertical moves upwards
        for rank in (current_rank + 1)..8 {
            lay(current_file, rank, &mut g);
        }
    
        // Vertical moves downwards
        for rank in (0..current_rank).rev() {
            lay(current_file, rank, &mut h);
        }


        valid_moves
    }

}