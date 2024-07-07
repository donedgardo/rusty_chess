use crate::board_move::BoardMove;
use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;
use crate::pieces::Piece;
use std::collections::HashMap;

pub struct CheckerBoard {
    moves: Vec<BoardMove>,
    pieces: HashMap<BoardPosition, Box<dyn Piece>>,
}

impl CheckerBoard {
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
            moves: vec![],
        }
    }

    pub fn with_pieces(pieces: Vec<BoardPiece>) -> Self {
        let mut pieces_map = HashMap::new();
        for piece in pieces {
            pieces_map.insert(piece.pos().clone(), piece.piece().clone_box());
        }
        Self {
            pieces: pieces_map,
            moves: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }
    pub fn spawn(&mut self, position: &BoardPosition, piece: Box<dyn Piece>) {
        self.pieces.insert(position.clone(), piece);
    }
    pub fn despawn(&mut self, position: &BoardPosition) {
        self.pieces.remove(position);
    }
    pub fn piece_at(&self, position: &BoardPosition) -> Option<&Box<dyn Piece>> {
        self.pieces.get(position)
    }
    pub fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) {
        let piece = self.pieces.remove(from);
        if let Some(p) = piece {
            self.moves.push(BoardMove::new(
                p.piece_type().clone(),
                from.clone(),
                to.clone(),
            ));
            self.pieces.insert(to.clone(), p);
        }
    }

    pub fn get_possible_moves(&self, position: &BoardPosition) -> Vec<BoardPosition> {
        let piece = self.pieces.get(position);
        match piece {
            None => vec![],
            Some(piece) => piece.moves(&self, position),
        }
    }

    pub fn get_last_move(&self) -> Option<&BoardMove> {
        self.moves.last()
    }

    pub fn is_last_row_for_white(&self, board_position: &BoardPosition) -> bool {
        board_position.y() + 1 == self.length()
    }

    pub fn is_last_row_for_black(&self, board_position: &BoardPosition) -> bool {
        board_position.y() == 0
    }

    pub fn is_far_left_side(&self, from: &BoardPosition) -> bool {
        from.x() == 0
    }

    pub fn is_far_right_side(&self, from: &BoardPosition) -> bool {
        from.x() + 1 == self.width()
    }
    pub fn width(&self) -> u8 {
        8
    }
    pub fn length(&self) -> u8 {
        8
    }
}

#[cfg(test)]
mod chess_board_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::{PieceColor, PieceType};
    use std::str::FromStr;

    #[test]
    fn new_board_is_empty() {
        let board = CheckerBoard::new();
        assert_eq!(board.is_empty(), true);
    }

    #[test]
    fn it_can_spawn_pieces() {
        let mut board = CheckerBoard::new();
        let pawn = Box::new(Pawn::new(PieceColor::Black));
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, pawn.clone());
        let piece = board.piece_at(&position).unwrap();
        assert_eq!(piece.color(), &PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::Pawn);
        assert_eq!(board.is_empty(), false);
    }

    #[test]
    fn it_can_despawn_pieces() {
        let mut board = CheckerBoard::new();
        let pawn = Pawn::new(PieceColor::Black);
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, Box::new(pawn));
        board.despawn(&position);
        let piece = board.piece_at(&position);
        assert!(piece.is_none());
        assert_eq!(board.is_empty(), true);
    }

    #[test]
    fn it_can_move_pieces() {
        let mut board = CheckerBoard::new();
        let pawn = Box::new(Pawn::new(PieceColor::Black));
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, pawn.clone());
        let new_position = BoardPosition::new(1, 1);
        board.move_piece(&position, &new_position);
        let piece_at_old_pos = board.piece_at(&position);
        assert!(piece_at_old_pos.is_none());
        let piece_at_new_pos = board.piece_at(&new_position).unwrap();
        assert_eq!(piece_at_new_pos.piece_type(), &PieceType::Pawn);
        assert_eq!(piece_at_new_pos.color(), &PieceColor::Black);
    }

    #[test]
    fn it_can_create_board_with_pieces() {
        let b2 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "b2");
        let pieces = vec![b2];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(!board.is_empty());
        let piece = board.piece_at(&"b2".parse().unwrap()).unwrap();
        assert_eq!(piece.color(), &PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::Pawn);
    }

    #[test]
    fn it_can_get_moves_from_pos() {
        let b2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b2");
        let pieces = vec![b2];
        let board = CheckerBoard::with_pieces(pieces);
        let moves = board.get_possible_moves(&board_pos!("b2"));
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn it_can_get_last_move() {
        let b2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "b2");
        let pieces = vec![b2];
        let mut board = CheckerBoard::with_pieces(pieces);
        board.move_piece(&board_pos!["b2"], &board_pos!["b3"]);
        let last_move = board.get_last_move().unwrap();
        assert_eq!(last_move.from(), &board_pos!["b2"]);
        assert_eq!(last_move.to(), &board_pos!["b3"]);
    }
    #[test]
    fn non_eight_row_is_not_last_row_for_white() {
        let board = CheckerBoard::new();
        for row in 0..7 {
            let pos = BoardPosition::new(0, row);
            assert!(!board.is_last_row_for_white(&pos));
        }
    }

    #[test]
    fn eight_row_is_last_row_for_white() {
        let board = CheckerBoard::new();
        assert!(board.is_last_row_for_white(&board_pos!("a8")));
    }
    #[test]
    fn non_first_row_is_not_last_row_for_black() {
        let board = CheckerBoard::new();
        for row in 1..=7 {
            let pos = BoardPosition::new(0, row);
            assert!(!board.is_last_row_for_black(&pos));
        }
    }

    #[test]
    fn first_row_is_last_row_for_black() {
        let board = CheckerBoard::new();
        assert!(board.is_last_row_for_black(&board_pos!("a1")));
    }

    #[test]
    fn non_first_column_is_not_far_left_side() {
        let board = CheckerBoard::new();
        for column in 1..=7 {
            let pos = BoardPosition::new(column, 0);
            assert!(!board.is_far_left_side(&pos));
        }
    }
    #[test]
    fn first_column_is_far_left_side() {
        let board = CheckerBoard::new();
        assert!(board.is_far_left_side(&board_pos!("a1")));
    }

    #[test]
    fn non_eight_column_is_not_far_right_side() {
        let board = CheckerBoard::new();
        for column in 0..7 {
            let pos = BoardPosition::new(column, 0);
            assert!(!board.is_far_right_side(&pos));
        }
    }
    #[test]
    fn eight_column_is_far_right_side() {
        let board = CheckerBoard::new();
        assert!(board.is_far_right_side(&board_pos!("h1")));
    }

    #[test]
    fn board_has_eight_width() {
        let board = CheckerBoard::new();
        assert_eq!(board.width(), 8);
    }

    #[test]
    fn board_has_eight_length() {
        let board = CheckerBoard::new();
        assert_eq!(board.length(), 8);
    }
}
