use crate::{board::Board, square::Square, pieces::{Piece, Color, MoveStatus}};

pub struct GameManager {
    board: Board,
    turn: Color,
    piece_selected: Option<Piece>,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            board: Board::new(),
            turn: Color::White,
            piece_selected: None,
        }
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn select_piece(&mut self, coord: Square) -> Option<&Piece> {
        self.piece_selected = self.board.get_piece(coord).copied();
        
        if self.piece_selected.is_none() {
            return None;
        }

        let by_color = self.piece_selected.as_ref().unwrap().get_color();
        
        self.piece_selected
            .as_ref()
            .unwrap()
            .get_valid_moves(&mut self.board, coord)
            .iter()
            .for_each(|i| {
                match (*i).1 {
                    MoveStatus::Capturable => self.board.mark_captureable((*i).0, by_color),
                    _ => self.board.mark_vulnerable((*i).0, by_color),
                }
            });
            
        self.piece_selected.as_ref()
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), String> {
        let mut coord_to = coord_to;
        let piece = self.piece_selected.unwrap();
        let color = piece.get_color();

        if coord_from == coord_to {
            self.board.clear_marks();
            self.piece_selected = None;
            return Ok(());
        }

        let condition = piece.get_valid_moves(&mut self.board, coord_from)
            .iter()
            .any(|i| {
                (*i).0 == coord_to && match (*i).1 {
                    MoveStatus::Capturable => true,
                    MoveStatus::Movable => true,
                    MoveStatus::Castling => true,
                    _ => false,
                }
        });

        if let Piece::K(_) = piece {
            match color {
                Color::Black => {
                    if coord_from == Square::E8 && coord_to == Square::G8 {
                        self.board.move_piece(Square::H8, Square::F8).ok();
                    } else if coord_from == Square::E8 && coord_to == Square::C8 {
                        self.board.move_piece(Square::A8, Square::D8).ok();
                    } else if coord_from == Square::E8 && coord_to == Square::B8 {
                        self.board.move_piece(Square::A8, Square::D8).ok();
                        coord_to = Square::C8;
                    }
                },
                Color::White => {
                    if coord_from == Square::E1 && coord_to == Square::G1 {
                        self.board.move_piece(Square::H1, Square::F1).ok();
                    } else if coord_from == Square::E1 && coord_to == Square::C1 {
                        self.board.move_piece(Square::A1, Square::D1).ok();
                    } else if coord_from == Square::E1 && coord_to == Square::B1 {
                        self.board.move_piece(Square::A1, Square::D1).ok();
                        coord_to = Square::C1;
                    }
                },
            }
        }

        if condition {
            self.board.move_piece(coord_from, coord_to).ok();
            self.board.clear_marks();
            self.piece_selected = None;
            self.turn = self.turn.opposite();
            Ok(())
        } else {
            Err((format!("cannot move piece from {coord_from} to {coord_to}"), color).0)
        }
    }
}
