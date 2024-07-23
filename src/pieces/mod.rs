use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use color::PieceColor;
use piece_type::PieceType;

mod bishop;
pub mod color;
mod diagonal_mover;
pub mod factory;
mod horizontal_vertical_mover;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod piece_type;
pub mod rook;

pub trait Piece: CloneBox + Send + Sync {
    fn color(&self) -> &PieceColor;
    fn piece_type(&self) -> &PieceType;
    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition>;
    fn is_opponent(&self, color: &PieceColor) -> bool;
}

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn Piece>;
}
impl<T> CloneBox for T
where
    T: 'static + Piece + Clone,
{
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Box<dyn Piece> {
        self.clone_box()
    }
}
