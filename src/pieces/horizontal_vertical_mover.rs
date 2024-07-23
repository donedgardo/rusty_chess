use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::pieces::color::PieceColor;

pub struct HorizontalVerticalMovement<'a> {
    board: &'a CheckerBoard,
    color: &'a PieceColor,
    pos: &'a BoardPosition,
}

impl<'a> HorizontalVerticalMovement<'a> {
    pub fn new(board: &'a CheckerBoard, pos: &'a BoardPosition, color: &'a PieceColor) -> Self {
        Self { board, color, pos }
    }

    pub fn get_moves(&self) -> Vec<BoardPosition> {
        let mut moves: Vec<BoardPosition> = Vec::with_capacity(16);
        self.add_horizontal_moves(&mut moves);
        self.add_vertical_moves(&mut moves);
        moves
    }
    fn add_horizontal_moves(&self, moves: &mut Vec<BoardPosition>) {
        for x in (0..self.pos.x()).rev() {
            if self.it_should_stop_moving(moves, x, self.pos.y()) {
                break;
            }
        }
        for x in self.pos.x() + 1..self.board.width() {
            if self.it_should_stop_moving(moves, x, self.pos.y()) {
                break;
            }
        }
    }

    fn add_vertical_moves(&self, moves: &mut Vec<BoardPosition>) {
        for y in self.pos.y() + 1..self.board.length() {
            if self.it_should_stop_moving(moves, self.pos.x(), y) {
                break;
            }
        }
        for y in (0..self.pos.y()).rev() {
            if self.it_should_stop_moving(moves, self.pos.x(), y) {
                break;
            }
        }
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
}
