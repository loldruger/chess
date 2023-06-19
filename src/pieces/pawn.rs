use crate::{board::Board, square::Square};

use super::{Color, CaptureStatus};

#[derive(Clone, Copy)]
pub struct Pawn {
    color: Color,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &Board, coord_from: Square) -> Vec<(Square, CaptureStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_rank();
        let current_rank = coord_from.get_file();

        // White pawn moves forward by one rank
        match self.color {
            Color::Black => {
                let target_rank = current_rank - 1;

                let is_enemy_piece1 = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece2 = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );

                if board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push((Square::from_position((current_file, target_rank)), CaptureStatus::Captureable));
                }
                // Double move from the starting rank
                if current_rank == 6 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push((Square::from_position((current_file, target_rank - 1)), CaptureStatus::Captureable));
                }
                // Capture diagonally to the left
                if current_file > 0 && is_enemy_piece1 {
                    valid_moves.push((Square::from_position((current_file - 1, target_rank)), CaptureStatus::Captureable));
                }
                // Capture diagonally to the right
                if current_file < 7 && is_enemy_piece2 {
                    valid_moves.push((Square::from_position((current_file + 1, target_rank)), CaptureStatus::Captureable));
                }
            },
            Color::White => {
                let target_rank = current_rank + 1;

                let is_enemy_piece1 = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece2 = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );

                if target_rank <= 7 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push((Square::from_position((current_file, target_rank)), CaptureStatus::Captureable));
                }
                // Double move from the starting rank
                if current_rank == 1 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    valid_moves.push((Square::from_position((current_file, target_rank + 1)), CaptureStatus::Captureable));
                }
                // Capture diagonally to the left
                if current_file > 0 && target_rank <= 7 && is_enemy_piece1 {
                    valid_moves.push((Square::from_position((current_file - 1, target_rank)), CaptureStatus::Captureable));
                }
                // Capture diagonally to the right
                if current_file < 7 && target_rank <= 7 && is_enemy_piece2 {
                    valid_moves.push((Square::from_position((current_file + 1, target_rank)), CaptureStatus::Captureable));
                }
            }
        }

        valid_moves
    }
}