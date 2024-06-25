use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;
use crate::pieces::Piece;
use std::collections::HashMap;

pub struct CheckerBoard {
    pieces: HashMap<BoardPosition, Box<dyn Piece>>,
}

impl CheckerBoard {
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
        }
    }

    pub fn with_pieces(pieces: Vec<BoardPiece>) -> Self {
        let mut pieces_map = HashMap::new();
        for piece in pieces {
            pieces_map.insert(piece.pos().clone(), piece.piece().clone_box());
        }
        Self { pieces: pieces_map }
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
    pub fn move_piece(&mut self, position: &BoardPosition, new_position: &BoardPosition) {
        let piece = self.pieces.remove(position);
        match piece {
            None => {}
            Some(p) => {
                self.pieces.insert(new_position.clone(), p);
            }
        }
    }

    pub fn get_possible_moves(&self, position: &BoardPosition) -> Vec<BoardPosition> {
        let piece = self.pieces.get(position);
        match piece {
            None => vec![],
            Some(piece) => piece.moves(&self, position),
        }
    }
}

#[cfg(test)]
mod chess_board_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
    use crate::pawn::Pawn;
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
}
