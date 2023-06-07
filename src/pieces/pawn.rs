use crate::{
    moves::{Promotable, Placable, Position}, board::Board,
};

use super::{Color, Rook, Bishop, Knight, Queen};

pub struct Pawn {
    color: Color,
    position: Position,
    is_selected: bool
}

impl Pawn {
    pub fn new(color: Color) -> Self { 
        Self {
            color,
            position: Position::from_tuple((0, 0)),
            is_selected: false
        }
    }

    fn en_passant(&mut self, board: &Board) {

        match self.color {
            Color::Black => todo!(),
            Color::White => todo!(),
        }
    }
}

impl Placable for Pawn {
    fn set_position(&mut self, position: Position) -> Result<(), ()> {
        self.position = position;
        Ok(())
    }

    fn move_valid(&mut self, board: &Board, position: Position) -> Result<(), ()> {
        todo!()
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();

        let current_file = self.position.get_file();
        let current_rank = self.position.get_rank();

        // White pawn moves forward by one rank
        match self.color {
            Color::White => {
                let target_rank = current_rank - 1;
                if target_rank >= 0 && board.is_empty(Position::from_tuple((current_file, target_rank))) {
                    valid_moves.push(Position::from_tuple((current_file, target_rank)));
                }
                // Double move from the starting rank
                if current_rank == 6 && target_rank >= 1 && board.is_empty(Position::from_tuple((current_file, target_rank))) {
                    valid_moves.push(Position::from_tuple((current_file, target_rank - 1)));
                }
                // Capture diagonally to the left
                if current_file > 0 && target_rank >= 0 && board.is_enemy_piece(Position::from_tuple((current_file - 1, target_rank)), self.color) {
                    valid_moves.push(Position::from_tuple((current_file - 1, target_rank)));
                }
                // Capture diagonally to the right
                if current_file < 7 && target_rank >= 0 && board.is_enemy_piece(Position::from_tuple((current_file + 1, target_rank)), self.color) {
                    valid_moves.push(Position::from_tuple((current_file + 1, target_rank)));
                }
            },
            Color::Black => {
                let target_rank = current_rank + 1;
                if target_rank <= 7 && board.is_empty(Position::from_tuple((current_file, target_rank))) {
                    valid_moves.push(Position::from_tuple((current_file, target_rank)));
                }
                // Double move from the starting rank
                if current_rank == 1 && target_rank <= 6 && board.is_empty(Position::from_tuple((current_file, target_rank))) {
                    valid_moves.push(Position::from_tuple((current_file, target_rank + 1)));
                }
                // Capture diagonally to the left
                if current_file > 0 && target_rank <= 7 && board.is_enemy_piece(Position::from_tuple((current_file - 1, target_rank)), self.color) {
                    valid_moves.push(Position::from_tuple((current_file - 1, target_rank)));
                }
                // Capture diagonally to the right
                if current_file < 7 && target_rank <= 7 && board.is_enemy_piece(Position::from_tuple((current_file + 1, target_rank)), self.color) {
                    valid_moves.push(Position::from_tuple((current_file + 1, target_rank)));
                }
            }
        }

        valid_moves
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }

    
}

impl Promotable for Pawn {
    fn into_rook(self) -> Rook {
        let mut piece = Rook::new(self.color);
        piece.set_position(self.position).unwrap();

        piece
    }

    fn into_bishop(self) -> Bishop {
        let mut piece = Bishop::new(self.color);
        piece.set_position(self.position).unwrap();
        
        piece
    }

    fn into_knight(self) -> Knight {
        let mut piece = Knight::new(self.color);
        piece.set_position(self.position).unwrap();
        
        piece
    }

    fn into_queen(self) -> Queen {
        let mut piece = Queen::new(self.color);
        piece.set_position(self.position).unwrap();
        
        piece
    }
}

impl ToString for Pawn {
    fn to_string(&self) -> String {
        match self.color {
            Color::Black => "♙".to_owned(),
            Color::White => "♟".to_owned(),
        }
    }
}