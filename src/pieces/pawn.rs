use crate::{
    moves::{Promotable, Placable}, board::Board, square::Square,
};

use super::{Color, Rook, Bishop, Knight, Queen};

#[derive(Clone, Copy)]
pub struct Pawn(Color);

impl Pawn {
    pub fn new(color: Color) -> Self { 
        Self(color)
    }

    fn en_passant(&mut self, board: &Board) {

        match self.0 {
            Color::Black => todo!(),
            Color::White => todo!(),
            _ => ()
        }
    }
}

impl Placable for Pawn {
    fn get_valid_moves(&self, board: &Board, coord: Square, is_threaten: bool) -> Vec<Square> {
        todo!()
        // let mut valid_moves = Vec::new();

        // let current_file = self.position.get_file();
        // let current_rank = self.position.get_rank();

        // // White pawn moves forward by one rank
        // match self.color {
        //     Color::White => {
        //         let target_rank = current_rank - 1;
        //         if target_rank >= 0 && board.is_empty(Square::from_position((current_file, target_rank))) {
        //             valid_moves.push(Square::from_position((current_file, target_rank)));
        //         }
        //         // Double move from the starting rank
        //         if current_rank == 6 && target_rank >= 1 && board.is_empty(Square::from_position((current_file, target_rank))) {
        //             valid_moves.push(Square::from_position((current_file, target_rank - 1)));
        //         }
        //         // Capture diagonally to the left
        //         if current_file > 0 && target_rank >= 0 && board.is_enemy_piece(Square::from_position((current_file - 1, target_rank)), self.color) {
        //             valid_moves.push(Square::from_position((current_file - 1, target_rank)));
        //         }
        //         // Capture diagonally to the right
        //         if current_file < 7 && target_rank >= 0 && board.is_enemy_piece(Square::from_position((current_file + 1, target_rank)), self.color) {
        //             valid_moves.push(Square::from_position((current_file + 1, target_rank)));
        //         }
        //     },
        //     Color::Black => {
        //         let target_rank = current_rank + 1;
        //         if target_rank <= 7 && board.is_empty(Square::from_position((current_file, target_rank))) {
        //             valid_moves.push(Square::from_position((current_file, target_rank)));
        //         }
        //         // Double move from the starting rank
        //         if current_rank == 1 && target_rank <= 6 && board.is_empty(Square::from_position((current_file, target_rank))) {
        //             valid_moves.push(Square::from_position((current_file, target_rank + 1)));
        //         }
        //         // Capture diagonally to the left
        //         if current_file > 0 && target_rank <= 7 && board.is_enemy_piece(Square::from_position((current_file - 1, target_rank)), self.color) {
        //             valid_moves.push(Square::from_position((current_file - 1, target_rank)));
        //         }
        //         // Capture diagonally to the right
        //         if current_file < 7 && target_rank <= 7 && board.is_enemy_piece(Square::from_position((current_file + 1, target_rank)), self.color) {
        //             valid_moves.push(Square::from_position((current_file + 1, target_rank)));
        //         }
        //     }
        // }

        // valid_moves
    }

    fn get_position(&self, board: &Board) -> Option<Square> {
        todo!()
    }

    fn get_color(&self) -> Color {
        self.0
    }

    
}

impl Promotable for Pawn {
    fn into_rook(self) -> Rook {
        Rook::new(self.0)
    }

    fn into_bishop(self) -> Bishop {
        Bishop::new(self.0)
    }

    fn into_knight(self) -> Knight {
        Knight::new(self.0)
    }

    fn into_queen(self) -> Queen {
        Queen::new(self.0)
    }
}
