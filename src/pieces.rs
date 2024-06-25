use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;

#[derive(Debug, PartialEq, Clone)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
}

pub trait Piece: CloneBox {
    fn color(&self) -> &PieceColor;
    fn piece_type(&self) -> &PieceType;
    fn moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition>;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Knight {
    color: PieceColor,
}

impl Knight {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for Knight {
    fn color(&self) -> &PieceColor {
        &self.color
    }
    fn piece_type(&self) -> &PieceType {
        &PieceType::Knight
    }

    fn moves(&self, _board: &CheckerBoard, _from: &BoardPosition) -> Vec<BoardPosition> {
        vec![]
    }
}

#[cfg(test)]
mod knight_piece_tests {
    use crate::pieces::{Knight, Piece, PieceColor};

    #[test]
    fn knight_can_be_white() {
        let color = PieceColor::White;
        let piece = Knight::new(color);
        assert_eq!(piece.color(), &PieceColor::White);
    }

    #[test]
    fn knight_can_be_black() {
        let color = PieceColor::Black;
        let piece = Knight::new(color);
        assert_eq!(piece.color(), &PieceColor::Black);
    }
}
