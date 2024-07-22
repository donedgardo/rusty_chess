use crate::pieces::bishop::Bishop;
use crate::pieces::color::PieceColor;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::piece_type::PieceType;
use crate::pieces::rook::Rook;
use crate::pieces::Piece;

pub struct PieceFactory;

impl PieceFactory {
    pub fn build(piece_type: PieceType, color: PieceColor) -> Box<dyn Piece> {
        return match piece_type {
            PieceType::Pawn => Box::new(Pawn::new(color)),
            PieceType::Knight => Box::new(Knight::new(color)),
            PieceType::King => Box::new(King::new(color)),
            PieceType::Rook => Box::new(Rook::new(color)),
            PieceType::Bishop => Box::new(Bishop::new(color)),
        };
    }
}

#[cfg(test)]
mod piece_factory_tests {
    use crate::pieces::color::PieceColor;
    use crate::pieces::factory::PieceFactory;
    use crate::pieces::piece_type::PieceType;

    #[test]
    fn can_build_pawn_from_type() {
        let pawn = PieceFactory::build(PieceType::Pawn, PieceColor::White);
        assert_eq!(pawn.piece_type(), &PieceType::Pawn);
        assert_eq!(pawn.color(), &PieceColor::White);
    }

    #[test]
    fn can_build_knight_from_type() {
        let piece = PieceFactory::build(PieceType::Knight, PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::Knight);
        assert_eq!(piece.color(), &PieceColor::Black);
    }

    #[test]
    fn can_build_king_from_type() {
        let piece = PieceFactory::build(PieceType::King, PieceColor::White);
        assert_eq!(piece.piece_type(), &PieceType::King);
        assert_eq!(piece.color(), &PieceColor::White);
    }

    #[test]
    fn can_build_rook_from_type() {
        let pawn = PieceFactory::build(PieceType::Rook, PieceColor::Black);
        assert_eq!(pawn.piece_type(), &PieceType::Rook);
        assert_eq!(pawn.color(), &PieceColor::Black);
    }

    #[test]
    fn can_build_bishop_from_type() {
        let pawn = PieceFactory::build(PieceType::Bishop, PieceColor::White);
        assert_eq!(pawn.piece_type(), &PieceType::Bishop);
        assert_eq!(pawn.color(), &PieceColor::White);
    }
}
