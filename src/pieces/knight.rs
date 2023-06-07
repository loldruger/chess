use crate::{
    moves::{Placable, Position}, board::Board,
};

use super::Color;

pub struct Knight {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }
}

impl Placable for Knight {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        todo!()
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_move = Vec::new();

        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();
    
        let knight_moves = [
            ( 2,  1),
            ( 2, -1),
            (-2,  1),
            (-2, -1),
            ( 1,  2),
            ( 1, -2),
            (-1,  2),
            (-1, -2),
        ];
    
        for &(file_offset, rank_offset) in &knight_moves {
            let target_file = current_file as i32 + file_offset;
            let target_rank = current_rank as i32 + rank_offset;
    
            if target_file >= 0 && target_file < 8 && target_rank >= 0 && target_rank < 8 {
                let position = Position::from_tuple((target_file, target_rank));
                if board.is_empty(position) {
                    valid_move.push(position);
                }
            }
        }
        
        valid_move
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

impl ToString for Knight {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♘".to_owned(),
            Color::White => "♞".to_owned(),
        }
    }
}