use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Knight {
    color: PieceColor,
}

impl Knight {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

impl Piece for Knight {
    fn color(&self) -> &PieceColor {
        &self.color
    }
    fn piece_type(&self) -> &PieceType {
        &PieceType::Knight
    }

    fn get_all_moves(&self, board: &CheckerBoard, from: &BoardPosition) -> Vec<BoardPosition> {
        let mut moves: [(i8, i8); 8] = [(0, 0); 8];
        for (i, m) in [(1, 2), (2, 1)].iter().enumerate() {
            moves[(i * 4) + 0] = (from.x() as i8 + m.0, from.y() as i8 + m.1);
            moves[(i * 4) + 1] = (from.x() as i8 + m.0, from.y() as i8 - m.1);
            moves[(i * 4) + 2] = (from.x() as i8 - m.0, from.y() as i8 + m.1);
            moves[(i * 4) + 3] = (from.x() as i8 - m.0, from.y() as i8 - m.1);
        }
        moves
            .into_iter()
            .filter(|position| board.is_pos_valid(position))
            .map(|position| BoardPosition::new(position.0 as u8, position.1 as u8))
            .filter(|position| board.pos_is_occupied_with_color(position, self.color()))
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
mod knight_piece_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::pieces::color::PieceColor;
    use crate::pieces::knight::Knight;
    use crate::pieces::piece_type::PieceType;
    use crate::pieces::Piece;
    use std::str::FromStr;

    #[test]
    fn can_be_white() {
        let color = PieceColor::White;
        let piece = Knight::new(color);
        assert_eq!(piece.color(), &PieceColor::White);
    }

    #[test]
    fn can_be_black() {
        let color = PieceColor::Black;
        let piece = Knight::new(color);
        assert_eq!(piece.color(), &PieceColor::Black);
    }

    #[test]
    fn white_is_not_white_opponent() {
        let color = PieceColor::White;
        let piece = Knight::new(color);
        assert!(!piece.is_opponent(&PieceColor::White))
    }

    #[test]
    fn black_is_white_opponent() {
        let color = PieceColor::White;
        let piece = Knight::new(color);
        assert!(piece.is_opponent(&PieceColor::Black))
    }

    #[test]
    fn can_move_l_move_zero_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("e2")));
    }

    #[test]
    fn can_move_l_move_ninety_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("f5")));
    }

    #[test]
    fn can_move_l_move_hundred_eighty_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("c6")));
    }

    #[test]
    fn can_move_reverse_l_move_zero_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("c2")));
    }

    #[test]
    fn can_move_reverse_l_move_ninety_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("f3")));
    }

    #[test]
    fn can_move_reverse_l_move_hundred_eighty_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("e6")));
    }

    #[test]
    fn can_move_reverse_l_move_two_hundred_seventy_degree() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let pieces = vec![kd4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("b5")));
    }

    #[test]
    fn cant_move_where_same_color_piece_is_at() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let b5 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b5");
        let pieces = vec![kd4, b5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("b5")));
    }

    #[test]
    fn cant_take_where_opponent_piece_is_at() {
        let kd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let b5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b5");
        let pieces = vec![kd4, b5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("b5")));
    }

    #[test]
    fn cant_move_outside_edge_of_board() {
        let edge = vec!["a4", "h4", "d8", "d1"];
        for pos in edge {
            let piece = BoardPiece::build(PieceType::Knight, PieceColor::White, pos);
            let pieces = vec![piece];
            let board = CheckerBoard::with_pieces(pieces);
            let moves = board.get_possible_moves(&board_pos!(pos));
            assert_eq!(moves.len(), 4);
        }
    }

    #[test]
    fn cant_move_outside_corner_of_board() {
        let corner = vec!["a1", "a8", "h1", "h8"];
        for pos in corner {
            let piece = BoardPiece::build(PieceType::Knight, PieceColor::White, pos);
            let pieces = vec![piece];
            let board = CheckerBoard::with_pieces(pieces);
            let moves = board.get_possible_moves(&board_pos!(pos));
            assert_eq!(moves.len(), 2);
        }
    }

    #[test]
    fn cant_move_into_check() {
        let kd3 = BoardPiece::build(PieceType::King, PieceColor::White, "d3");
        let nd4 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d4");
        let rd5 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "d5");
        let pieces = vec![kd3, nd4, rd5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.is_empty());
    }
}
