use crate::board::CheckerBoard;
use crate::board_pos;
use crate::board_position::BoardPosition;
use crate::board_side_effects::BoardUpdate;
use crate::pieces::color::PieceColor;
use crate::pieces::factory::PieceFactory;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;
use std::str::FromStr;

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
        from: &BoardPosition,
        to: &BoardPosition,
    ) -> Vec<BoardUpdate> {
        if from == &board_pos!["e1"] && to == &board_pos!["c1"] {
            vec![
                BoardUpdate(board_pos!["a1"], None),
                BoardUpdate(
                    board_pos!["d1"],
                    Some(PieceFactory::build(PieceType::Rook, PieceColor::White)),
                ),
            ]
        } else if from == &board_pos!["e1"] && to == &board_pos!["g1"] {
            vec![
                BoardUpdate(board_pos!["h1"], None),
                BoardUpdate(
                    board_pos!["f1"],
                    Some(PieceFactory::build(PieceType::Rook, PieceColor::White)),
                ),
            ]
        } else {
            vec![]
        }
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

    fn put_king_in_empty_board(pos: &str) -> Vec<BoardPosition> {
        let king = BoardPiece::build(PieceType::King, PieceColor::White, pos);
        let pieces = vec![king];
        let board = CheckerBoard::with_pieces(pieces);
        board.get_possible_moves(&board_pos!(pos))
    }
}

#[cfg(test)]
mod white_castling_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_side_effects::BoardSideEffects;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use std::str::FromStr;

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
    fn cant_castle_king_side_if_piece_on_f1() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let kf1 = BoardPiece::build(PieceType::Knight, PieceColor::White, "f1");
        let board = CheckerBoard::with_pieces(vec![ke1, rh1, kf1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    #[test]
    fn cant_castle_queen_side_if_piece_on_d1() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let kd1 = BoardPiece::build(PieceType::Knight, PieceColor::White, "d1");
        let board = CheckerBoard::with_pieces(vec![ke1, ra1, kd1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
    }

    #[test]
    fn cant_castle_queen_side_if_piece_on_c1() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let rd1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let kc1 = BoardPiece::build(PieceType::Knight, PieceColor::White, "c1");
        let board = CheckerBoard::with_pieces(vec![ke1, rd1, kc1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
    }

    #[test]
    fn cant_castle_if_king_on_the_way_of_check() {
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

    #[test]
    fn cant_castle_if_king_lands_on_check() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let rd2 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "c2");
        let rf2 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "g2");
        let board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1, rd2, rf2]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    #[test]
    fn cant_castle_if_king_has_moved() {
        let kd1 = BoardPiece::build(PieceType::King, PieceColor::White, "d1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let d7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d7");
        let mut board = CheckerBoard::with_pieces(vec![kd1, ra1, rh1, d7]);
        board.move_piece(&board_pos!["d1"], &board_pos!["e1"]);
        board.move_piece(&board_pos!["d7"], &board_pos!["d5"]);

        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("c1")));
        assert!(!moves.contains(&board_pos!("g1")));
    }
    #[test]
    fn cant_castle_king_side_if_piece_on_g1() {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let kg1 = BoardPiece::build(PieceType::Knight, PieceColor::White, "g1");
        let board = CheckerBoard::with_pieces(vec![ke1, rh1, kg1]);
        let moves = board.get_possible_moves(&board_pos!("e1"));
        assert!(!moves.contains(&board_pos!("g1")));
    }

    #[test]
    fn castling_queen_side_updates_a1_to_empty() {
        let (board, _) = castle_queen_side();
        assert!(board.piece_at(&board_pos!("a1")).is_none());
    }

    #[test]
    fn castling_king_side_updates_h1_to_empty() {
        let (board, _) = castle_king_side();
        assert!(board.piece_at(&board_pos!("h1")).is_none());
    }

    #[test]
    fn castling_queen_side_updates_d1_with_rook() {
        let (board, _) = castle_queen_side();
        let rd1 = board.piece_at(&board_pos!("d1")).unwrap();
        assert_eq!(rd1.piece_type(), &PieceType::Rook);
        assert_eq!(rd1.color(), &PieceColor::White);
    }

    #[test]
    fn castling_king_side_updates_f1_with_rook() {
        let (board, _) = castle_king_side();
        let rf1 = board.piece_at(&board_pos!("f1")).unwrap();
        assert_eq!(rf1.piece_type(), &PieceType::Rook);
        assert_eq!(rf1.color(), &PieceColor::White);
    }

    #[test]
    fn castling_queen_side_move_has_correct_side_effects() {
        let (_, side_effects) = castle_queen_side();
        assert_eq!(side_effects.updatesZ[0].pos(), &board_pos!["a1"]);
        assert!(side_effects.updatesZ[0].piece().is_none());

        assert_eq!(side_effects.updatesZ[1].pos(), &board_pos!["d1"]);
        let updated_piece = side_effects.updatesZ[1].piece().clone().unwrap();
        assert_eq!(updated_piece.color(), &PieceColor::White);
        assert_eq!(updated_piece.piece_type(), &PieceType::Rook);
    }

    #[test]
    fn castling_king_side_move_has_correct_side_effects() {
        let (_, side_effects) = castle_king_side();
        assert_eq!(side_effects.updatesZ[0].pos(), &board_pos!["h1"]);
        assert!(side_effects.updatesZ[0].piece().is_none());

        assert_eq!(side_effects.updatesZ[1].pos(), &board_pos!["f1"]);
        let updated_piece = side_effects.updatesZ[1].piece().clone().unwrap();
        assert_eq!(updated_piece.color(), &PieceColor::White);
        assert_eq!(updated_piece.piece_type(), &PieceType::Rook);
    }

    fn castle_queen_side() -> (CheckerBoard, BoardSideEffects) {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let mut board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1]);
        let side_effects = board.move_piece(&board_pos!["e1"], &board_pos!["c1"]);
        (board, side_effects)
    }

    fn castle_king_side() -> (CheckerBoard, BoardSideEffects) {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let mut board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1]);
        let side_effects = board.move_piece(&board_pos!["e1"], &board_pos!["g1"]);
        (board, side_effects)
    }
}

