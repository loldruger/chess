use std::collections::HashSet;

use crate::{square::{Square, SquareKind, SquareStatus}, board::Board};

use super::Color;

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
    coord: Square,
    is_checked: bool,
}

impl King {
    pub fn new(color: Color) -> King {
        King {
            color,
            coord: Square::None,
            is_checked: false,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub fn set_checked(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &Board, coord_from: Square) -> Vec<(Square, bool)> {
        let mut valid_moves = Vec::new();
        let current_file = coord_from.get_rank();
        let current_rank = coord_from.get_file();

        let direction = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];

        for (file, rank) in direction {
            let dest_file = current_file + file;
            let dest_rank = current_rank + rank;

            if dest_file >= 0 && dest_file < 8 && dest_rank >= 0 && dest_rank < 8 {
                let position = Square::from_position((dest_file, dest_rank));
                if board.is_empty(position) &&
                    !board.is_vulnerable(position, self.color.opposite()) && 
                    !board.is_under_attack(position, self.color.opposite())
                    {
                    valid_moves.push((position, false));
                }
            }
        }
        
        let opponent_move = board
            .get_valid_moves_all(self.color.opposite())
            .iter()
            .map(|x| x.0)
            .collect::<Vec<Square>>();

        valid_moves.retain(|x| !opponent_move.contains(&x.0));

        valid_moves
    }

    pub fn get_coord(&self) -> Square {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Square) {
        self.coord = coord;
    }
}