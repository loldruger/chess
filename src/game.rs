// use std::{cell::RefCell, borrow::{Borrow, BorrowMut}, rc::Rc};

use crate::{board::Board, pieces::{Color, Piece}, square::Square};

pub struct GameManager {
    board: Board,
    turn: Color,
    piece_selected: (Option<Piece>, Square),
    piece_selected_valid_moves: Vec<Square>,
    piece_selected_threaten_moves: Vec<Square>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            piece_selected: (None, Square::A1),
            piece_selected_valid_moves: Vec::new(),
            piece_selected_threaten_moves: Vec::new(),
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
    
    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn opposite_turn(&mut self) {
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
        }
    }

    pub fn select_piece(&mut self, coord: Square) -> Result<(), String> {
        self.piece_selected.0 = self.board.get_piece(coord).cloned();
        if self.piece_selected.0.is_none() {
            return Err(format!("no piece selected"));
        }

        if self.piece_selected.0.unwrap().get_color() != self.turn {
            return Err(format!("not your turn"));
        }

        self.piece_selected.1 = coord;

        self.piece_selected_valid_moves = self.board.get_valid_moves(coord, false);
        self.piece_selected_threaten_moves = self.board.get_valid_moves(coord, true);

        self.board.clear_board();
        self.board.mark_threaten(&self.piece_selected_threaten_moves, self.piece_selected.0.unwrap().get_color());
        self.board.mark_under_attack(&self.piece_selected_valid_moves, self.piece_selected.0.unwrap().get_color());

        Ok(())
    }

    pub fn move_piece(&mut self, coord_to: Square) -> Result<(), String> {
        if self.board.get_piece(coord_to).is_some_and(|x| x.is_threatened()) {
            self.board.get_piece_mut(coord_to).unwrap().set_position(Square::None);
        } else {
            if !self.piece_selected_valid_moves.iter().any(|&x| x == coord_to) {
                return Err(format!("invalid move"));
            }
        }

        if let Some(piece) = self.piece_selected.0 {
            let coord_from = self.piece_selected.1;

            let king = match piece {
                Piece::K(p) => Some(p),
                _ => None
            };

            if king.is_some_and(|_| coord_from == Square::E1 && coord_to == Square::G1) {
                self.board.move_piece(Square::H1, Square::F1).unwrap();
            } else if king.is_some_and(|_| coord_from == Square::E1 && coord_to == Square::C1) {
                self.board.move_piece(Square::A1, Square::D1).unwrap();
            } else if king.is_some_and(|_| coord_from == Square::E1 && coord_to == Square::B1) {
                return Err(format!("invalid move"));
            }

            self.board.clear_board();
            self.board.move_piece(coord_from, coord_to).and_then(|_| {
                self.piece_selected = (None, Square::None);
                self.piece_selected_valid_moves = Vec::new();
                self.turn = match self.turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
            
                Ok(())
            }).or_else(|x| {
                Err(x)
            })
        } else {
            Err(format!("no piece selected"))
        }
    }

    pub fn reset_threaten(&mut self) {
        self.board.reset_threaten();
    }
}


