use crate::{board::Board, square::{Square, SquareKind}};

use super::{Color, MoveStatus};

#[derive(Clone, Debug)]
pub struct Pawn {
    color: Color,
    coord: Square,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            coord: Square::None,
        }
    }

    pub fn set_coord(&mut self, coord: Square) {
        self.coord = coord;
    }

    pub fn get_coord(&self) -> Square {
        self.coord
    }
    
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_valid_moves(&self, board: &Board, coord_from: Square) -> Vec<(Square, MoveStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_file();
        let current_rank = coord_from.get_rank();

        // White pawn moves forward by one rank
        match self.color {
            Color::Black => {
                let target_rank = current_rank - 1;

                let is_enemy_piece_left = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_right = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_left_en_passant = board.get_piece(Square::from_position((current_file - 1, current_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_right_en_passant = board.get_piece(Square::from_position((current_file + 1, current_rank))).is_some_and(|x| x.get_color() != self.color );

                if board.is_empty(Square::from_position((current_file, target_rank))) {
                    if board.get_piece(Square::from_position((current_file, target_rank))).is_none() {
                        valid_moves.push((Square::from_position((current_file, target_rank)), MoveStatus::Movable { by_color: self.color, activated: false }));
                    }
                }
                // Double move from the starting rank
                if current_rank == 6 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    if board.get_piece(Square::from_position((current_file, target_rank - 1))).is_none() {
                        valid_moves.push((Square::from_position((current_file, target_rank - 1)), MoveStatus::Movable { by_color: self.color, activated: false }));
                    }
                }
                // Capture diagonally to the left
                if current_file > 0 {
                    let square = Square::from_position((current_file - 1, target_rank));
                    if is_enemy_piece_left {
                        valid_moves.push((square, MoveStatus::Capturable { by_color: self.color, activated: false }));
                    } else {
                        valid_moves.push((square, MoveStatus::Threaten { by_color: self.color, activated: false }));
                    }
                }
                // Capture diagonally to the right
                if current_file < 7 {
                    let square = Square::from_position((current_file + 1, target_rank));
                    if is_enemy_piece_right{
                        valid_moves.push((square, MoveStatus::Capturable { by_color: self.color, activated: false }));
                    } else {
                        valid_moves.push((square, MoveStatus::Threaten { by_color: self.color, activated: false }));
                    }
                }

                if current_rank == 3 && is_enemy_piece_left_en_passant {
                    if let Some(piece) = board.get_piece(Square::from_position((current_file, current_rank))) {
                        if let super::Piece::P(_) = piece {
                            valid_moves.push((Square::from_position((current_file - 1, target_rank)), MoveStatus::EnPassant { by_color: self.color, activated: false }));
                        }
                    }
                }

                if current_rank == 3 && is_enemy_piece_right_en_passant {
                    if let Some(piece) = board.get_piece(Square::from_position((current_file, current_rank))) {
                        if let super::Piece::P(_) = piece {
                            valid_moves.push((Square::from_position((current_file + 1, target_rank)), MoveStatus::EnPassant { by_color: self.color, activated: false }));
                        }
                    }
                }
            },
            Color::White => {
                let target_rank = current_rank + 1;

                let is_enemy_piece_left = board.get_piece(Square::from_position((current_file - 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_right = board.get_piece(Square::from_position((current_file + 1, target_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_left_en_passant = board.get_piece(Square::from_position((current_file - 1, current_rank))).is_some_and(|x| x.get_color() != self.color );
                let is_enemy_piece_right_en_passant = board.get_piece(Square::from_position((current_file + 1, current_rank))).is_some_and(|x| x.get_color() != self.color );

                if target_rank <= 7 &&
                    board.is_empty(Square::from_position((current_file, target_rank))) && 
                    board.get_piece(Square::from_position((current_file, target_rank))).is_none(){
                        valid_moves.push((Square::from_position((current_file, target_rank)), MoveStatus::Movable { by_color: self.color, activated: false }));

                }
                // Double move from the starting rank
                if current_rank == 1 && board.is_empty(Square::from_position((current_file, target_rank))) {
                    if board.get_piece(Square::from_position((current_file, target_rank + 1))).is_none() {
                        valid_moves.push((Square::from_position((current_file, target_rank + 1)), MoveStatus::Movable { by_color: self.color, activated: false }));
                    }
                }
                // Capture diagonally to the left
                if current_file > 0 && target_rank <= 7 {
                    let square = Square::from_position((current_file - 1, target_rank));
                    if is_enemy_piece_left {
                        valid_moves.push((square, MoveStatus::Capturable { by_color: self.color, activated: false }));
                    } else {
                        valid_moves.push((square, MoveStatus::Threaten { by_color: self.color, activated: false }));
                    }
                }
                // Capture diagonally to the right
                if current_file < 7 && target_rank <= 7 {
                    let square = Square::from_position((current_file + 1, target_rank));
                    if is_enemy_piece_right{
                        valid_moves.push((square, MoveStatus::Capturable { by_color: self.color, activated: false }));
                    } else {
                        valid_moves.push((square, MoveStatus::Threaten { by_color: self.color, activated: false }));
                    }
                }

                if current_rank == 4 && is_enemy_piece_left_en_passant {
                    if let Some(piece) = board.get_piece(Square::from_position((current_file, current_rank))) {
                        if let super::Piece::P(_) = piece {
                            valid_moves.push((Square::from_position((current_file - 1, target_rank)), MoveStatus::EnPassant { by_color: self.color, activated: false }));
                        }
                    }
                }

                if current_rank == 4 && is_enemy_piece_right_en_passant {
                    if let Some(piece) = board.get_piece(Square::from_position((current_file, current_rank))) {
                        if let super::Piece::P(_) = piece {
                            valid_moves.push((Square::from_position((current_file + 1, target_rank)), MoveStatus::EnPassant { by_color: self.color, activated: false }));
                        }
                    }
                }
            }
        }

        valid_moves
    }

    pub fn move_to(&mut self, board: &mut Board, coord_to: Square) -> Result<(), &'static str> {
        let rank_from = self.coord.get_rank() as usize;
        let file_from = self.coord.get_file() as usize;
        let rank_to = coord_to.get_rank() as usize;
        let file_to = coord_to.get_file() as usize;

        match self.color {
            Color::Black => {
                if rank_from == 1 && rank_to == 0 {
                    // self.set_state(GameState::Promoting { pawn });
                }

                let square = board.get_square_mut(coord_to).unwrap();
                if let SquareKind::Empty(MoveStatus::EnPassant { by_color, .. }) = square {
                    if *by_color == self.color {
                        board.despawn(Square::from_position((file_to as i32, rank_from as i32))).ok();
                        
                    }
                }
            },
            Color::White => {
                if rank_from == 6 && rank_to == 7 {
                    // self.set_state(GameState::Promoting { pawn });
                } 

                let square = board.get_square_mut(coord_to).unwrap();
                if let SquareKind::Empty(MoveStatus::EnPassant { by_color, .. }) = square {
                    if *by_color == self.color {
                        board.despawn(Square::from_position((file_to as i32, rank_from as i32))).ok();
                        
                    }
                }

            },
        }
        
        board.move_piece(self.coord, coord_to)
    }

    pub fn try_into_queen(&self) -> Option<super::Queen> {
        Some(super::Queen::new(self.color))
    }

    pub fn try_into_knight(&self) -> Option<super::Knight> {
        Some(super::Knight::new(self.color))
    }

    pub fn try_into_bishop(&self) -> Option<super::Bishop> {
        Some(super::Bishop::new(self.color))
    }

    pub fn try_into_rook(&self) -> Option<super::Rook> {
        Some(super::Rook::new(self.color))
    }
}