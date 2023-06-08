use crate::{moves::{Placable, Position}, pieces::{Color, Piece}};

#[derive(Clone, Copy)]
pub enum Square {
    Empty,
    Pieces(Piece)
}

pub struct Board {
    square: [[Square; 8]; 8],
    pieces: Vec<Piece>
}

impl Board {
    pub fn new() -> Self {
        Self {
            square: [[Square::Empty; 8]; 8],
            pieces: Vec::new()
        }
    }
    pub fn spawn(&mut self, piece: Piece) -> Result<(), ()> {
        let bound_rank = piece.get_position().get_rank() as usize;
        let bound_file = piece.get_position().get_file() as usize;
        
        self.square[bound_rank][bound_file] = Square::Pieces(piece);
        self.pieces.push(piece);
        
        Ok(())
    }
    
    pub fn get_square_info(&self, position: Position) -> Option<&Piece> {
        self.pieces.iter().find(|x| {
            x.get_position() == position
        })
    }

    pub fn is_empty(&self, position: Position) -> bool {
        let bound_rank = position.get_rank() as usize;
        let bound_file = position.get_file() as usize;

        match self.square[bound_rank][bound_file] {
            Square::Empty => true,
            _ => false
        }
    }

    pub fn is_enemy_piece(&self, position: Position, color: Color) -> bool {
        self.get_square_info(position)
            .is_some_and(|x| x.get_color() != color )
    }

    pub fn is_square_under_attack(&self, position: Position, color: Color) -> bool {
        self.pieces
            .iter()
            .filter(|&piece| {piece.get_color() != color})
            .any(|piece| piece.get_valid_moves(self).contains(&position))
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in self.square.iter() {
            for &symbol in rank {
                let a = match symbol {
                    Square::Empty => '_',
                    Square::Pieces(piece) => match piece {
                        Piece::P(piece) => if piece.get_color() == Color::Black {'♙'} else {'♟'},
                        Piece::B(piece) => if piece.get_color() == Color::Black {'♗'} else {'♝'},
                        Piece::N(piece) => if piece.get_color() == Color::Black {'♘'} else {'♞'},
                        Piece::R(piece) => if piece.get_color() == Color::Black {'♖'} else {'♜'},
                        Piece::Q(piece) => if piece.get_color() == Color::Black {'♕'} else {'♛'},
                        Piece::K(piece) => if piece.get_color() == Color::Black {'♔'} else {'♚'},
                    },
                };
                write!(f, "{} ", a)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}