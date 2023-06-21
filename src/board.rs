use crate::{square::{Square, SquareKind, SquareStatus}, pieces::{Piece, Color, MoveStatus}};

pub struct Board {
    square: [[SquareKind; 8]; 8],
    capture_board: Vec<(Square, Color)>
}

impl Board {
    pub fn new() -> Board {
        Board {
            square: [[SquareKind::Empty(SquareStatus::Normal); 8]; 8],
            capture_board: Vec::new(),

        }
    }

    pub fn spawn(&mut self, coord: Square, piece: Piece) -> Result<(), &'static str> {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;
        
        if let Some(_) = self.square[file][rank].get_piece() {
            return Err("Square is occupied!");
        }

        self.square[file][rank] = SquareKind::Piece(piece, SquareStatus::Normal);
        
        piece.get_valid_moves(self, coord)
            .iter()
            .for_each(|i| {
                match (*i).1 {
                    MoveStatus::Capturable | MoveStatus::CapturablePossibly => self.capture_board.push(((*i).0, piece.get_color())),
                    _ => (),
                }
            });
            
        Ok(())
    }

    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        let rank = square.get_rank() as usize;
        let file = square.get_file() as usize;

        self.square[file][rank].get_piece()
    }

    pub fn get_piece_mut(&mut self, square: Square) -> Option<&mut Piece> {
        let rank = square.get_rank() as usize;
        let file = square.get_file() as usize;

        self.square[file][rank].get_piece_mut()
    }

    pub fn get_capture_board(&self) -> &Vec<(Square, Color)> {
        &self.capture_board
    }

    pub fn is_under_attack(&self, coord: Square, by_color: Color) -> bool {
        self.capture_board.iter().any(|x| x.0 == coord && x.1 != by_color)
    }

    pub fn is_empty(&self, coord: Square) -> bool {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;

        self.square[file][rank].is_empty()
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), String> {
        let rank_from = coord_from.get_rank() as usize;
        let file_from = coord_from.get_file() as usize;
        let rank_to = coord_to.get_rank() as usize;
        let file_to = coord_to.get_file() as usize;

        match self.square[file_from][rank_from] {
            SquareKind::Piece(piece, status) => {
                self.square[file_from][rank_from] = SquareKind::Empty(status);
                self.square[file_to][rank_to] = SquareKind::Piece(piece, status);

                Ok(())
            },
            _ => Err(format!("cannot find any pieces at {coord_from}")),
        }
    }

    pub fn mark_captureable(&mut self, coord: Square, by_color: Color) {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;

        self.square[file][rank] = match self.square[file][rank] {
            SquareKind::Empty(_) => SquareKind::Empty(SquareStatus::Capturable {by_color}),
            SquareKind::Piece(piece, _) => SquareKind::Piece(piece, SquareStatus::Capturable {by_color}),
        };
    }

    pub fn mark_vulnerable(&mut self, coord: Square, by_color: Color) {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;

        match self.square[file][rank] {
            SquareKind::Empty(_) => self.square[file][rank] = SquareKind::Empty(SquareStatus::Vulnerable {by_color}),
            SquareKind::Piece(piece, _) => self.square[file][rank] = SquareKind::Piece(piece, SquareStatus::Vulnerable {by_color}),
        };
    }

    pub fn clear_marks(&mut self) {
        for rank in self.square.iter_mut() {
            for square in rank.iter_mut() {
                match square {
                    SquareKind::Empty(SquareStatus::Capturable {..}) => *square = SquareKind::Empty(SquareStatus::Normal),
                    SquareKind::Empty(SquareStatus::Vulnerable {..}) => *square = SquareKind::Empty(SquareStatus::Normal),
                    SquareKind::Piece(piece, SquareStatus::Capturable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::Normal),
                    SquareKind::Piece(piece, SquareStatus::Vulnerable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::Normal),
                    _ => (),
                }
            }
        }
    }

    pub fn update_capture_board(&mut self) {
        self.capture_board.clear();

        for (i, rank) in self.square.clone().iter().enumerate() {
            for (j, square) in rank.iter().enumerate() {
                match square {
                    SquareKind::Piece(piece, _) => {
                        piece.get_valid_moves(self, Square::from_position((j as i32, i as i32))).iter().for_each(|i| {
                            match (*i).1 {
                                MoveStatus::Capturable | MoveStatus::CapturablePossibly => self.capture_board.push(((*i).0, piece.get_color())),
                                _ => (),
                            }
                        });
                    },
                    _ => (),
                }
            }
        }
    }

    pub fn is_king_checked(&self, color: Color) -> bool {
        for rank in self.square.iter() {
            for square in rank.iter() {
                match square {
                    SquareKind::Piece(piece, _) => {
                        if let Piece::K(king) = piece {
                            if king.get_color() == color && king.is_checked() {
                                return true;
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        false
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  A B C D E F G H")?;
        for (i, rank) in self.square.iter().rev().enumerate() {
            write!(f, "{} ", 8 - i)?;
            for symbol in rank {
                let a = match symbol {
                    SquareKind::Empty(status) => {
                        match status {
                            SquareStatus::Normal => "_".to_owned(),
                            SquareStatus::Capturable {..} => "X".to_owned(),
                            SquareStatus::Vulnerable {..} => "V".to_owned(),
                        }
                    },
                    SquareKind::Piece(piece, status) => match piece {
                        Piece::P(pawn) => {
                            let a = if pawn.get_color() == Color::Black { "♙" } else { "♟" };

                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::B(bishop) => {
                            let a = if bishop.get_color() == Color::Black { "♗" } else { "♝" }; 
                            
                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::N(knight) => {
                            let a = if knight.get_color() == Color::Black { "♘" } else { "♞" }; 
                            
                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::R(rook) => {
                            let a = if rook.get_color() == Color::Black { "♖" } else { "♜" }; 

                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::Q(queen) => {
                            let a = if queen.get_color() == Color::Black { "♕" } else { "♛" }; 
                            
                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::K(king) => {
                            let a = if king.get_color() == Color::Black { "♔" } else { "♚" }; 
                            
                            if king.is_checked() {
                                println!("king is checked");
                            }
                            
                            match status {
                                SquareStatus::Capturable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
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