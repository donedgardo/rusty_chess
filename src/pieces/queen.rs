use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::diagonal_mover::DiagonalMover;
use crate::pieces::horizontal_vertical_mover::HorizontalVerticalMovement;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Queen {
    color: PieceColor,
}

impl Queen {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for Queen {
    fn color(&self) -> &PieceColor {
        &self.color
    }

    fn piece_type(&self) -> &PieceType {
        &PieceType::Queen
    }

    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let mut moves = Vec::with_capacity(29);
        let horizontal_vertical_mover = HorizontalVerticalMovement::new(board, from, self.color());
        let h_v_moves = horizontal_vertical_mover.get_moves();
        let diagonal_mover = DiagonalMover::new(board, from, self.color());
        let diagonal_moves = diagonal_mover.get_diagonal_moves();
        moves.extend(h_v_moves);
        moves.extend(diagonal_moves);
        moves
    }

    fn is_opponent(&self, color: &PieceColor) -> bool {
        self.color() != color
    }

    fn takes(&self, board: &CheckerBoard, from: &BoardPosition, to: &BoardPosition) -> Vec<BoardPosition> {
        let moves = self.get_all_moves(board, from);
        if moves.contains(to) && board.piece_at(to).is_some() {
            return vec![to.clone()];
        }
        vec![]
    }
}

#[cfg(test)]
mod queen_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use crate::pieces::queen::Queen;
    use crate::pieces::Piece;
    use std::str::FromStr;

    #[test]
    fn it_can_have_a_color() {
        let queen = Queen::new(PieceColor::White);
        assert_eq!(queen.color(), &PieceColor::White);
    }
    #[test]
    fn it_has_queen_type() {
        let queen = Queen::new(PieceColor::Black);
        assert_eq!(queen.piece_type(), &PieceType::Queen);
    }

    #[test]
    fn opposite_color_is_opponent() {
        let queen = Queen::new(PieceColor::Black);
        assert!(queen.is_opponent(&PieceColor::White))
    }

    #[test]
    fn can_move_vertically() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let pieces = vec![qd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        for pos in ["d1", "d2", "d3", "d5", "d6", "d7", "d8"] {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }
    #[test]
    fn can_move_horizontally() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let pieces = vec![qd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        for pos in ["a4", "b4", "c4", "e4", "f4", "g4", "h4"] {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_move_diagonally() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let pieces = vec![qd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        for pos in [
            "a1", "b2", "c3", "e5", "f6", "g7", "h8", "a7", "b6", "c5", "e3", "f2", "g1",
        ] {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }
    #[test]
    fn cant_move_past_same_color() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d3");
        let pieces = vec![qd4, d3];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("d3")))
    }
    #[test]
    fn can_move_up_to_different_color() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d3");
        let pieces = vec![qd4, d3];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("d2")))
    }

    #[test]
    fn cant_move_into_check() {
        let qd4 = BoardPiece::build(PieceType::Queen, PieceColor::White, "d4");
        let rd5 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "d5");
        let kd3 = BoardPiece::build(PieceType::King, PieceColor::White, "d3");
        let pieces = vec![qd4, kd3, rd5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert_eq!(moves, vec![board_pos!("d5")])
    }

    #[test]
    fn takes_is_empty_when_no_takes_in_move() {
        let qe2 = BoardPiece::build(PieceType::Queen, PieceColor::White, "e2");
        let mut board = CheckerBoard::with_pieces(vec![qe2]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("d3"));
        assert!(side_effects.takes.is_empty());
    }

    #[test]
    fn takes_has_pos_taken() {
        let qe2 = BoardPiece::build(PieceType::Queen, PieceColor::White, "e2");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d3");
        let mut board = CheckerBoard::with_pieces(vec![qe2, d3]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("d3"));
        assert!(side_effects.takes.contains(&board_pos!("d3")));
    }
}
