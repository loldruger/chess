use crate::{square::Square, board::Board};

use super::{Color, MoveStatus};

#[derive(Clone, Debug)]
pub struct Rook {
    color: Color,
    coord: Square,
}

impl Rook {
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
        let mut is_king_pierced = false;

        let mut lay = |file, rank, pierce_counter: &mut u32| {
            let position = Square::from_position((file, rank));
            let capture_status = if *pierce_counter > 0 {
                MoveStatus::Pierced { by_color: self.color, activated: false }
            } else {
                MoveStatus::Capturable { by_color: self.color, activated: false }
            };

            if !board.is_empty(position) {
                let query = board.get_piece_mut(position).unwrap();
                let color = query.get_color();

                if color != self.color {
                    let status = MoveStatus::Capturable { by_color: self.color, activated: false };

                    if capture_status == status {
                        if let super::Piece::K(ref mut king) = query {
                            king.set_checked(true);
                            is_king_pierced = true;
                        }
                    }

                    valid_moves.push((position, capture_status));
                } 

                *pierce_counter += 1;
            } else {
                if *pierce_counter > 0 && !is_king_pierced {
                    // capture_status = MoveStatus::None;
                }

                if *pierce_counter < 2 {
                    valid_moves.push((position, capture_status));
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

    pub fn move_to(&mut self, board: &mut Board, coord_to: Square) -> Result<(), &str> {
        board.move_piece(self.coord, coord_to)
    }
}