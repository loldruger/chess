use crate::{board::Board, square::Square};

use super::Color;

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

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &mut Board, coord_from: Square) -> Vec<(Square, bool)> {
        let mut valid_moves = Vec::new();

        

        valid_moves
    }

    pub fn get_coord(&self) -> Square {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Square) {
        self.coord = coord;
    }
}