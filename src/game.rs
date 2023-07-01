use crate::{board::Board, square::Square, pieces::{Piece, Color, MoveStatus, Pawn}};

pub enum GameState {
    Playing { turn: Color },
    InCheck { by_color: Color },
    Promoting { pawn: Pawn },
}

pub struct GameManager {
    board: Board,
    state: GameState,
    turn_count: u32,
    piece_selected: Option<Piece>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            state: GameState::Playing { turn: Color::White },
            turn_count: 0,
            piece_selected: None,
        }
    }

    pub fn get_turn(&self) -> Color {
        match self.state {
            GameState::Playing { turn } => turn,
            GameState::InCheck { by_color } => by_color,
            GameState::Promoting { ref pawn } => pawn.get_color(),
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub fn select_piece(&mut self, coord: Square) -> Result<(), &str> {
        let piece = self.board
            .get_piece_mut(coord)
            .ok_or("No piece found!")?;
        let color = piece.get_color();

        self.piece_selected = Some(piece.clone());
        self.piece_selected
            .as_ref()
            .unwrap()
            .get_valid_moves(&mut self.board, coord)
            .iter()
            .for_each(|i| {
                match (*i).1 {
                    MoveStatus::Capturable {..} => self.board.mark_valid_moves(MoveStatus::Capturable { by_color: color, activated: true }, (*i).0),
                    MoveStatus::Threaten {..} => self.board.mark_valid_moves(MoveStatus::Threaten { by_color: color, activated: true }, (*i).0),
                    MoveStatus::Pierced {..} => self.board.mark_valid_moves(MoveStatus::Pierced { by_color: color, activated: true }, (*i).0),
                    MoveStatus::EnPassant {..} => self.board.mark_valid_moves(MoveStatus::EnPassant { by_color: color, activated: true }, (*i).0),
                    MoveStatus::Castling {..} => self.board.mark_valid_moves(MoveStatus::Castling { by_color: color, activated: true }, (*i).0),
                    MoveStatus::Movable {..} => self.board.mark_valid_moves(MoveStatus::Movable { by_color: color, activated: true }, (*i).0),
                    _ => (),
                }
            });
            
        Ok(())
    }

    pub fn move_piece(&mut self, coord_from: Square, coord_to: Square) -> Result<(), &str> {
        let color = self.piece_selected.as_ref().unwrap().get_color();

        if coord_from == coord_to {
            self.board.clear_marks();
            self.piece_selected = None;
            
            return Ok(());
        }

        let condition = self.piece_selected
            .as_ref()
            .unwrap()
            .get_valid_moves(&mut self.board, coord_from)
            .iter()
            .any(|i| {
                (*i).0 == coord_to && match (*i).1 {
                    MoveStatus::Capturable {..} => true,
                    MoveStatus::EnPassant {..} => true,
                    MoveStatus::Movable {..} => true,
                    MoveStatus::Castling {..} => true,
                    _ => false,
                }
            });

        if !condition {
            return Err("Invalid move!");
        }

        self.piece_selected.as_mut().unwrap().move_to(&mut self.board, coord_to).unwrap();
        self.piece_selected = None;
        self.state = GameState::Playing { turn: color.opposite() };
        self.turn_count += 1;
        self.board.clear_marks();

        Ok(())
    }
}