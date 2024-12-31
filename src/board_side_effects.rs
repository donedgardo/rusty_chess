use crate::board_position::BoardPosition;
use crate::pieces::Piece;

pub struct BoardUpdate(pub BoardPosition, pub Option<Box<dyn Piece>>);

impl BoardUpdate {
    pub fn pos(&self) -> &BoardPosition {
        &self.0
    }
    pub fn piece(&self) -> &Option<Box<dyn Piece>> {
        &self.1
    }
}

pub struct BoardSideEffects {
    pub takes: Vec<BoardPosition>,
    pub updates: Vec<BoardUpdate>,
}
