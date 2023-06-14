use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
    coord: Square,
    is_threatened: bool,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            coord: Square::None,
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

impl Placable for Queen {
    fn set_position(&mut self, position: Square) {
        self.coord = position;
    }

    fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threatened: bool) -> Vec<Square> {
        let mut valid_moves = Vec::new();
        
        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;
        
        for file in (current_file + 1)..8 {
            let position = Square::from_position((file, current_rank));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }
    
        // Horizontal moves to the left
        for file in (0..current_file).rev() {
            let position = Square::from_position((file, current_rank));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves upwards
        for rank in (current_rank + 1)..8 {
            let position = Square::from_position((current_file, rank));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }
    
        // Vertical moves downwards
        for rank in (0..current_rank).rev() {
            let position = Square::from_position((current_file, rank));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Top-right to bottom-left diagonal moves
        for i in 1..=i32::min(current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file - i), (current_rank + i)));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Top-left to bottom-right diagonal moves
        for i in 1..=i32::min(7 - current_file, 7 - current_rank) {
            let position = Square::from_position(((current_file + i), (current_rank + i)));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Bottom-left to top-right diagonal moves
        for i in 1..=i32::min(7 - current_file, current_rank) {
            let position = Square::from_position(((current_file + i), (current_rank - i)));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        // Bottom-right to top-left diagonal moves
        for i in 1..=i32::min(current_file, current_rank) {
            let position = Square::from_position(((current_file - i), (current_rank - i)));
            if !board.is_empty(position) && !is_threatened {
                let query = board.get_piece(position).unwrap();
                let color = query.get_color();

                board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                break;
            }
            valid_moves.push(position);
        }

        valid_moves
    }

    fn get_position(&self) -> Square {
        self.coord
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
