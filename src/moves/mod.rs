use std::ops::{Add, Sub};

pub mod promotable;
pub mod placable;

pub use promotable::Promotable;
pub use placable::Placable;

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    file: i32,
    rank: i32
}

impl Position {
    pub fn from_tuple(pos: (i32, i32)) -> Self {
        Self {
            file: pos.0,
            rank: pos.1,
        }
    }

    pub fn from_str(pos: &str) -> Option<Self> {
        if pos.len() != 2 {
            return None;
        }
        
        let file = pos.chars().nth(0).unwrap();
        let rank = pos.chars().nth(1).unwrap();

        Some(Self {
            file: (file as i32) - ('A' as i32),
            rank: ('8' as i32) - (rank as i32)
        })
    }

    pub fn get_file(&self) -> i32 {
        self.file
    }

    pub fn get_rank(&self) -> i32 {
        self.rank
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            file: self.file + rhs.file,
            rank: self.rank + rhs.rank,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            file: self.file - rhs.file,
            rank: self.rank - rhs.rank,
        }
    }
}