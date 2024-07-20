use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use color::PieceColor;
use piece_type::PieceType;

pub mod color;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod piece_type;

pub trait Piece: CloneBox {
    fn color(&self) -> &PieceColor;
    fn piece_type(&self) -> &PieceType;
    fn get_valid_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition>;
    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition>;
    fn is_opponent(&self, color: &PieceColor) -> bool;
}

// TODO: Possible refactor
// Maybe we can  remove CloneBox by just having a factory of piece
// so we don't have to clone Box<dyn Piece> in board.with_pieces
// We can then remove the clones from test by passing a ref in spawn
// and use the factory builder
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
