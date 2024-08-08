use crate::board::CheckerBoard;
use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::horizontal_vertical_mover::HorizontalVerticalMovement;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Rook {
    color: PieceColor,
}

impl Rook {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for Rook {
    fn color(&self) -> &PieceColor {
        &self.color
    }

    fn piece_type(&self) -> &PieceType {
        &PieceType::Rook
    }

    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let horizontal_vertical_mover = HorizontalVerticalMovement::new(board, from, self.color());
        horizontal_vertical_mover.get_moves()
    }

    fn is_opponent(&self, color: &PieceColor) -> bool {
        &self.color != color
    }

    fn takes(
        &self,
        board: &CheckerBoard,
        from: &BoardPosition,
        to: &BoardPosition,
    ) -> Vec<BoardPosition> {
        let moves = self.get_all_moves(board, from);
        if moves.contains(to) && board.piece_at(to).is_some() {
            return vec![to.clone()];
        }
        vec![]
    }

    fn side_effects(
        &self,
        _board: &CheckerBoard,
        _from: &BoardPosition,
        _to: &BoardPosition,
    ) -> Vec<BoardPiece> {
        vec![]
    }
}

#[cfg(test)]
mod rook_test {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use crate::pieces::rook::Rook;
    use crate::pieces::Piece;
    use std::str::FromStr;

    #[test]
    fn can_get_color() {
        let rook = Rook::new(PieceColor::White);
        assert_eq!(rook.color(), &PieceColor::White);
    }

    #[test]
    fn has_rook_piece_type() {
        let rook = Rook::new(PieceColor::White);
        assert_eq!(rook.piece_type(), &PieceType::Rook);
    }

    #[test]
    fn can_check_opponent() {
        let rook = Rook::new(PieceColor::White);
        assert!(rook.is_opponent(&PieceColor::Black));
    }

    #[test]
    fn can_move_horizontal_from_left_side() {
        let a1 = "a1";
        let moves = spawn_rook_in_empty_board(a1);
        let horizontal_moves = ["b1", "c1", "d1", "e1", "f1", "g1", "h1"];
        for pos in horizontal_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
        assert!(!moves.contains(&board_pos!(a1)));
    }

    #[test]
    fn can_move_horizontal_from_right_side() {
        let h1 = "h1";
        let moves = spawn_rook_in_empty_board(h1);
        let horizontal_moves = ["a1", "b1", "c1", "d1", "e1", "f1", "g1"];
        for pos in horizontal_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
        assert!(!moves.contains(&board_pos!(h1)));
    }

    #[test]
    fn can_move_vertically_from_bottom_of_board() {
        let e1 = "e1";
        let moves = spawn_rook_in_empty_board(e1);
        let vertical_moves = ["e2", "e3", "e4", "e5", "e6", "e7", "e8"];
        for pos in vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
        assert!(!moves.contains(&board_pos!(e1)));
    }

    #[test]
    fn can_move_vertically_from_top_of_board() {
        let e8 = "e8";
        let moves = spawn_rook_in_empty_board(e8);
        let vertical_moves = ["e1", "e2", "e3", "e4", "e5", "e6", "e7"];
        for pos in vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
        assert!(!moves.contains(&board_pos!(e8)));
    }

    #[test]
    fn cant_horizontally_move_past_pieces_of_same_color_on_the_right() {
        let a1 = "a1";
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, a1);
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let pieces = vec![ra1, ke1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a1));
        let illegal_vertical_moves = ["e1", "f1", "g1", "h1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn cant_horizontally_move_past_pieces_of_same_color_on_the_left() {
        let h1 = "h1";
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, h1);
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let pieces = vec![rh1, ke1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(h1));
        let illegal_vertical_moves = ["a1", "b1", "c1", "d1", "e1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn cant_vertically_move_past_top_pieces_of_same_color() {
        let a1 = "a1";
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, a1);
        let ka5 = BoardPiece::build(PieceType::King, PieceColor::White, "a5");
        let pieces = vec![ra1, ka5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a1));
        let illegal_vertical_moves = ["a5", "a6", "a7", "a8"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn cant_vertically_move_past_bottom_pieces_of_same_color() {
        let a8 = "a8";
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::White, a8);
        let ka5 = BoardPiece::build(PieceType::King, PieceColor::White, "a5");
        let pieces = vec![ra8, ka5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a8));
        let illegal_vertical_moves = ["a5", "a4", "a3", "a2", "a1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_horizontally_take_up_to_pieces_of_different_color_on_the_right() {
        let a1 = "a1";
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, a1);
        let e1 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "e1");
        let pieces = vec![ra1, e1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a1));
        let illegal_vertical_moves = ["f1", "g1", "h1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
        let legal_vertical_moves = ["b1", "c1", "d1", "e1"];
        for pos in legal_vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_horizontally_take_up_to_pieces_of_different_color_on_the_left() {
        let h1 = "h1";
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, h1);
        let e1 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "e1");
        let pieces = vec![rh1, e1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(h1));
        let illegal_vertical_moves = ["a1", "b1", "c1", "d1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
        let legal_vertical_moves = ["e1", "f1", "g1"];
        for pos in legal_vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_vertically_move_up_to_top_pieces_of_different_color() {
        let a1 = "a1";
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, a1);
        let a5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a5");
        let pieces = vec![ra1, a5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a1));
        let illegal_vertical_moves = ["a6", "a7", "a8"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
        let legal_vertical_moves = ["a2", "a3", "a4", "a5"];
        for pos in legal_vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_vertically_move_up_to_bottom_pieces_of_different_color() {
        let a8 = "a8";
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::White, a8);
        let a5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a5");
        let pieces = vec![ra8, a5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!(a8));
        let illegal_vertical_moves = ["a4", "a3", "a2", "a1"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
        let legal_vertical_moves = ["a6", "a7"];
        for pos in legal_vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn can_not_move_into_check() {
        let ka1 = BoardPiece::build(PieceType::King, PieceColor::White, "a1");
        let ra2 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a2");
        let ra3 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a3");
        let pieces = vec![ka1, ra2, ra3];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        let illegal_vertical_moves = ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"];
        for pos in illegal_vertical_moves {
            assert!(!moves.contains(&board_pos!(pos)));
        }
        let legal_vertical_moves = ["a3"];
        for pos in legal_vertical_moves {
            assert!(moves.contains(&board_pos!(pos)));
        }
    }

    #[test]
    fn takes_is_empty_when_no_takes_in_move() {
        let re2 = BoardPiece::build(PieceType::Rook, PieceColor::White, "e2");
        let mut board = CheckerBoard::with_pieces(vec![re2]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("e1"));
        assert!(side_effects.takes.is_empty());
    }

    #[test]
    fn takes_has_pos_taken() {
        let re2 = BoardPiece::build(PieceType::Rook, PieceColor::White, "e2");
        let e1 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "e1");
        let mut board = CheckerBoard::with_pieces(vec![re2, e1]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("e1"));
        assert!(side_effects.takes.contains(&board_pos!("e1")));
    }

    fn spawn_rook_in_empty_board(pos: &str) -> Vec<BoardPosition> {
        let rook = BoardPiece::build(PieceType::Rook, PieceColor::White, pos);
        let pieces = vec![rook];
        let board = CheckerBoard::with_pieces(pieces);
        board.get_possible_moves(&board_pos!(pos))
    }
}
