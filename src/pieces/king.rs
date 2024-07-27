use crate::board::CheckerBoard;
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

    fn takes(&self, board: &CheckerBoard, from: &BoardPosition, to: &BoardPosition) -> Vec<BoardPosition> {
        todo!()
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

    fn put_king_in_empty_board(pos: &str) -> Vec<BoardPosition> {
        let king = BoardPiece::build(PieceType::King, PieceColor::White, pos);
        let pieces = vec![king];
        let board = CheckerBoard::with_pieces(pieces);
        board.get_possible_moves(&board_pos!(pos))
    }
}
