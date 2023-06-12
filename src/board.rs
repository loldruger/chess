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
                self.square[file][rank] = SquareKind::Pieces(piece);
                Ok(())
            },
            _ => Err(format!("there is already a piece at {:#?}", coord.into_position())),
        }
    }
    
    pub fn is_empty(&self, coord: Square) -> bool {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Pieces(_) => false,
            SquareKind::Empty | SquareKind::Vulnerable(_) | SquareKind::UnderAttack(_) => true,
        }
    }
    
    pub fn is_threatened(&mut self, coord: Square, color: Color) -> bool {
        self.square
            .clone()
            .iter()
            .flatten()
            .any(|square| match square {
                SquareKind::Pieces(piece) => 
                    piece.get_color() != color && piece.get_valid_moves(self, coord, true).contains(&coord),
                _ => false,
            })
    }

    pub fn get_piece(&self, coord: Square) -> Option<&Piece> {
        let (rank, file) = coord.into_position();

        self.square[file][rank].get_piece()
    }

    pub fn get_piece_mut(&mut self, coord: Square) -> Option<&mut Piece> {
        let (rank, file) = coord.into_position();

        self.square[file][rank].get_piece_mut()
    }

    pub fn get_valid_moves(&mut self, coord: Square, is_threaten: bool) -> Vec<Square> {
        let (rank, file) = coord.into_position();

        match self.square[file][rank] {
            SquareKind::Pieces(piece) => piece.get_valid_moves(self, coord, is_threaten),
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
                SquareKind::Pieces(_) => (),
                _ => (),
            }
        }
    }

    pub fn clear_board(&mut self) {
        for rank in self.square.iter_mut() {
            for square in rank.iter_mut() {
                match square {
                    SquareKind::UnderAttack(_) | SquareKind::Vulnerable(_) => *square = SquareKind::Empty,
                    _ => (),
                }
            }
        }
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), String> {
        let (rank_from, file_from) = coord_from.into_position();

        match self.square[file_from][rank_from] {
            SquareKind::Pieces(piece) => {
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
                    SquareKind::Pieces(piece) => match piece {
                        Piece::P(pawn) => {
                            let a = if pawn.get_color() == Color::Black { "♙" } else { "♟" };

                            if pawn.is_threatened() {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::B(bishop) => {
                            let a = if bishop.get_color() == Color::Black { "♗" } else { "♝" }; 
                            
                            if bishop.is_threatened() {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::N(knight) => {
                            let a = if knight.get_color() == Color::Black { "♘" } else { "♞" }; 
                            
                            if knight.is_threatened() {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::R(rook) => {
                            let a = if rook.get_color() == Color::Black { "♖" } else { "♜" }; 

                            if rook.is_threatened() {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::Q(queen) => {
                            let a = if queen.get_color() == Color::Black { "♕" } else { "♛" }; 
                            
                            if queen.is_threatened() {
                                format!("\x1b[0;31m{a}\x1b[0;37m")
                            } else {
                                a.to_owned()
                            }
                        },
                        Piece::K(king) => {
                            let a = if king.get_color() == Color::Black { "♔" } else { "♚" }; 
                            
                            if king.is_threatened() {
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