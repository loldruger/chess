use crate::{board::Board, pieces::Color};

use super::Position;

pub trait Placable {
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position) -> Result<(), ()>;
    fn get_valid_moves(&self, board: &Board) -> Vec<Position>;
}