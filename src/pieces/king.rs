use crate::{square::Square, board::Board};

use super::{Color, MoveStatus};

#[derive(Clone, Debug)]
pub struct King {
    color: Color,
    coord: Square,
    is_checked: bool,
    is_once_moved: bool,
}

impl King {
    pub fn new(color: Color) -> King {
        King {
            color,
            coord: Square::None,
            is_checked: false,
            is_once_moved: false,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub fn set_checked(&mut self, is_checked: bool) {
        self.is_checked = is_checked;
    }

    pub fn set_once_moved(&mut self) {
        self.is_once_moved = true;
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

    pub fn get_valid_moves(&self, board: &mut Board, coord_from: Square) -> Vec<(Square, MoveStatus)> {
        let mut valid_moves = Vec::new();

        let current_file = coord_from.get_file();
        let current_rank = coord_from.get_rank();

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
                if board.is_empty(position) {
                    valid_moves.push((position, MoveStatus::Capturable { by_color: self.color, activated: false }));
                }
            }
        }
        
        match self.color {
            Color::Black => {
                if board.get_piece(Square::H8).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::F8) &&
                    board.is_empty(Square::G8) &&
                    !board.is_under_attack(Square::G8, Color::Black) &&
                    !board.is_under_attack(Square::F8, Color::Black) &&
                    !self.is_checked &&
                    !self.is_once_moved
                {
                    valid_moves.push((Square::G8, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::F8, MoveStatus::Castling { by_color: self.color, activated: false }));
                }
            
                if board.get_piece(Square::A8).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::B8) &&
                    board.is_empty(Square::C8) &&
                    board.is_empty(Square::D8) &&
                    !board.is_under_attack(Square::B8, Color::Black) &&
                    !board.is_under_attack(Square::C8, Color::Black) &&
                    !board.is_under_attack(Square::D8, Color::Black) &&
                    !self.is_checked &&
                    !self.is_once_moved
                {
                    valid_moves.push((Square::B8, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::C8, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::D8, MoveStatus::Castling { by_color: self.color, activated: false }));
                }
            },
            Color::White => {
                if board.get_piece(Square::H1).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::F1) &&
                    board.is_empty(Square::G1) &&
                    !board.is_under_attack(Square::G1, Color::Black) &&
                    !board.is_under_attack(Square::F1, Color::Black) &&
                    !self.is_checked &&
                    !self.is_once_moved
                {
                    valid_moves.push((Square::G1, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::F1, MoveStatus::Castling { by_color: self.color, activated: false }));
                }
                
                if board.get_piece(Square::A1).is_some_and(|x| x.get_color() == self.color) &&
                    board.is_empty(Square::B1) &&
                    board.is_empty(Square::C1) &&
                    board.is_empty(Square::D1) &&
                    !board.is_under_attack(Square::B1, Color::Black) &&
                    !board.is_under_attack(Square::C1, Color::Black) &&
                    !board.is_under_attack(Square::D1, Color::Black) &&
                    !self.is_checked &&
                    !self.is_once_moved
                {
                    valid_moves.push((Square::B1, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::C1, MoveStatus::Castling { by_color: self.color, activated: false }));
                    valid_moves.push((Square::D1, MoveStatus::Castling { by_color: self.color, activated: false }));
                }
            },
        }

        let opponent_move = board
            .get_capture_board()
            .iter()
            .filter_map(|x| {
                let color = match x.1 {
                    MoveStatus::None => todo!(),
                    MoveStatus::Capturable { by_color, .. } => by_color,
                    MoveStatus::Pierced { by_color, .. } => by_color,
                    MoveStatus::EnPassant { by_color, .. } => by_color,
                    MoveStatus::Castling { by_color, .. } => by_color,
                    MoveStatus::Movable { by_color, .. } => by_color,
                };

                if color == self.color.opposite() {
                    Some(x.0)
                } else {
                    None
                }
            }).collect::<Vec<Square>>();

        valid_moves.retain(|x| !opponent_move.contains(&x.0));
        
        valid_moves
    }

    pub fn move_to(&mut self, board: &mut Board, mut coord_to: Square) -> Result<(), &'static str> {
        self.is_once_moved = true;

        match self.color {
            Color::Black => {
                if self.coord == Square::E8 && coord_to == Square::G8 {
                    board.move_piece(Square::H8, Square::F8).ok();
                } else if self.coord == Square::E8 && coord_to == Square::C8 {
                    board.move_piece(Square::A8, Square::D8).ok();
                } else if self.coord == Square::E8 && coord_to == Square::B8 {
                    board.move_piece(Square::A8, Square::D8).ok();
                    coord_to = Square::C8;
                }
            },
            Color::White => {
                if self.coord == Square::E1 && coord_to == Square::G1 {
                    board.move_piece(Square::H1, Square::F1).ok();
                } else if self.coord == Square::E1 && coord_to == Square::C1 {
                    board.move_piece(Square::A1, Square::D1).ok();
                } else if self.coord == Square::E1 && coord_to == Square::B1 {
                    board.move_piece(Square::A1, Square::D1).ok();
                    coord_to = Square::C1;
                }
            },
        }

        board.move_piece(self.coord, coord_to)
    }

}