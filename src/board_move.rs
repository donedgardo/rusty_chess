use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::piece_type::PieceType;

#[derive(Clone)]
pub struct BoardMove(PieceType, PieceColor, BoardPosition, BoardPosition);
impl BoardMove {
    pub fn new(
        piece_type: PieceType,
        color: PieceColor,
        from: BoardPosition,
        to: BoardPosition,
    ) -> Self {
        Self(piece_type, color, from, to)
    }
    pub fn from(&self) -> &BoardPosition {
        &self.2
    }

    pub fn to(&self) -> &BoardPosition {
        &self.3
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.0
    }

    pub fn piece_color(&self) -> &PieceColor {
        &self.1
    }
}