#[cfg(test)]
mod black_castling_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_side_effects::BoardSideEffects;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use std::str::FromStr;

    #[test]
    fn cant_castle_if_there_is_no_rook_in_a_or_h() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let board = CheckerBoard::with_pieces(vec![ke8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_if_king_is_checked() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let re7 = BoardPiece::build(PieceType::Rook, PieceColor::White, "e7");
        let board = CheckerBoard::with_pieces(vec![ke8, ra8, rh8, re7]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_king_side_if_piece_on_g8() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let kg8 = BoardPiece::build(PieceType::Knight, PieceColor::Black, "g8");
        let board = CheckerBoard::with_pieces(vec![ke8, rh8, kg8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_king_side_if_piece_on_f1() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let kf8 = BoardPiece::build(PieceType::Knight, PieceColor::Black, "f8");
        let board = CheckerBoard::with_pieces(vec![ke8, rh8, kf8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_queen_side_if_piece_on_d1() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let kd8 = BoardPiece::build(PieceType::Knight, PieceColor::Black, "d8");
        let board = CheckerBoard::with_pieces(vec![ke8, ra8, kd8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
    }

    #[test]
    fn cant_castle_queen_side_if_piece_on_c1() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let rd8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let kc8 = BoardPiece::build(PieceType::Knight, PieceColor::Black, "c8");
        let board = CheckerBoard::with_pieces(vec![ke8, rd8, kc8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
    }

    #[test]
    fn cant_castle_if_king_on_the_way_of_check() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let rd7 = BoardPiece::build(PieceType::Rook, PieceColor::White, "d7");
        let rf7 = BoardPiece::build(PieceType::Rook, PieceColor::White, "f7");
        let board = CheckerBoard::with_pieces(vec![ke8, ra8, rh8, rd7, rf7]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_if_king_lands_on_check() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let rc7 = BoardPiece::build(PieceType::Rook, PieceColor::White, "c7");
        let rg7 = BoardPiece::build(PieceType::Rook, PieceColor::White, "g7");
        let board = CheckerBoard::with_pieces(vec![ke8, ra8, rh8, rc7, rg7]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn cant_castle_if_king_has_moved() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "d8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let d2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d2");
        let mut board = CheckerBoard::with_pieces(vec![ke8, ra8, rh8, d2]);
        board.move_piece(&board_pos!["d8"], &board_pos!["e8"]);
        board.move_piece(&board_pos!["d2"], &board_pos!["d4"]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(!moves.contains(&board_pos!("c8")));
        assert!(!moves.contains(&board_pos!("g8")));
    }

    #[test]
    fn can_castle_if_there_is_rook_of_same_color_in_a_or_h() {
        let ke8 = BoardPiece::build(PieceType::King, PieceColor::Black, "e8");
        let ra8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "a8");
        let rh8 = BoardPiece::build(PieceType::Rook, PieceColor::Black, "h8");
        let board = CheckerBoard::with_pieces(vec![ke8, ra8, rh8]);
        let moves = board.get_possible_moves(&board_pos!("e8"));
        assert!(moves.contains(&board_pos!("c8")));
        assert!(moves.contains(&board_pos!("g8")));
    }

    #[test]
    #[ignore]
    fn castling_queen_side_updates_a1_to_empty() {
        let (board, _) = castle_queen_side();
        assert!(board.piece_at(&board_pos!("a1")).is_none());
    }

    #[test]
    #[ignore]
    fn castling_king_side_updates_h1_to_empty() {
        let (board, _) = castle_king_side();
        assert!(board.piece_at(&board_pos!("h1")).is_none());
    }

    #[test]
    #[ignore]
    fn castling_queen_side_updates_d1_with_rook() {
        let (board, _) = castle_queen_side();
        let rd1 = board.piece_at(&board_pos!("d1")).unwrap();
        assert_eq!(rd1.piece_type(), &PieceType::Rook);
        assert_eq!(rd1.color(), &PieceColor::White);
    }

    #[test]
    #[ignore]
    fn castling_king_side_updates_f1_with_rook() {
        let (board, _) = castle_king_side();
        let rf1 = board.piece_at(&board_pos!("f1")).unwrap();
        assert_eq!(rf1.piece_type(), &PieceType::Rook);
        assert_eq!(rf1.color(), &PieceColor::White);
    }

    #[test]
    #[ignore]
    fn castling_queen_side_move_has_correct_side_effects() {
        let (_, side_effects) = castle_queen_side();
        assert_eq!(side_effects.updatesZ[0].pos(), &board_pos!["a1"]);
        assert!(side_effects.updatesZ[0].piece().is_none());

        assert_eq!(side_effects.updatesZ[1].pos(), &board_pos!["d1"]);
        let updated_piece = side_effects.updatesZ[1].piece().clone().unwrap();
        assert_eq!(updated_piece.color(), &PieceColor::White);
        assert_eq!(updated_piece.piece_type(), &PieceType::Rook);
    }

    #[test]
    #[ignore]
    fn castling_king_side_move_has_correct_side_effects() {
        let (_, side_effects) = castle_king_side();
        assert_eq!(side_effects.updatesZ[0].pos(), &board_pos!["h1"]);
        assert!(side_effects.updatesZ[0].piece().is_none());

        assert_eq!(side_effects.updatesZ[1].pos(), &board_pos!["f1"]);
        let updated_piece = side_effects.updatesZ[1].piece().clone().unwrap();
        assert_eq!(updated_piece.color(), &PieceColor::White);
        assert_eq!(updated_piece.piece_type(), &PieceType::Rook);
    }

    fn castle_queen_side() -> (CheckerBoard, BoardSideEffects) {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let mut board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1]);
        let side_effects = board.move_piece(&board_pos!["e1"], &board_pos!["c1"]);
        (board, side_effects)
    }

    fn castle_king_side() -> (CheckerBoard, BoardSideEffects) {
        let ke1 = BoardPiece::build(PieceType::King, PieceColor::White, "e1");
        let ra1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "a1");
        let rh1 = BoardPiece::build(PieceType::Rook, PieceColor::White, "h1");
        let mut board = CheckerBoard::with_pieces(vec![ke1, ra1, rh1]);
        let side_effects = board.move_piece(&board_pos!["e1"], &board_pos!["g1"]);
        (board, side_effects)
    }
}
