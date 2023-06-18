use crate::{board::Board, square::Square, pieces::{Piece, Color}};

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
                if (*i).1 {
                    self.board.mark_vulnerable((*i).0, by_color);
                } else {
                    self.board.mark_under_attack((*i).0, by_color);
                }
            });
            
        self.piece_selected.as_ref()
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), String> {
        let piece = self.piece_selected.unwrap();
        let color = piece.get_color();

        if piece.get_valid_moves(&mut self.board, coord_from).iter().any(|i| (*i).0 == coord_to) {
            self.board.move_piece(coord_from, coord_to).ok();

            self.piece_selected = None;
            self.turn = self.turn.opposite();
            Ok(())
        } else {
            Err((format!("cannot move piece from {coord_from} to {coord_to}"), color).0)
        }
    }
}
