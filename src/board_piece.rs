use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::factory::PieceFactory;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

pub struct BoardPiece(BoardPosition, Box<dyn Piece>);

impl BoardPiece {
    pub fn build(piece_type: PieceType, color: PieceColor, position: &str) -> Self {
        BoardPiece(
            position.parse().unwrap(),
            PieceFactory::build(piece_type, color),
        )
    }

    pub fn pos(&self) -> &BoardPosition {
        &self.0
    }
    pub fn piece(&self) -> &Box<dyn Piece> {
        &self.1
    }
}

#[cfg(test)]
mod board_piece_test {
    use super::*;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;

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
    #[test]
    fn it_builds_black_king() {
        let d8 = BoardPiece::build(PieceType::King, PieceColor::Black, "d8");
        assert_eq!(d8.pos(), &"d8".parse().unwrap());
        assert_eq!(d8.piece().piece_type(), &PieceType::King);
        assert_eq!(d8.piece().color(), &PieceColor::Black);
    }
}
