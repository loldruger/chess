use crate::{square::{Square, SquareKind, SquareStatus}, pieces::{Piece, Color, MoveStatus}};

pub struct Board {
    square: [[SquareKind; 8]; 8],
    capture_board: Vec<(Square, Color)>
}

impl Board {
    pub fn new() -> Board {
        Board {
            square: [[SquareKind::Empty(SquareStatus::None); 8]; 8],
            capture_board: Vec::new(),

        }
    }

    pub fn spawn(&mut self, coord: Square, piece: Piece) -> Result<(), &'static str> {
        let rank = coord.get_rank() as usize;
        let file = coord.get_file() as usize;
        
        if let Some(_) = self.square[file][rank].get_piece() {
            return Err("Square is occupied!");
        }

        self.square[file][rank] = SquareKind::Piece(piece, SquareStatus::None);
        
        piece.get_valid_moves(self, coord)
            .iter()
            .for_each(|i| {
                match (*i).1 {
                    MoveStatus::Capturable | MoveStatus::Pierced => self.capture_board.push(((*i).0, piece.get_color())),
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
                    SquareKind::Empty(SquareStatus::Capturable {..}) => *square = SquareKind::Empty(SquareStatus::None),
                    SquareKind::Empty(SquareStatus::Vulnerable {..}) => *square = SquareKind::Empty(SquareStatus::None),
                    SquareKind::Piece(piece, SquareStatus::Capturable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::None),
                    SquareKind::Piece(piece, SquareStatus::Vulnerable {..}) => *square = SquareKind::Piece(*piece, SquareStatus::None),
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
                                MoveStatus::Capturable | MoveStatus::Pierced => self.capture_board.push(((*i).0, piece.get_color())),
                                _ => (),
                            }
                        });
                    },
                    _ => (),
                }
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   A B C D E F G H")?;
        writeln!(f, "  ╔════════════════╗")?;
        for rank in (0..8).rev() {
            write!(f, "{} ║", rank + 1)?;
            for file in 0..8 {
                if (file + rank) % 2 == 1 {
                    write!(f, "\x1b[48;5;235m")?;
                } else {
                    write!(f, "\x1b[48;5;250m")?;
                }
                match self.square[file][rank] {
                    SquareKind::Empty(status) => {
                        match status {
                            SquareStatus::None => {
                                write!(f, "  ")?;
                            },
                            SquareStatus::Capturable {..} => {
                                write!(f, "\x1b[31mｘ\x1b[0m")?;
                            },
                            SquareStatus::Movable {..} => {
                                write!(f, "\x1b[31mｘ\x1b[0m")?;
                            },
                            SquareStatus::Vulnerable { .. } => {
                                // write!(f, "\x1b[41;5;250m  \x1b[0m")?;
                                write!(f, "  ")?;
                            },
                        }
                    },
                    SquareKind::Piece(piece, _) => {
                        match piece {
                            Piece::P(pawn) => {
                                if pawn.get_color() == Color::Black {
                                    write!(f, "♙ ")?;
                                } else {
                                    write!(f, "♟ ")?;
                                }
                            },
                            Piece::R(rook) => {
                                if rook.get_color() == Color::Black {
                                    write!(f, "♖ ")?;
                                } else {
                                    write!(f, "♜ ")?;
                                }
                            },
                            Piece::N(knight) => {
                                if knight.get_color() == Color::Black {
                                    write!(f, "♘ ")?;
                                } else {
                                    write!(f, "♞ ")?;
                                }
                            },
                            Piece::B(bishop) => {
                                if bishop.get_color() == Color::Black {
                                    write!(f, "♗ ")?;
                                } else {
                                    write!(f, "♝ ")?;
                                }
                            },
                            Piece::Q(queen) => {
                                if queen.get_color() == Color::Black {
                                    write!(f, "♕ ")?;
                                } else {
                                    write!(f, "♛ ")?;
                                }
                            },
                            Piece::K(king) => {
                                if king.get_color() == Color::Black {
                                    write!(f, "♔ ")?;
                                } else {
                                    write!(f, "♚ ")?;
                                }
                            },
                        }
                    },
                }
                
                write!(f, "\x1b[0m")?;
            }
            writeln!(f, "║ {}", rank + 1)?;
        }
        writeln!(f, "  ╚════════════════╝")?;
        writeln!(f, "   A B C D E F G H")?;

        Ok(())
    }
}