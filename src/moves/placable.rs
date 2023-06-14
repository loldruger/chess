use crate::{board::Board, pieces::Color, square::Square};

pub trait Placable {
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Square;
    fn get_valid_moves(&self, board: &mut Board, coord: Square, is_threatening: bool) -> Vec<Square>;
    fn set_position(&mut self, position: Square);
}
