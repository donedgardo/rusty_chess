use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::{Piece, PieceColor, PieceType};

pub struct BoardPiece(BoardPosition, Box<dyn Piece>);

impl BoardPiece {
    pub fn build(piece_type: PieceType, color: PieceColor, position: &str) -> Self {
        match piece_type {
            PieceType::Pawn => BoardPiece(position.parse().unwrap(), Box::new(Pawn::new(color))),
            PieceType::Knight => {
                BoardPiece(position.parse().unwrap(), Box::new(Knight::new(color)))
            }
        }
    }

    pub fn pos(&self) -> &BoardPosition {
        &self.0
    }
    pub fn piece(&self) -> &Box<dyn Piece> {
        &self.1
    }

    pub fn moves(&self, board: &CheckerBoard) -> Vec<BoardPosition> {
        self.piece().moves(&board, self.pos())
    }
}

#[cfg(test)]
mod board_piece_test {
    use super::*;
    use crate::pieces::{PieceColor, PieceType};

    #[test]
    fn it_builds_white_pawn() {
        let a1 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a1");
        assert_eq!(a1.pos(), &"a1".parse().unwrap());
        assert_eq!(a1.piece().piece_type(), &PieceType::Pawn);
        assert_eq!(a1.piece().color(), &PieceColor::White);
    }
    #[test]
    fn it_builds_black_pawn() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        assert_eq!(a7.pos(), &"a7".parse().unwrap());
        assert_eq!(a7.piece().piece_type(), &PieceType::Pawn);
        assert_eq!(a7.piece().color(), &PieceColor::Black);
    }
    #[test]
    fn it_builds_white_knight() {
        let a1 = BoardPiece::build(PieceType::Knight, PieceColor::White, "a1");
        assert_eq!(a1.pos(), &"a1".parse().unwrap());
        assert_eq!(a1.piece().piece_type(), &PieceType::Knight);
        assert_eq!(a1.piece().color(), &PieceColor::White);
    }
    #[test]
    fn it_builds_black_knight() {
        let a1 = BoardPiece::build(PieceType::Knight, PieceColor::Black, "c8");
        assert_eq!(a1.pos(), &"c8".parse().unwrap());
        assert_eq!(a1.piece().piece_type(), &PieceType::Knight);
        assert_eq!(a1.piece().color(), &PieceColor::Black);
    }
}
