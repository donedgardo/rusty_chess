use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;

pub struct BoardSideEffects {
    pub takes: Vec<BoardPosition>,
    pub updates: Vec<BoardPiece>,
}
