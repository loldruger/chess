pub mod pawn;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod queen;

pub use pawn::Pawn;
pub use rook::Rook;
pub use bishop::Bishop;
pub use knight::Knight;
pub use queen::Queen;

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White
}

// pub enum Piece {
//     Pawn(Color),
//     Bishop(Color),
//     Knight(Color),
//     Rook(Color),
//     Queen(Color),
//     King(Color)
// }