use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;

struct DiagonalMover;

impl DiagonalMover {
    pub fn it_should_stop_moving(
        board: &CheckerBoard,
        moves: &mut Vec<BoardPosition>,
        x: u8,
        y: u8,
        color: &PieceColor,
    ) -> bool {
        let pos = BoardPosition::new(x, y);
        let board_piece = board.piece_at(&pos);
        return match board_piece {
            None => {
                moves.push(pos);
                false
            }
            Some(piece) => {
                if color != piece.color() {
                    moves.push(pos);
                }
                true
            }
        };
    }

    pub fn add_up_left_diagonal_moves(
        board: &CheckerBoard,
        pos: &BoardPosition,
        moves: &mut Vec<BoardPosition>,
        color: &PieceColor,
    ) {
        let mut x = pos.x();
        let mut y = pos.y();
        while x > 0 && y < board.length() - 1 {
            x = x - 1;
            y = y + 1;
            if Self::it_should_stop_moving(board, moves, x, y, color) {
                break;
            }
        }
    }
    pub fn add_up_right_diagonal_moves(
        board: &CheckerBoard,
        pos: &BoardPosition,
        moves: &mut Vec<BoardPosition>,
        color: &PieceColor,
    ) {
        let mut x = pos.x();
        let mut y = pos.y();
        while x < board.width() - 1 && y < board.length() - 1 {
            x = x + 1;
            y = y + 1;
            if Self::it_should_stop_moving(board, moves, x, y, color) {
                break;
            }
        }
    }
    pub fn add_down_left_diagonal_moves(
        board: &CheckerBoard,
        pos: &BoardPosition,
        moves: &mut Vec<BoardPosition>,
        color: &PieceColor,
    ) {
        let mut x = pos.x();
        let mut y = pos.y();
        while x > 0 && y > 0 {
            x = x - 1;
            y = y - 1;
            if Self::it_should_stop_moving(board, moves, x, y, color) {
                break;
            }
        }
    }

    pub fn add_down_right_diagonal_moves(
        board: &CheckerBoard,
        pos: &BoardPosition,
        moves: &mut Vec<BoardPosition>,
        color: &PieceColor,
    ) {
        let mut x = pos.x();
        let mut y = pos.y();
        while x < board.width() - 1 && y > 0 {
            x = x + 1;
            y = y - 1;
            if Self::it_should_stop_moving(board, moves, x, y, color) {
                break;
            }
        }
    }

    pub fn get_diagonal_moves(
        board: &CheckerBoard,
        from: &BoardPosition,
        color: &PieceColor,
    ) -> Vec<BoardPosition> {
        let mut moves = Vec::with_capacity(13);
        DiagonalMover::add_up_left_diagonal_moves(board, from, &mut moves, color);
        DiagonalMover::add_up_right_diagonal_moves(board, from, &mut moves, color);
        DiagonalMover::add_down_left_diagonal_moves(board, from, &mut moves, color);
        DiagonalMover::add_down_right_diagonal_moves(board, from, &mut moves, color);
        moves
    }
}

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
        DiagonalMover::get_diagonal_moves(board, from, self.color())
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
