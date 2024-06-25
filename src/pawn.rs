use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::{Piece, PieceColor, PieceType};

#[derive(Debug, Clone, PartialEq)]
pub struct Pawn {
    color: PieceColor,
}

impl Pawn {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }

    fn get_most_forward_moves(&self, from: &BoardPosition) -> Box<dyn Iterator<Item = u8>> {
        match self.color {
            PieceColor::White => Self::get_white_most_forward_moves(from),
            PieceColor::Black => Self::get_black_most_forward_moves(from),
        }
    }

    fn get_white_most_forward_moves(from: &BoardPosition) -> Box<dyn Iterator<Item = u8>> {
        if from.y() == 7 {
            return Box::new(vec![].into_iter());
        }
        let most_forward_move = if from.y() == 1 {
            from.y() + 2
        } else {
            from.y() + 1
        };
        Box::new((from.y() + 1..=most_forward_move).into_iter())
    }

    fn get_black_most_forward_moves(from: &BoardPosition) -> Box<dyn Iterator<Item = u8>> {
        if from.y() == 0 {
            return Box::new(vec![].into_iter());
        }
        let most_forward_move = if from.y() == 6 {
            from.y() - 2
        } else {
            from.y() - 1
        };
        Box::new((most_forward_move..=from.y() - 1).rev().into_iter())
    }

    fn get_possible_moves(
        board: &CheckerBoard,
        from: &&BoardPosition,
        possible_forward_moves: Box<dyn Iterator<Item = u8>>,
    ) -> Vec<BoardPosition> {
        let mut possible_moves = vec![];
        for pos in possible_forward_moves {
            if board.piece_at(&BoardPosition::new(from.x(), pos)).is_some() {
                break;
            }
            possible_moves.push(BoardPosition::new(from.x(), pos));
        }
        possible_moves
    }
}

impl Piece for Pawn {
    fn color(&self) -> &PieceColor {
        &self.color
    }
    fn piece_type(&self) -> &PieceType {
        &PieceType::Pawn
    }
    fn moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let possible_forward_moves = self.get_most_forward_moves(from);
        Self::get_possible_moves(board, &from, possible_forward_moves)
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod pawn_piece_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::pawn::Pawn;
    use crate::pieces::{Piece, PieceColor, PieceType};
    use std::str::FromStr;

    #[test]
    fn pawn_can_be_white() {
        let color = PieceColor::White;
        let pawn = Pawn::new(color);
        assert_eq!(pawn.color(), &PieceColor::White);
    }

    #[test]
    fn white_pawn_can_move_one_space_upward() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let pieces = vec![a2];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(moves.contains(&"a3".parse().unwrap()));
    }

    #[test]
    fn white_pawn_can_move_two_space_forward_from_second_row() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let pieces = vec![a2];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(moves.contains(&"a4".parse().unwrap()));
    }

    #[test]
    fn white_pawn_can_not_move_two_space_if_not_in_second_row() {
        let a3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a3");
        let pieces = vec![a3];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a3"));
        assert!(!moves.contains(&"a5".parse().unwrap()));
    }

    #[test]
    fn white_pawn_cant_move_past_a_piece() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let a3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a3");
        let pieces = vec![a2, a3];
        let board = CheckerBoard::with_pieces(pieces);

        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(!moves.contains(&board_pos!("a3")));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn white_can_not_move_outside_board() {
        let a8 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a8");
        let pieces = vec![a8];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a8"));
        assert!(moves.is_empty());
    }

    #[test]
    fn white_pawn_can_not_move_two_space_forward_from_second_row_if_path_blocked() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let a4 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a4");
        let pieces = vec![a2, a4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn pawn_can_be_black() {
        let color = PieceColor::Black;
        let pawn = Pawn::new(color);
        assert_eq!(pawn.color(), &PieceColor::Black);
    }

    #[test]
    fn black_pawn_can_move_one_downward() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let board = CheckerBoard::with_pieces(vec![a7]);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(moves.contains(&board_pos!("a6")));
    }

    #[test]
    fn black_pawn_can_move_two_spaces_downward_from_row_seven() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let board = CheckerBoard::with_pieces(vec![a7]);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(moves.contains(&board_pos!("a5")));
    }

    #[test]
    fn black_pawn_can_not_move_two_space_forward_if_not_on_row_seven() {
        let a6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a6");
        let board = CheckerBoard::with_pieces(vec![a6]);
        let moves = board.get_possible_moves(&board_pos!("a6"));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn black_pawn_cant_move_past_a_piece() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let a6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a6");
        let pieces = vec![a7, a6];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(!moves.contains(&board_pos!("a6")));
        assert!(!moves.contains(&board_pos!("a5")));
    }

    #[test]
    fn black_can_not_move_outside_board() {
        let a1 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a1");
        let pieces = vec![a1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a1"));
        assert!(moves.is_empty());
    }

    #[test]
    fn black_pawn_can_not_move_two_space_forward_from_seventh_row_if_path_blocked() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let a5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a5");
        let pieces = vec![a7, a5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(!moves.contains(&board_pos!("a5")));
    }
}
