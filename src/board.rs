use crate::{moves::{Placable}, pieces::{Color, Piece}, square::{SquareKind, Square}};

pub struct Board {
    square: [[SquareKind; 8]; 8]
}

impl Board {
    pub fn new() -> Self {
        Self {
            square: [[SquareKind::Empty; 8]; 8]
        }
    }

    pub fn spawn(&mut self, piece: Piece, coord: Square) -> Result<(), String> {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Empty | SquareKind::UnderAttack(_) | SquareKind::Vulnerable(_) => {
                self.square[file][rank] = SquareKind::Pieces {
                    piece,
                    position: coord,
                    is_under_attack: false,
                };
        
                Ok(())
            },
            _ => Err(format!("there is already a piece at {:#?}", coord.into_position())),
        }
    }
    
    pub fn is_empty(&self, coord: Square) -> bool {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Empty => true,
            _ => false,
        }
    }
    
    pub fn get_piece(&self, coord: Square) -> Option<Piece> {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Pieces { piece, .. } => Some(piece),
            _ => None,
        }
    }

    pub fn get_valid_moves(&self, coord: Square, is_threaten: bool) -> Vec<Square> {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Pieces { piece, .. } => piece.get_valid_moves(self, coord, is_threaten),
            _ => Vec::new(),
        }
    }

    pub fn mark_under_attack(&mut self, coords: &Vec<Square>, color: Color) {
        for i in coords {
            let (rank, file) = i.into_position();
            self.square[file][rank] = SquareKind::UnderAttack(color);
        }
    }

    pub fn mark_threaten(&mut self, coords: &Vec<Square>, color: Color) {
        for i in coords {
            let (rank, file) = i.into_position();
            match self.square[file][rank] {
                SquareKind::Empty => self.square[file][rank] = SquareKind::Vulnerable(color),
                SquareKind::Pieces { mut is_under_attack, .. } => is_under_attack = true,
                _ => (),
            }
        }
    }

    pub fn clear_board(&mut self) {
        for rank in self.square.iter_mut() {
            for square in rank.iter_mut() {
                match square {
                    SquareKind::UnderAttack(_) => *square = SquareKind::Empty,
                    _ => (),
                }
            }
        }
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), String> {
        let (rank_from, file_from) = coord_from.into_position();

        match self.square[file_from][rank_from] {
            SquareKind::Pieces { piece, .. } => {
                self.square[file_from][rank_from] = SquareKind::Empty;
                self.spawn(piece, coord_to)?;

                Ok(())
            },
            _ => Err(format!("cannot find any pieces at {coord_from}")),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  A B C D E F G H")?;
        for (i, rank) in self.square.iter().rev().enumerate() {
            write!(f, "{} ", 8 - i)?;
            for symbol in rank {
                let a = match symbol {
                    SquareKind::Empty => "_".to_owned(),
                    SquareKind::UnderAttack(_) => "X".to_owned(),
                    SquareKind::Vulnerable(_) => "V".to_owned(),
                    SquareKind::Pieces { piece, is_under_attack,.. } => match piece {
                        Piece::P(piece) => {
                            let a = if piece.get_color() == Color::Black { "♙" } else { "♟" };

                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::B(piece) => {
                            let a = if piece.get_color() == Color::Black { "♗" } else { "♝" }; 
                            
                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::N(piece) => {
                            let a = if piece.get_color() == Color::Black { "♘" } else { "♞" }; 
                            
                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::R(piece) => {
                            let a = if piece.get_color() == Color::Black { "♖" } else { "♜" }; 
                            
                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::Q(piece) => {
                            let a = if piece.get_color() == Color::Black { "♕" } else { "♛" }; 
                            
                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::K(piece) => {
                            let a = if piece.get_color() == Color::Black { "♔" } else { "♚" }; 
                            
                            if *is_under_attack {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                    },
                };
                write!(f, "{} ", a)?;
            }
            write!(f, "{} ", 8 - i)?;
            writeln!(f)?;
        }
        writeln!(f, "  A B C D E F G H")?;
        Ok(())
    }
}