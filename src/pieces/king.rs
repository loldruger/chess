use crate::{
    moves::Placable, board::Board, square::Square,
};

use super::Color;

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
    coord: Square,
    is_threatened: bool,
    is_able_castling: bool,
}
impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            coord: Square::None,
            is_threatened: false,
            is_able_castling: true,
        }
    }

    pub fn is_threatened(&self) -> bool {
        self.is_threatened
    }

    pub fn set_threatened(&mut self, is_threatened: bool) {
        self.is_threatened = is_threatened;
    }

    pub fn unable_castling(&mut self) {
        self.is_able_castling = false;
    }

}

impl Placable for King {
    fn set_position(&mut self, position: Square) {
        self.coord = position;
    }

    fn get_valid_moves(&self, board: &mut Board, coord: Square, _: bool) -> Vec<Square> {
        let mut valid_move = Vec::new();

        let (current_file, current_rank) = coord.into_position();
        let current_file = current_file as i32;
        let current_rank = current_rank as i32;

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
                if board.is_empty(position) && !board.is_threatened(position, self.color) {
                    valid_move.push(position);
                } else if !board.is_empty(position) {
                    let query = board.get_piece(position).unwrap();
                    let color = query.get_color();

                    board.get_piece_mut(position).unwrap().set_threatened(color != self.color);
                }
            }
        }

        if self.is_able_castling && !self.is_threatened {
            let is_right_path_blocked = board.is_threatened(Square::F1, self.color) ||
                                                board.is_threatened(Square::G1, self.color) ||
                                                board.get_piece(Square::F1).is_some() ||
                                                board.get_piece(Square::G1).is_some();

            let is_left_path_blocked = board.is_threatened(Square::B1, self.color) ||
                                                board.is_threatened(Square::C1, self.color) ||
                                                board.is_threatened(Square::D1, self.color) ||
                                                board.get_piece(Square::B1).is_some() ||
                                                board.get_piece(Square::C1).is_some() ||
                                                board.get_piece(Square::D1).is_some();

            let dest_file_right = current_file + 2;
            let dest_file_left = current_file - 3;
            let dest_rank = current_rank;

            let query_right = board.get_piece(Square::H1).and_then(|x| {
                let a = match x {
                    crate::pieces::Piece::R(rook) => Some(rook).is_some_and(|r|  r.get_color() == self.color ),//&& r.is_able_castling() ),
                    _ => false,
                };
                Some(a)
            }).or(Some(false)).unwrap();

            let query_left = board.get_piece(Square::A1).and_then(|x| {
                let a = match x {
                    crate::pieces::Piece::R(rook) => Some(rook).is_some_and(|r|  r.get_color() == self.color ),//&& r.is_able_castling() ),
                    _ => false,
                };
                Some(a)
            }).or(Some(false)).unwrap();

            if !is_right_path_blocked && query_right && !board.is_threatened(Square::from_position((dest_file_right, dest_rank)), self.color) {
                valid_move.push(Square::from_position((dest_file_right, dest_rank)));
            }

            if !is_left_path_blocked && query_left && !board.is_threatened(Square::from_position((dest_file_left, dest_rank)), self.color) {
                valid_move.push(Square::from_position((dest_file_left + 1, dest_rank)));
                valid_move.push(Square::from_position((dest_file_left, dest_rank)));
            }

            let _self = board.get_piece_mut(coord).unwrap();
            match _self {
                crate::pieces::Piece::K(king) => {
                    king.unable_castling();
                },
                _ => {},
            }
        }

        valid_move
    }


    fn get_position(&self) -> Square {
        self.coord
    }

    fn get_color(&self) -> Color {
        self.color
    }


}

