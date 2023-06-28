use crate::{square::{SquareKind, Square}, pieces::{Piece, Color, MoveStatus}};

pub struct Board {
    square: Vec<Vec<SquareKind>>,
    capture_board: Vec<(Square, MoveStatus)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            square: vec![vec![SquareKind::Empty(MoveStatus::None); 8]; 8],
            capture_board: Vec::new(),
        }
    }

    pub fn spawn(&mut self, mut piece: Piece, coord_to: Square) -> Result<(), &str> {
        let file = coord_to.get_file() as usize;
        let rank = coord_to.get_rank() as usize;

        match &self.square[file][rank].clone() {
            SquareKind::Empty(status) => {
                piece.set_coord(coord_to);
                piece.get_valid_moves(self, coord_to)
                    .iter()
                    .for_each(|i| {
                        match (*i).1 {
                            MoveStatus::Capturable {..} => self.capture_board.push(((*i).0, (*i).1)),
                            MoveStatus::Pierced  {..} => self.capture_board.push(((*i).0, (*i).1)),
                            _ => (),
                        }
                    });

                self.square[file][rank] = SquareKind::Occupied(piece, *status);

                Ok(())
            },
            SquareKind::Occupied(_, _) => {
                Err("Square is not empty")
            },
        }
    }

    pub fn is_empty(&self, square: Square) -> bool {
        let file = square.get_file() as usize;
        let rank = square.get_rank() as usize;

        match &self.square[file][rank] {
            SquareKind::Empty(_) => true,
            SquareKind::Occupied(_, _) => false,
        }
    }

    pub fn is_under_attack(&self, square: Square, by_color: Color) -> bool {
        let file = square.get_file() as usize;
        let rank = square.get_rank() as usize;

        match &self.square[file][rank] {
            SquareKind::Empty(_) => false,
            SquareKind::Occupied(piece, _) => {
                if piece.get_color() == by_color {
                    false
                } else {
                    true
                }
            },
        }
    }

    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        let rank = square.get_rank() as usize;
        let file = square.get_file() as usize;

        match &self.square[file][rank] {
            SquareKind::Empty(_) => None,
            SquareKind::Occupied(piece, _) => Some(piece),
        }
    }

    pub fn get_piece_mut(&mut self, square: Square) -> Option<&mut Piece> {
        let file = square.get_file() as usize;
        let rank = square.get_rank() as usize;

        match &mut self.square[file][rank] {
            SquareKind::Empty(_) => None,
            SquareKind::Occupied(piece, _) => Some(piece),
        }
    }

    pub fn get_capture_board(&mut self) -> &Vec<(Square, MoveStatus)> {
        &self.capture_board
    }

    pub fn update_capture_board(&mut self) {
        self.capture_board.clear();

        for (i, rank) in self.square.clone().iter().enumerate() {
            for (j, square) in rank.iter().enumerate() {
                match square {
                    SquareKind::Occupied(piece, _) => {
                        piece
                            .get_valid_moves(self, Square::from_position((i as i32, j as i32)))
                            .iter()
                            .for_each(|i| {
                                match (*i).1 {
                                    MoveStatus::Capturable {..} => self.capture_board.push(((*i).0, (*i).1)),
                                    MoveStatus::Pierced {..} => self.capture_board.push(((*i).0, (*i).1)),
                                    _ => (),
                                }
                            });
                    },
                    _ => (),
                }
            }
        }
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), &'static str> {
        let file_from = coord_from.get_file() as usize;
        let rank_from = coord_from.get_rank() as usize;

        let file_to = coord_to.get_file() as usize;
        let rank_to = coord_to.get_rank() as usize;

        match self.square[file_from][rank_from] {
            SquareKind::Empty(_) => Err("Square is empty"),
            SquareKind::Occupied(ref mut piece, status) => {
                piece.set_coord(coord_to);

                self.square[file_to][rank_to] = SquareKind::Occupied(piece.clone(), status);
                self.square[file_from][rank_from] = SquareKind::Empty(status);
                
                Ok(())  
            },
        }
    }

    pub fn mark_moves(&mut self, move_kind: MoveStatus, coord: Square) {
        let file = coord.get_file() as usize;
        let rank = coord.get_rank() as usize;
        
        self.square[file][rank] = match &self.square[file][rank] {
            SquareKind::Empty(_) => SquareKind::Empty(move_kind),
            SquareKind::Occupied(piece, _) => SquareKind::Occupied(piece.clone(), move_kind),
        };
    }

    pub fn clear_marks(&mut self) {
        for rank in self.square.iter_mut() {
            for square in rank.iter_mut() {
                *square = match square {
                    SquareKind::Empty(_) => SquareKind::Empty(MoveStatus::None),
                    SquareKind::Occupied(piece, status) => {
                        let a = match status {
                            MoveStatus::None => MoveStatus::None,
                            MoveStatus::Capturable { by_color, .. } => MoveStatus::Capturable { by_color: *by_color, activated: false },
                            MoveStatus::Pierced { by_color, .. } => MoveStatus::Pierced { by_color: *by_color, activated: false },
                            MoveStatus::EnPassant { by_color, .. } => MoveStatus::EnPassant { by_color: *by_color, activated: false },
                            MoveStatus::Castling { by_color, .. } => MoveStatus::Castling { by_color: *by_color, activated: false },
                            MoveStatus::Movable { by_color, .. } => MoveStatus::Movable { by_color: *by_color, activated: false },
                        };
        
                        SquareKind::Occupied(piece.clone(), a)
                    },
                };
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

                match &self.square[file][rank] {
                    SquareKind::Empty(move_status) => {
                        match move_status {
                            MoveStatus::Capturable {..} => write!(f, "\x1b[31mｘ\x1b[0m")?,
                            MoveStatus::Pierced {..} => write!(f, "\x1b[31mＰ\x1b[0m")?,
                            MoveStatus::EnPassant {..} => write!(f, "\x1b[31mＥ\x1b[0m")?,
                            MoveStatus::Castling {..} => write!(f, "\x1b[32mＣ\x1b[0m")?,
                            MoveStatus::Movable {..} => write!(f, "\x1b[32mｖ\x1b[0m")?,
                            _ => {
                                if (file + rank) % 2 == 1 {
                                    write!(f, "\x1b[48;5;235m")?;
                                } else {
                                    write!(f, "\x1b[48;5;250m")?;
                                }

                                write!(f, "  ")?;
                            },
                        }
                    },
                    SquareKind::Occupied(piece, status) => {
                        if let MoveStatus::Capturable { by_color, activated } = status {
                            if *by_color == piece.get_color().opposite() && *activated {
                                write!(f, "\x1b[31m")?;
                            } 
                        }
                        match piece {
                            Piece::P(pawn) => {
                                match pawn.get_color() {
                                    Color::White => write!(f, "♟ ")?,
                                    Color::Black => write!(f, "♙ ")?,
                                }
                            },
                            Piece::N(knight) => {
                                match knight.get_color() {
                                    Color::White => write!(f, "♞ ")?,
                                    Color::Black => write!(f, "♘ ")?,
                                }
                            },
                            Piece::B(bishop) => {
                                match bishop.get_color() {
                                    Color::White => write!(f, "♝ ")?,
                                    Color::Black => write!(f, "♗ ")?,
                                }
                            },
                            Piece::R(rook) => {
                                match rook.get_color() {
                                    Color::White => write!(f, "♜ ")?,
                                    Color::Black => write!(f, "♖ ")?,
                                }
                            },
                            Piece::Q(queen) => {
                                match queen.get_color() {
                                    Color::White => write!(f, "♛ ")?,
                                    Color::Black => write!(f, "♕ ")?,
                                }
                            },
                            Piece::K(king) => {
                                match king.get_color() {
                                    Color::White => write!(f, "♚ ")?,
                                    Color::Black => write!(f, "♔ ")?,
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