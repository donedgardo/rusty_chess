use crate::board_position::BoardPosition;
use crate::pieces::piece_type::PieceType;

#[derive(Clone)]
pub struct BoardMove(PieceType, BoardPosition, BoardPosition);
impl BoardMove {
    pub fn new(piece_type: PieceType, from: BoardPosition, to: BoardPosition) -> Self {
        Self(piece_type, from, to)
    }
    pub fn from(&self) -> &BoardPosition {
        &self.1
    }

    pub fn to(&self) -> &BoardPosition {
        &self.2
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.0
    }
}
