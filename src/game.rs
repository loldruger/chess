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

    pub fn get_selected_piece(&self) -> &Option<Piece> {
        &self.piece_selected.0
    }

    pub fn get_selected_piece_valid_moves(&self) -> &Vec<Square> {
        &self.piece_selected_valid_moves
    }

    pub fn select_piece(&mut self, position: Square) -> Result<(), String> {
        self.piece_selected.0 = self.board.get_piece(position).cloned();
        self.piece_selected.1 = position;

        self.piece_selected_valid_moves = self.board.get_valid_moves(position, false);
        self.piece_selected_threaten_moves = self.board.get_valid_moves(position, true);

        self.board.clear_board();
        self.board.mark_threaten(&self.piece_selected_threaten_moves, self.piece_selected.0.unwrap().get_color());
        self.board.mark_under_attack(&self.piece_selected_valid_moves, self.piece_selected.0.unwrap().get_color());

        Ok(())
    }

    pub fn move_piece(&mut self, position: Square) ->Result<(), String> {
        if !self.piece_selected_valid_moves.iter().any(|&x| x == position) {
            return Err(format!("invalid move"));
        }

        if let Some(_) = self.piece_selected.0 {
            let coord_from = self.piece_selected.1;
            self.board.clear_board();
            self.board.move_piece(coord_from, position).and_then(|_| {
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

}


