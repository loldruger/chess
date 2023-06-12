use crate::pieces::{Piece, Color};

#[derive(Copy, Clone, PartialEq, Eq)]

pub enum Square {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
    None
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let calc = *self as u8;
        write!(f, "{:?}", ((calc / 8) as usize, (calc % 8) as usize))
    }
}

impl Square {
    pub fn from_position(pos: (usize, usize)) -> Square {
        if pos.0 > 7 || pos.1 > 7 {
            panic!("Invalid position");
        }

        let calc = pos.0 * 8 + pos.1;

        unsafe { std::mem::transmute(calc as u8) }
    }

    pub fn into_position(self) -> (usize, usize) {
        let calc = self as u8;

        ((calc / 8) as usize, (calc % 8) as usize)
    }
}

#[derive(Clone, Copy)]
pub enum SquareKind {
    Empty,
    UnderAttack(Color),
    Vulnerable(Color),
    Pieces { 
        piece: Piece,
        position: Square,
        is_under_attack: bool
    }
}

impl SquareKind {
    pub fn get_piece(&self) -> Option<Piece> {
        match self {
            SquareKind::Pieces { piece, .. } => Some(*piece),
            _ => None,
        }
    }

    pub fn get_position(&self) -> Option<Square> {
        match self {
            SquareKind::Pieces { position, .. } => Some(*position),
            _ => None,
        }
    }
}
