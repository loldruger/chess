use crate::board::Board;

use super::Position;

pub trait Placable {
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position) -> Result<(), ()>;
    fn get_valid_moves(&self, board: &Board) -> Vec<Position>;
}