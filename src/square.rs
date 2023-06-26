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
        write!(f, "{:?}", ((calc / 8) as i32, (calc % 8) as i32))
    }
}

impl Square {
    pub fn from_position(pos: (i32, i32)) -> Square {
        if pos.0 > 8 || pos.1 > 8 {
            panic!("Invalid position");
        }

        let calc = pos.0 * 8 + pos.1;

        unsafe { std::mem::transmute(calc as u8) }
    }

    pub fn from_str(s: &str) -> Option<Square> {
        let mut chars = s.chars();

        let file = chars.next().unwrap().to_ascii_uppercase() as u8;
        let rank = chars.next().unwrap().to_digit(10);
        if rank.is_none() {
            return None;
        }
        let rank = rank.unwrap() as u8;

        if file < b'A' || file > b'H' || rank < 1 || rank > 8 {
            return None;
        }

        let calc = (file - b'A') * 8 + (rank - 1);

        Some(unsafe { std::mem::transmute(calc as u8) })
    }
    
    pub fn into_position(self) -> (i32, i32) {
        let calc = self as u8;

        ((calc / 8) as i32, (calc % 8) as i32)
    }

    pub fn get_rank(self) -> i32 {
        let calc = self as u8;

        (calc % 8) as i32
    }

    pub fn get_file(self) -> i32 {
        let calc = self as u8;

        (calc / 8) as i32
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SquareStatus {
    None,
    Movable { by_color: Color },
    Capturable { by_color: Color },
    Vulnerable { by_color: Color },
}

#[derive(Clone, Copy)]
pub enum SquareKind {
    Empty(SquareStatus),
    Piece(Piece, SquareStatus)
}

impl SquareKind {
    pub fn get_piece(&self) -> Option<&Piece> {
        match self {
            SquareKind::Piece(piece, _) => Some(piece),
            _ => None,
        }
    }

    pub fn get_piece_mut(&mut self) -> Option<&mut Piece> {
        match self {
            SquareKind::Piece(ref mut piece, _) => Some(piece),
            _ => None,
        }
    }

    pub fn is_under_attack(&self, by_color: Color) -> bool {
        match self {
            SquareKind::Empty(SquareStatus::Capturable {by_color}) => true,
            SquareKind::Piece(_, SquareStatus::Capturable {by_color}) => true,
            _ => false,
        }
    }

    pub fn is_vulnerable(&self, by_color: Color) -> bool {
        match self {
            SquareKind::Empty(SquareStatus::Vulnerable {by_color}) => true,
            SquareKind::Piece(_, SquareStatus::Vulnerable {by_color}) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            SquareKind::Empty(_) => true,
            _ => false,
        }
    }
}
