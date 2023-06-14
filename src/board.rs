use crate::{moves::{Placable}, pieces::{Color, Piece}, square::{SquareKind, Square}};

pub struct Board {
    square: [[SquareKind; 8]; 8],
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            square: [[SquareKind::Empty; 8]; 8],
            pieces: Vec::new(),
        }
    }

    pub fn spawn(&mut self, mut piece: Piece, coord: Square) -> Result<(), String> {
        let (rank, file) = coord.into_position();
        let file = file as usize;
        let rank = rank as usize;

        piece.set_position(coord);
        self.pieces.push(piece);
        
        match self.square[file][rank] {
            SquareKind::Empty | SquareKind::UnderAttack(_) | SquareKind::Vulnerable(_) => {
                self.square[file][rank] = SquareKind::Pieces(piece);
                Ok(())
            },
            _ => Err(format!("there is already a piece at {coord}")),
        }
    }
    
    pub fn is_empty(&self, coord: Square) -> bool {
        let (rank, file) = coord.into_position();
        let file = file as usize;
        let rank = rank as usize;

        if file > 7 || rank > 7 {
            return false;
        }

        match self.square[file][rank] {
            SquareKind::Pieces(_) => false,
            SquareKind::Empty | SquareKind::Vulnerable(_) | SquareKind::UnderAttack(_) => true,
        }
    }
    
    pub fn is_threatened(&mut self, coord: Square, color: Color) -> bool {
        self.pieces
            .clone()
            .iter()
            .filter(|&x| x.get_color() != color )
            .flat_map(|x| x.get_valid_moves(self, x.get_position(), true),)
            .any(|x| x == coord)
        }

    pub fn get_piece(&self, coord: Square) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|x| x.get_position() == coord)
    }

    pub fn get_piece_mut(&mut self, coord: Square) -> Option<&mut Piece> {
        self.pieces
            .iter_mut()
            .find(|x| x.get_position() == coord)
    }

    pub fn get_valid_moves(&mut self, coord: Square, is_threaten: bool) -> Vec<Square> {
        let (rank, file) = coord.into_position();
        let file = file as usize;
        let rank = rank as usize;

        match self.square[file][rank] {
            SquareKind::Pieces(piece) => piece.get_valid_moves(self, coord, is_threaten),
            _ => Vec::new(),
        }
    }

    pub fn mark_under_attack(&mut self, coords: &Vec<Square>, color: Color) {
        for i in coords {
            let (rank, file) = i.into_position();
            let file = file as usize;
            let rank = rank as usize;

            self.square[file][rank] = SquareKind::UnderAttack(color);
        }
    }

    pub fn mark_threaten(&mut self, coords: &Vec<Square>, color: Color) {
        for i in coords {
            let (rank, file) = i.into_position();
            let file = file as usize;
            let rank = rank as usize;

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
        let (rank_to, file_to) = coord_to.into_position();

        let file_from = file_from as usize;
        let rank_from = rank_from as usize;
        let file_to = file_to as usize;
        let rank_to = rank_to as usize;

        match self.square[file_from][rank_from] {
            SquareKind::Pieces(piece) => {
                self.square[file_from][rank_from] = SquareKind::Empty;
                self.square[file_to][rank_to] = SquareKind::Pieces(piece);
                self.pieces.iter_mut().find(|x| x.get_position() == coord_from).unwrap().set_position(coord_to);

                Ok(())
            },
            _ => Err(format!("cannot find any pieces at {coord_from}")),
        }
    }

    pub fn reset_threaten(&mut self) {
        for piece in self.pieces.iter_mut() {
            piece.set_threatened(false);
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