use crate::{board::Board, square::Square};

use super::Color;

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
}

impl Queen {
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

        

        valid_moves
    }

}