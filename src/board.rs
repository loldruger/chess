use crate::{moves::{Placable, Position}, pieces::{Color, Piece}};

#[derive(Clone)]
pub enum Square {
    Empty,
    Pieces(Piece)
}

pub struct Board {
    square: Vec<Vec<Square>>
}

impl Board {
    pub fn new() -> Self {
        Self {
            square: vec![vec![Square::Empty; 8]; 8]
        }
    }
    pub fn spawn(&mut self, piece: Piece) -> Result<(), ()> {
        let bound_rank = piece.get_position().get_rank() as usize;
        let bound_file = piece.get_position().get_file() as usize;
        
        match self.square[bound_rank][bound_file] {
            Square::Empty => {
                self.square[bound_rank][bound_file] = Square::Pieces(piece);
        
                Ok(())
            },
            Square::Pieces(_) => Err(()),
        }
    }
    
    pub fn get_square_info(&self, position: Position) -> Option<&Square> {
        let bound_rank = position.get_rank() as usize;
        let bound_file = position.get_file() as usize;
        
        Some(&self.square[bound_rank][bound_file])
    }

    pub fn get_piece_mut(&mut self, position: Position) -> Option<&mut Piece> {
        let result = self.square.iter_mut().flatten().find(|x| {
            match x {
                Square::Empty => false,
                Square::Pieces(p) => p.get_position() == position,
            }
        });

        match result {
            Some(p) => {
                match p {
                    Square::Empty => None,
                    Square::Pieces(p) => Some(p),
                }
            },
            None => None,
        }
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
            .is_some_and(|x| {
                match x {
                    Square::Empty => false,
                    Square::Pieces(piece) => match piece {
                        Piece::P(piece) => piece.get_color() != color,
                        Piece::B(piece) => piece.get_color() != color,
                        Piece::N(piece) => piece.get_color() != color,
                        Piece::R(piece) => piece.get_color() != color,
                        Piece::Q(piece) => piece.get_color() != color,
                        Piece::K(piece) => piece.get_color() != color,
                    },
                }
            })
    }

    pub fn is_square_under_attack(&self, position: Position, color: Color) -> bool {
        self.square
            .iter()
            .flatten()
            .filter(|&piece| {
                match piece {
                    Square::Empty => false,
                    Square::Pieces(p) => p.get_color() != color,
                }
            })
            .any(|piece| {
                match piece {
                    Square::Empty => false,
                    Square::Pieces(p) => p.get_valid_moves(self).contains(&position),
                }
            })
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in self.square.iter() {
            for symbol in rank {
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