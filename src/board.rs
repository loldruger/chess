use crate::{moves::{Placable, Position}, pieces::Color};

pub struct Board {
    board: [[char; 8]; 8]
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [['_'; 8]; 8]
        }
    }

    pub fn spawn(&mut self, piece: &(impl Placable + ToString)) -> Result<(), ()> {
        let pos = piece.get_position();
        let bound_rank = pos.get_rank() as usize;
        let bound_file = pos.get_file() as usize;

        self.board[bound_rank][bound_file] = piece.to_string().chars().nth(0).unwrap();
        Ok(())
    }

    pub fn show_valid_move(&mut self, target: &impl Placable) {
        for i in target.get_valid_moves(self).iter() { 
            let bound_rank = i.get_rank() as usize;
            let bound_file = i.get_file() as usize;

            // if self.board[bound_rank][bound_file].eq(&'_') {
                self.board[bound_rank][bound_file] = 'x'
            // }
        }
    }

    pub fn is_empty(&self, position: Position) -> bool {
        let bound_rank = position.get_rank() as usize;
        let bound_file = position.get_file() as usize;

        match self.board[bound_rank][bound_file] {
            '_' => true,
            _ => false
        }
    }

    pub fn is_enemy_piece(&self, position: Position, color: Color) -> bool {
        let bound_rank = position.get_rank() as usize;
        let bound_file = position.get_file() as usize;

        match color {
            Color::Black => 
                match self.board[bound_rank][bound_file] {
                    '♚'|'♛'|'♜'|'♝'|'♞'|'♟' => true,
                    '_' => false,
                    _ => false
                },
            Color::White =>
                match self.board[bound_rank][bound_file] {
                    '♚'|'♛'|'♜'|'♝'|'♞'|'♟' => false,
                    '_' => false,
                    _ => true
                },
        }
        

    }
}