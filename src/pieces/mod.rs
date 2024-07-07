use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;

pub mod knight;
pub mod pawn;

#[derive(Debug, PartialEq, Clone)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
}

pub trait Piece: CloneBox {
    fn color(&self) -> &PieceColor;
    fn piece_type(&self) -> &PieceType;
    fn moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition>;
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
