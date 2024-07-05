use crate::board::{BoardMove, CheckerBoard};
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

    fn get_possible_take_positions(
        &self,
        from: &BoardPosition,
    ) -> Box<dyn Iterator<Item = BoardPosition>> {
        match self.color {
            PieceColor::White => Self::get_white_possible_take_positions(from),
            PieceColor::Black => Self::get_black_possible_take_positions(from),
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

    fn get_white_possible_take_positions(
        from: &BoardPosition,
    ) -> Box<dyn Iterator<Item = BoardPosition>> {
        if from.y() == 7 {
            return Box::new(vec![].into_iter());
        }
        if from.x() == 0 {
            return Box::new(vec![BoardPosition::new(from.x() + 1, from.y() + 1)].into_iter());
        } else if from.x() == 7 {
            return Box::new(vec![BoardPosition::new(from.x() - 1, from.y() + 1)].into_iter());
        }
        return Box::new(
            vec![
                BoardPosition::new(from.x() - 1, from.y() + 1),
                BoardPosition::new(from.x() + 1, from.y() + 1),
            ]
            .into_iter(),
        );
    }
    fn get_black_possible_take_positions(
        from: &BoardPosition,
    ) -> Box<dyn Iterator<Item = BoardPosition>> {
        if from.y() == 0 {
            return Box::new(vec![].into_iter());
        }
        if from.x() == 0 {
            return Box::new(vec![BoardPosition::new(from.x() + 1, from.y() - 1)].into_iter());
        } else if from.x() == 7 {
            return Box::new(vec![BoardPosition::new(from.x() - 1, from.y() - 1)].into_iter());
        }
        return Box::new(
            vec![
                BoardPosition::new(from.x() - 1, from.y() - 1),
                BoardPosition::new(from.x() + 1, from.y() - 1),
            ]
            .into_iter(),
        );
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
    fn get_white_possible_en_passant_takes(
        &self,
        from: &BoardPosition,
        last_move: Option<&BoardMove>,
    ) -> Option<BoardPosition> {
        return match last_move {
            None => None,
            Some(board_move) => {
                if from.x().abs_diff(board_move.to().x()) != 1
                    || board_move.from().y().abs_diff(board_move.to().y()) != 2
                {
                    return None;
                }
                return Some(BoardPosition::new(
                    board_move.to().x(),
                    board_move.to().y() + 1,
                ));
            }
        };
    }

    fn get_possible_moves(
        &self,
        board: &CheckerBoard,
        from: &&BoardPosition,
        possible_forward_moves: Box<dyn Iterator<Item = u8>>,
        possible_takes: Box<dyn Iterator<Item = BoardPosition>>,
    ) -> Vec<BoardPosition> {
        let mut possible_moves = vec![];
        for forward_move in possible_forward_moves {
            if board
                .piece_at(&BoardPosition::new(from.x(), forward_move))
                .is_some()
            {
                break;
            }
            possible_moves.push(BoardPosition::new(from.x(), forward_move));
        }
        for possible_take in possible_takes {
            if let Some(piece) = board.piece_at(&possible_take) {
                if self.is_opponent(piece.color()) {
                    possible_moves.push(possible_take);
                }
            }
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
        let possible_takes = self.get_possible_take_positions(from);
        // let _possible_en_passant = self.get_possible_en_passant(from, board.get_last_move());
        self.get_possible_moves(board, &from, possible_forward_moves, possible_takes)
            .into_iter()
            .collect()
    }
    fn is_opponent(&self, color: &PieceColor) -> bool {
        &self.color != color
    }
}

#[cfg(test)]
mod white_pawn_tests {
    use crate::board::{BoardMove, CheckerBoard};
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
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
    fn white_is_not_opponent() {
        let color = PieceColor::White;
        let pawn = Pawn::new(color);
        assert!(!pawn.is_opponent(&PieceColor::White));
    }

    #[test]
    fn black_is_opponent() {
        let color = PieceColor::White;
        let pawn = Pawn::new(color);
        assert!(pawn.is_opponent(&PieceColor::Black));
    }

    #[test]
    fn can_move_one_space_upward() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let pieces = vec![a2];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(moves.contains(&"a3".parse().unwrap()));
    }

    #[test]
    fn can_move_two_space_forward_from_second_row() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let pieces = vec![a2];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(moves.contains(&"a4".parse().unwrap()));
    }

    #[test]
    fn can_not_move_two_space_if_not_in_second_row() {
        let a3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a3");
        let pieces = vec![a3];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a3"));
        assert!(!moves.contains(&"a5".parse().unwrap()));
    }

    #[test]
    fn cant_move_past_a_piece() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let a3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a3");
        let pieces = vec![a2, a3];
        let board = CheckerBoard::with_pieces(pieces);

        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(!moves.contains(&board_pos!("a3")));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn can_not_move_outside_board() {
        let a8 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a8");
        let pieces = vec![a8];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a8"));
        assert!(moves.is_empty());
    }

    #[test]
    fn can_not_move_two_space_forward_from_second_row_if_path_blocked() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let a4 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a4");
        let pieces = vec![a2, a4];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn possible_take_positions_in_a() {
        let a2 = Pawn::new(PieceColor::White);
        let possible_takes = a2
            .get_possible_take_positions(&board_pos!("a2"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("b3")]);
    }

    #[test]
    fn possible_take_positions_in_h() {
        let h2 = Pawn::new(PieceColor::White);
        let possible_takes = h2
            .get_possible_take_positions(&board_pos!("h2"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("g3")]);
    }

    #[test]
    fn possible_take_positions_in_row_8() {
        let a8 = Pawn::new(PieceColor::White);
        let possible_takes = a8
            .get_possible_take_positions(&board_pos!("a8"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![]);
    }

    #[test]
    fn possible_take_positions_in_middle() {
        let d4 = Pawn::new(PieceColor::White);
        let possible_takes = d4
            .get_possible_take_positions(&board_pos!("d4"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("c5"), board_pos!("e5")]);
    }

    #[test]
    fn can_take_forward_right() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let b3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b3");
        let pieces = vec![a2, b3];
        let board = CheckerBoard::with_pieces(pieces);
        let a2moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(a2moves.contains(&board_pos!("b3")));
    }

    #[test]
    fn can_take_forward_left() {
        let b2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b2");
        let a3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a3");
        let pieces = vec![b2, a3];
        let board = CheckerBoard::with_pieces(pieces);
        let b2moves = board.get_possible_moves(&board_pos!("b2"));
        assert!(b2moves.contains(&board_pos!("a3")));
    }

    #[test]
    fn cant_take_if_no_piece_in_upper_side() {
        let c2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c2");
        let pieces = vec![c2];
        let board = CheckerBoard::with_pieces(pieces);
        let c2moves = board.get_possible_moves(&board_pos!("c2"));
        assert!(!c2moves.contains(&board_pos!("b3")));
        assert!(!c2moves.contains(&board_pos!("d3")));
    }
    #[test]
    fn can_take_upward_left_and_right() {
        let d4 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d4");
        let c5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c5");
        let e5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "e5");
        let pieces = vec![d4, e5, c5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(moves.contains(&board_pos!("c5")));
        assert!(moves.contains(&board_pos!("e5")));
    }

    #[test]
    fn cant_take_white_piece() {
        let d4 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d4");
        let c5 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c5");
        let pieces = vec![d4, c5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("d4"));
        assert!(!moves.contains(&board_pos!("c5")));
    }

    #[test]
    fn cant_en_passant_if_last_move_is_none() {
        let d5 = Pawn::new(PieceColor::White);
        let possible_takes = d5.get_white_possible_en_passant_takes(&board_pos!("d5"), None);
        assert!(possible_takes.is_none());
    }

    #[test]
    fn cant_en_passant_if_last_move_is_not_adjacent() {
        let d5 = Pawn::new(PieceColor::White);
        let last_move = BoardMove::new(board_pos!("a7"), board_pos!("a5"));
        let possible_takes =
            d5.get_white_possible_en_passant_takes(&board_pos!("d5"), Some(&last_move));
        assert!(possible_takes.is_none());
    }

    #[test]
    fn cant_en_passant_if_last_move_is_not_two_squared_move() {
        let d5 = Pawn::new(PieceColor::White);
        let last_move = BoardMove::new(board_pos!("c6"), board_pos!("c5"));
        let possible_takes =
            d5.get_white_possible_en_passant_takes(&board_pos!("d5"), Some(&last_move));
        assert!(possible_takes.is_none());
    }

    #[test]
    #[ignore]
    fn cant_en_passant_if_last_move_is_not_a_pawn() {
        let d5 = Pawn::new(PieceColor::White);
        let last_move = BoardMove::new(board_pos!("c7"), board_pos!("c5"));
        let possible_takes =
            d5.get_white_possible_en_passant_takes(&board_pos!("d5"), Some(&last_move));
        assert!(possible_takes.is_none());
    }

    #[test]
    fn can_en_passant_if_last_move_is_two_squared_move() {
        let d5 = Pawn::new(PieceColor::White);
        let last_move = BoardMove::new(board_pos!("c7"), board_pos!("c5"));
        let possible_takes =
            d5.get_white_possible_en_passant_takes(&board_pos!("d5"), Some(&last_move));
        assert_eq!(possible_takes, Some(board_pos!("c6")));
    }

    #[test]
    fn cant_en_passant_if_black_didnt_pass_from_7_row() {
        let b6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b6");
        let c5 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c5");
        let pieces = vec![b6, c5];
        let mut board = CheckerBoard::with_pieces(pieces);
        board.move_piece(&board_pos!["b6"], &board_pos!["b5"]);
        let moves = board.get_possible_moves(&board_pos!("c5"));
        assert!(!moves.contains(&board_pos!("b6")));
    }

    #[test]
    #[ignore]
    fn can_en_passant_if_black_pass_from_7_row() {
        let c5 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c5");
        let b6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b7");
        let pieces = vec![b6, c5];
        let mut board = CheckerBoard::with_pieces(pieces);
        board.move_piece(&board_pos!["b7"], &board_pos!["b5"]);
        let moves = board.get_possible_moves(&board_pos!("c5"));
        assert!(moves.contains(&board_pos!("b6")));
    }
}

#[cfg(test)]
mod black_pawn_tests {
    use super::*;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use std::str::FromStr;
    #[test]
    fn pawn_can_be_black() {
        let color = PieceColor::Black;
        let pawn = Pawn::new(color);
        assert_eq!(pawn.color(), &PieceColor::Black);
    }

    #[test]
    fn black_is_not_opponent() {
        let color = PieceColor::Black;
        let pawn = Pawn::new(color);
        assert!(!pawn.is_opponent(&PieceColor::Black));
    }

    #[test]
    fn white_is_opponent() {
        let color = PieceColor::Black;
        let pawn = Pawn::new(color);
        assert!(pawn.is_opponent(&PieceColor::White));
    }

    #[test]
    fn can_move_one_downward() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let board = CheckerBoard::with_pieces(vec![a7]);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(moves.contains(&board_pos!("a6")));
    }

    #[test]
    fn can_move_two_spaces_downward_from_row_seven() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let board = CheckerBoard::with_pieces(vec![a7]);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(moves.contains(&board_pos!("a5")));
    }

    #[test]
    fn cant_move_two_space_forward_if_not_on_row_seven() {
        let a6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a6");
        let board = CheckerBoard::with_pieces(vec![a6]);
        let moves = board.get_possible_moves(&board_pos!("a6"));
        assert!(!moves.contains(&board_pos!("a4")));
    }

    #[test]
    fn cant_move_past_a_piece() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let a6 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a6");
        let pieces = vec![a7, a6];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(!moves.contains(&board_pos!("a6")));
        assert!(!moves.contains(&board_pos!("a5")));
    }

    #[test]
    fn cant_move_outside_board() {
        let a1 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a1");
        let pieces = vec![a1];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a1"));
        assert!(moves.is_empty());
    }

    #[test]
    fn can_not_move_two_space_forward_from_seventh_row_if_path_blocked() {
        let a7 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a7");
        let a5 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a5");
        let pieces = vec![a7, a5];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("a7"));
        assert!(!moves.contains(&board_pos!("a5")));
    }
    #[test]
    fn possible_take_positions_in_a() {
        let a7 = Pawn::new(PieceColor::Black);
        let possible_takes = a7
            .get_possible_take_positions(&board_pos!("a7"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("b6")]);
    }

    #[test]
    fn possible_take_positions_in_h() {
        let h7 = Pawn::new(PieceColor::Black);
        let possible_takes = h7
            .get_possible_take_positions(&board_pos!("h7"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("g6")]);
    }

    #[test]
    fn possible_take_positions_in_row_1() {
        let b1 = Pawn::new(PieceColor::Black);
        let possible_takes = b1
            .get_possible_take_positions(&board_pos!("b1"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![]);
    }

    #[test]
    fn possible_take_positions_in_middle() {
        let d5 = Pawn::new(PieceColor::Black);
        let possible_takes = d5
            .get_possible_take_positions(&board_pos!("d5"))
            .collect::<Vec<BoardPosition>>();
        assert_eq!(possible_takes, vec![board_pos!("c4"), board_pos!("e4")]);
    }

    #[test]
    fn pawn_take_bottom_right() {
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a2");
        let b1 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b1");
        let pieces = vec![a2, b1];
        let board = CheckerBoard::with_pieces(pieces);
        let a2moves = board.get_possible_moves(&board_pos!("a2"));
        assert!(a2moves.contains(&board_pos!("b1")));
    }

    #[test]
    fn pawn_take_bottom_left() {
        let b3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b3");
        let a2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "a2");
        let pieces = vec![b3, a2];
        let board = CheckerBoard::with_pieces(pieces);
        let b3moves = board.get_possible_moves(&board_pos!("b3"));
        assert!(b3moves.contains(&board_pos!("a2")));
    }

    #[test]
    fn pawn_cant_take_if_no_piece_in_bottom_side() {
        let c4 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c4");
        let pieces = vec![c4];
        let board = CheckerBoard::with_pieces(pieces);
        let c4moves = board.get_possible_moves(&board_pos!("c4"));
        assert!(!c4moves.contains(&board_pos!("b3")));
        assert!(!c4moves.contains(&board_pos!("d3")));
    }
    #[test]
    fn pawn_can_take_bottom_left_and_right() {
        let c4 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c4");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "d3");
        let b3 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b3");
        let pieces = vec![c4, d3, b3];
        let board = CheckerBoard::with_pieces(pieces);
        let c4moves = board.get_possible_moves(&board_pos!("c4"));
        assert!(c4moves.contains(&board_pos!("b3")));
        assert!(c4moves.contains(&board_pos!("d3")));
    }
    #[test]
    fn pawn_cant_take_black_piece() {
        let c4 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "c4");
        let d3 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d3");
        let pieces = vec![d3, c4];
        let board = CheckerBoard::with_pieces(pieces);
        let c4moves = board.get_possible_moves(&board_pos!("c4"));
        assert!(!c4moves.contains(&board_pos!("d3")));
    }
}
