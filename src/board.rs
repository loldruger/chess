use crate::{moves::{Placable, Position}, pieces::{Color, Queen, King, Knight, Bishop, Rook, Pawn}};

pub struct Board<'a> {
    board: [[char; 8]; 8],
    pieces: Vec<Box<&'a dyn Placable>>
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Self {
            board: [['_'; 8]; 8],
            pieces: Vec::new()
        }
    }

    pub fn spawn_at(&mut self, piece: &'a mut (impl Placable + ToString), position: Position) -> Result<(), ()> {
        piece.set_position(position)?;
        self.pieces.push(Box::new(piece));


        let bound_rank = position.get_rank() as usize;
        let bound_file = position.get_file() as usize;

        self.board[bound_rank][bound_file] = piece.to_string().chars().nth(0).unwrap();
        
        self.show_valid_move(piece);
        Ok(())
    }

    pub fn show_valid_move(&mut self, target: &impl Placable) {
        for i in target.get_valid_moves(self).iter() { 
            let bound_rank = i.get_rank() as usize;
            let bound_file = i.get_file() as usize;

            if self.board[bound_rank][bound_file].eq(&'_') {
                self.board[bound_rank][bound_file] = 'x'
            }
        }
    }

    pub fn get_square_info(&self, position: Position) -> Option<&Box<&dyn Placable>> {
        self.pieces.iter().find(|x| {
            x.get_position() == position
        })
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

    pub fn is_square_under_attack(&self, position: Position, color: Color) -> bool {
        self.pieces
            .iter()
            .filter(|&piece| {piece.get_color() != color})
            .any(|piece| piece.get_valid_moves(self).contains(&position))
    }
}

impl<'a> std::fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in self.board.iter() {
            for &symbol in rank {
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}