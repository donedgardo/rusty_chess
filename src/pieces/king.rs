use crate::board::CheckerBoard;
use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct King {
    color: PieceColor,
}

impl King {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for King {
    fn color(&self) -> &PieceColor {
        &self.color
    }

    fn piece_type(&self) -> &PieceType {
        &PieceType::King
    }

    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let eight_directions: [(i8, i8); 8] = [
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (1, 1),
            (-1, -1),
            (1, -1),
        ];

        eight_directions
            .iter()
            .map(|direction| (from.x() as i8 + direction.0, from.y() as i8 + direction.1))
            .filter(|direction| board.is_pos_valid(direction))
            .map(|direction| BoardPosition::new(direction.0 as u8, direction.1 as u8))
            .filter(|position| board.pos_is_occupied_with_color(position, self.color()))
            .into_iter()
            .collect()
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
mod king_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
    use crate::pieces::color::PieceColor;
    use crate::pieces::king::King;
    use crate::pieces::piece_type::PieceType;
    use crate::pieces::Piece;
    use std::str::FromStr;

    #[test]
    fn can_be_white() {
        let king = King::new(PieceColor::White);
        assert_eq!(king.color(), &PieceColor::White);
    }
    #[test]
    fn can_be_black() {
        let king = King::new(PieceColor::Black);
        assert_eq!(king.color(), &PieceColor::Black);
    }
    #[test]
    fn is_of_type_king() {
        let king = King::new(PieceColor::Black);
        assert_eq!(king.piece_type(), &PieceType::King);
    }
    #[test]
    fn white_is_black_opponent() {
        let king = King::new(PieceColor::Black);
        assert!(king.is_opponent(&PieceColor::White));
    }
    #[test]
    fn black_is_white_opponent() {
        let king = King::new(PieceColor::White);
        assert!(king.is_opponent(&PieceColor::Black));
    }
    #[test]
    fn can_move_up() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("d5")));
    }

    #[test]
    fn can_move_down() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("d3")));
    }

    #[test]
    fn can_move_left() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("c4")));
    }

    #[test]
    fn can_move_right() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("e4")));
    }

    #[test]
    fn can_move_upward_left() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("c5")));
    }

    #[test]
    fn can_move_upward_right() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("e5")));
    }

    #[test]
    fn can_move_downward_left() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("c3")));
    }

    #[test]
    fn can_move_downward_right() {
        let moves = put_king_in_empty_board("d4");
        assert!(moves.contains(&board_pos!("e3")));
    }

    #[test]
    fn cant_move_off_edge() {
        let edges = vec!["a4", "d1", "h4", "d8"];
        for edge in edges {
            let moves = put_king_in_empty_board(edge);
            assert_eq!(moves.len(), 5);
        }
    }
    #[test]
    fn cant_move_off_corner() {
        let edges = vec!["a1", "a8", "h1", "h8"];
        for edge in edges {
            let moves = put_king_in_empty_board(edge);
            assert_eq!(moves.len(), 3);
        }
    }

    #[test]
    fn cant_take_same_color() {
        let kd4 = BoardPiece::build(PieceType::King, PieceColor::White, "d4");
        let d5 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d5");
        let pieces = vec![kd4, d5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("d5")));
    }

    #[test]
    fn can_take_other_color() {
        let kd4 = BoardPiece::build(PieceType::King, PieceColor::White, "d4");
        let d5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d5");
        let pieces = vec![kd4, d5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("d5")));
    }

    #[test]
    fn cant_move_into_check() {
        let kd4 = BoardPiece::build(PieceType::King, PieceColor::White, "d4");
        let d5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d5");
        let pieces = vec![kd4, d5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("c4")));
        assert!(!moves.contains(&board_pos!("e4")));
    }

    #[test]
    fn takes_is_empty_when_no_takes_in_move() {
        let ke2 = BoardPiece::build(PieceType::King, PieceColor::White, "e2");
        let mut board = CheckerBoard::with_pieces(vec![ke2]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("d3"));
        assert!(side_effects.takes.is_empty());
    }

    #[test]
    fn takes_has_pos_taken() {
        let ke2 = BoardPiece::build(PieceType::King, PieceColor::White, "e2");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d3");
        let mut board = CheckerBoard::with_pieces(vec![ke2, d3]);
        let side_effects = board.move_piece(&board_pos!("e2"), &board_pos!("d3"));
        assert!(side_effects.takes.contains(&board_pos!("d3")));
    }

    #[test]
    fn cant_castle_if_there_is_no_rook_in_a_or_h() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let board = CheckerBoard::with_pieces(vec![ke1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    #[test]
    fn can_castle_if_there_is_rook_of_same_color_in_a_or_h() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(moves.contains(&board_pos!("c1")));
        assert!(moves.contains(&board_pos!("g1")));
    }

    #[test]
    fn cant_castle_if_king_is_checked() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let re2 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "e2");
        let board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1, re2]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    #[test]
    #[ignore]
    fn cant_castle_if_king_on_the_way() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let rd2 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "d2");
        let rf2 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "f2");
        let board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1, rd2, rf2]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    fn put_king_in_empty_board(pos: &str) -> Vec<BoardPosition> {
        let king = BoardPiece::build(PieceType::King, PieceColor::White, pos);
        let pieces = vec![king];
        let board = CheckerBoard::with_pieces(pieces);
        board.get_possible_moves(&board_pos!(pos))
    }
}
