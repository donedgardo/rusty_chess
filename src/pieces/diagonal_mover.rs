use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;

pub struct DiagonalMover<'a> {
    board: &'a CheckerBoard,
    color: &'a PieceColor,
    pos: &'a BoardPosition,
}

impl<'a> DiagonalMover<'a> {
    pub fn new(board: &'a CheckerBoard, pos: &'a BoardPosition, color: &'a PieceColor) -> Self {
        Self { board, color, pos }
    }

    pub fn get_diagonal_moves(&self) -> Vec<BoardPosition> {
        let mut moves = Vec::with_capacity(13);
        self.add_up_left_diagonal_moves(&mut moves);
        self.add_up_right_diagonal_moves(&mut moves);
        self.add_down_left_diagonal_moves(&mut moves);
        self.add_down_right_diagonal_moves(&mut moves);
        moves
    }

    fn it_should_stop_moving(&self, moves: &mut Vec<BoardPosition>, x: u8, y: u8) -> bool {
        let pos = BoardPosition::new(x, y);
        let board_piece = self.board.piece_at(&pos);
        return match board_piece {
            None => {
                moves.push(pos);
                false
            }
            Some(piece) => {
                if self.color != piece.color() {
                    moves.push(pos);
                }
                true
            }
        };
    }

    fn add_up_left_diagonal_moves(&self, moves: &mut Vec<BoardPosition>) {
        let mut x = self.pos.x();
        let mut y = self.pos.y();
        while x > 0 && y < self.board.length() - 1 {
            x = x - 1;
            y = y + 1;
            if self.it_should_stop_moving(moves, x, y) {
                break;
            }
        }
    }
    fn add_up_right_diagonal_moves(&self, moves: &mut Vec<BoardPosition>) {
        let mut x = self.pos.x();
        let mut y = self.pos.y();
        while x < self.board.width() - 1 && y < self.board.length() - 1 {
            x = x + 1;
            y = y + 1;
            if self.it_should_stop_moving(moves, x, y) {
                break;
            }
        }
    }
    fn add_down_left_diagonal_moves(&self, moves: &mut Vec<BoardPosition>) {
        let mut x = self.pos.x();
        let mut y = self.pos.y();
        while x > 0 && y > 0 {
            x = x - 1;
            y = y - 1;
            if self.it_should_stop_moving(moves, x, y) {
                break;
            }
        }
    }

    fn add_down_right_diagonal_moves(&self, moves: &mut Vec<BoardPosition>) {
        let mut x = self.pos.x();
        let mut y = self.pos.y();
        while x < self.board.width() - 1 && y > 0 {
            x = x + 1;
            y = y - 1;
            if self.it_should_stop_moving(moves, x, y) {
                break;
            }
        }
    }
}
