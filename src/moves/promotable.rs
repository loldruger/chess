use crate::pieces::{Rook, Bishop, Knight, Queen};

pub trait Promotable {
    fn into_rook(self) -> Rook;
    fn into_bishop(self) -> Bishop;
    fn into_knight(self) -> Knight;
    fn into_queen(self) -> Queen;
}