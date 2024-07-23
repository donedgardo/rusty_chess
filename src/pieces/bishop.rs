use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::diagonal_mover::DiagonalMover;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Bishop {
    color: PieceColor,
}

impl Bishop {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for Bishop {
    fn color(&self) -> &PieceColor {
        &self.color
    }

    fn piece_type(&self) -> &PieceType {
        &PieceType::Bishop
    }

    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let diagonal_mover = DiagonalMover::new(board, from, self.color());
        diagonal_mover.get_diagonal_moves()
    }

    fn is_opponent(&self, color: &PieceColor) -> bool {
        self.color() != color
    }
}
#[cfg(test)]
mod bishop_test {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use crate::pieces::Piece;
    use std::str::FromStr;

    #[test]
    fn it_can_get_color() {
        let bishop = Bishop::new(PieceColor::White);
        assert_eq!(bishop.color(), &PieceColor::White);
    }

    #[test]
    fn is_of_type_bishop() {
        let bishop = Bishop::new(PieceColor::Black);
        assert_eq!(bishop.piece_type(), &PieceType::Bishop);
    }

    #[test]
    fn other_color_is_opponent() {
        let bishop = Bishop::new(PieceColor::Black);
        assert!(bishop.is_opponent(&PieceColor::White));
    }

    #[test]
    fn can_move_diagonally_up_left() {
        let bh1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "h1");
        let board = CheckerBoard::with_pieces(vec![bh1]);
        let possible_moves = board.get_possible_moves(&board_pos!("h1"));
        let expected_moves = ["g2", "f3", "e4", "d5", "c6", "b7", "a8"];
        for pos in expected_moves {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
    }
    #[test]
    fn can_move_diagonally_up_right() {
        let ba1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "a1");
        let board = CheckerBoard::with_pieces(vec![ba1]);
        let possible_moves = board.get_possible_moves(&board_pos!("a1"));
        let expected_moves = ["b2", "c3", "d4", "e5", "f6", "g7", "h8"];
        for pos in expected_moves {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
    }
    #[test]
    fn can_move_diagonally_down_left() {
        let bh8 = BoardPiece::build(PieceType::Bishop, PieceColor::Black, "h8");
        let board = CheckerBoard::with_pieces(vec![bh8]);
        let possible_moves = board.get_possible_moves(&board_pos!("h8"));
        let expected_moves = ["a1", "b2", "c3", "d4", "e5", "f6", "g7"];
        for pos in expected_moves {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_move_diagonally_down_right() {
        let bb8 = BoardPiece::build(PieceType::Bishop, PieceColor::Black, "b8");
        let board = CheckerBoard::with_pieces(vec![bb8]);
        let possible_moves = board.get_possible_moves(&board_pos!("b8"));
        let expected_moves = ["c7", "d6", "e5", "f4", "g3", "h2"];
        for pos in expected_moves {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn cant_move_past_same_color_piece_diagonally_up_left() {
        let bh1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "h1");
        let f3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "f3");
        let board = CheckerBoard::with_pieces(vec![bh1, f3]);
        let possible_moves = board.get_possible_moves(&board_pos!("h1"));
        assert_eq!(possible_moves, vec![board_pos!("g2")]);
    }

    #[test]
    fn cant_move_past_same_color_piece_diagonally_up_right() {
        let ba1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "a1");
        let c3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c3");
        let board = CheckerBoard::with_pieces(vec![ba1, c3]);
        let possible_moves = board.get_possible_moves(&board_pos!("a1"));
        assert_eq!(possible_moves, vec![board_pos!("b2")]);
    }
    #[test]
    fn cant_move_past_same_color_piece_diagonally_down_left() {
        let bh8 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "h8");
        let f6 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "f6");
        let board = CheckerBoard::with_pieces(vec![bh8, f6]);
        let possible_moves = board.get_possible_moves(&board_pos!("h8"));
        assert_eq!(possible_moves, vec![board_pos!("g7")]);
    }

    #[test]
    fn cant_move_past_same_color_piece_diagonally_down_right() {
        let ba8 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "a8");
        let c6 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c6");
        let board = CheckerBoard::with_pieces(vec![ba8, c6]);
        let possible_moves = board.get_possible_moves(&board_pos!("a8"));
        assert_eq!(possible_moves, vec![board_pos!("b7")]);
    }

    #[test]
    fn cant_move_up_to_different_color_piece_diagonally_up_left() {
        let bh1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "h1");
        let f3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "f3");
        let board = CheckerBoard::with_pieces(vec![bh1, f3]);
        let possible_moves = board.get_possible_moves(&board_pos!("h1"));
        for pos in ["g2", "f3"] {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
        assert_eq!(possible_moves.len(), 2);
    }

    #[test]
    fn cant_move_up_to_different_color_piece_diagonally_up_right() {
        let ba1 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "a1");
        let c3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c3");
        let board = CheckerBoard::with_pieces(vec![ba1, c3]);
        let possible_moves = board.get_possible_moves(&board_pos!("a1"));
        for pos in ["b2", "c3"] {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
        assert_eq!(possible_moves.len(), 2);
    }

    #[test]
    fn cant_move_up_to_different_color_piece_diagonally_down_left() {
        let bh8 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "h8");
        let f6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "f6");
        let board = CheckerBoard::with_pieces(vec![bh8, f6]);
        let possible_moves = board.get_possible_moves(&board_pos!("h8"));
        for pos in ["g7", "f6"] {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
        assert_eq!(possible_moves.len(), 2);
    }

    #[test]
    fn cant_move_up_to_different_color_piece_diagonally_down_right() {
        let ba8 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "a8");
        let c6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c6");
        let board = CheckerBoard::with_pieces(vec![ba8, c6]);
        let possible_moves = board.get_possible_moves(&board_pos!("a8"));
        for pos in ["b7", "c6"] {
            assert!(possible_moves.contains(&board_pos!(pos)));
        }
        assert_eq!(possible_moves.len(), 2);
    }

    #[test]
    fn cant_move_into_check() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let be2 = BoardPiece::build(PieceType::Bishop, PieceColor::White, "e2");
        let re3 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "e3");
        let board = CheckerBoard::with_pieces(vec![ke1, be2, re3]);
        let possible_moves = board.get_possible_moves(&board_pos!("e2"));
        assert!(possible_moves.is_empty());
    }
}
