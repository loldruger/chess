use crate::{square::{Square, SquareKind, SquareStatus}, pieces::{Piece, Color, CaptureStatus}};

pub struct Board {
    square: [[SquareKind; 8]; 8]
}

impl Board {
    pub fn new() -> Board {
        Board {
            square: [[SquareKind::Empty(SquareStatus::Normal); 8]; 8],
        }
    }

    pub fn spawn(&mut self, coord: Square, mut piece: Piece) -> Result<(), &'static str> {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;
        
        if let Some(_) = self.square[file][rank].get_piece() {
            return Err("Square is occupied!");
        }

        self.square[file][rank] = SquareKind::Piece(piece, SquareStatus::Normal);

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

    pub fn get_square(&self) -> &[[SquareKind; 8]; 8] {
        &self.square
    }

    pub fn get_pieces(&self, color: Color) -> Vec<&Piece> {
        self.square
            .iter()
            .flatten()
            .filter_map(|x| {
                if let SquareKind::Piece(piece, _) = x {
                    if piece.get_color() == color {
                        Some(piece)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<&Piece>>()
    }

    pub fn get_valid_moves(&mut self, coord_from: Square) -> Vec<(Square, CaptureStatus)> {
        let rank = coord_from.get_rank() as usize;
        let file = coord_from.get_file() as usize;

        match self.square[file][rank] {
            SquareKind::Piece(piece, _) => piece.get_valid_moves(self, coord_from),
            _ => Vec::new(),
        }
    }
    
    pub fn get_valid_moves_all(&self, color: Color) -> Vec<(Square, CaptureStatus)> {
        let mut all_moves = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::from_position((rank, file));
                if let Some(piece) = self.get_piece(square) {
                    if piece.get_color() == color {
                        let valid_moves = piece.get_valid_moves(self, square);
                        all_moves.extend(valid_moves);
                    }
                }
            }
        }
        all_moves
    }

    pub fn is_under_attack(&self, coord: Square, by_color: Color) -> bool {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;

        self.square[file][rank].is_under_attack(by_color)
    }

    pub fn is_vulnerable(&self, coord: Square, by_color: Color) -> bool {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;

        self.square[file][rank].is_vulnerable(by_color)
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
            SquareKind::Empty(_) => SquareKind::Empty(SquareStatus::Captureable {by_color}),
            SquareKind::Piece(piece, _) => SquareKind::Piece(piece, SquareStatus::Captureable {by_color}),
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
                    SquareKind::Empty(SquareStatus::Captureable {..}) => *square = SquareKind::Empty(SquareStatus::Normal),
                    SquareKind::Empty(SquareStatus::Vulnerable {..}) => *square = SquareKind::Empty(SquareStatus::Normal),
                    SquareKind::Piece(piece, SquareStatus::Captureable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::Normal),
                    SquareKind::Piece(piece, SquareStatus::Vulnerable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::Normal),
                    _ => (),
                }
            }
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
                    SquareKind::Empty(status) => {
                        match status {
                            SquareStatus::Normal => "_".to_owned(),
                            SquareStatus::Captureable {..} => "X".to_owned(),
                            SquareStatus::Vulnerable {..} => "V".to_owned(),
                        }
                    },
                    SquareKind::Piece(piece, status) => match piece {
                        Piece::P(pawn) => {
                            let a = if pawn.get_color() == Color::Black { "♙" } else { "♟" };

                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::B(bishop) => {
                            let a = if bishop.get_color() == Color::Black { "♗" } else { "♝" }; 
                            
                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::N(knight) => {
                            let a = if knight.get_color() == Color::Black { "♘" } else { "♞" }; 
                            
                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::R(rook) => {
                            let a = if rook.get_color() == Color::Black { "♖" } else { "♜" }; 

                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::Q(queen) => {
                            let a = if queen.get_color() == Color::Black { "♕" } else { "♛" }; 
                            
                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
                                _ => a.to_owned()
                            }
                        },
                        Piece::K(king) => {
                            let a = if king.get_color() == Color::Black { "♔" } else { "♚" }; 

                            match status {
                                SquareStatus::Captureable { .. } => format!("\x1b[0;31m{a}\x1b[0;37m"),
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