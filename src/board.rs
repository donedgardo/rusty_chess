use crate::board_move::BoardMove;
use crate::board_piece::BoardPiece;
use crate::board_position::BoardPosition;
use crate::board_side_effects::BoardSideEffects;
use crate::pieces::color::PieceColor;
use crate::pieces::factory::PieceFactory;
use crate::pieces::piece_type::PieceType;
use crate::pieces::Piece;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CheckerBoard {
    moves: Vec<BoardMove>,
    pieces: HashMap<BoardPosition, Box<dyn Piece>>,
}

impl CheckerBoard {
    pub fn new() -> Self {
        Self {
            pieces: HashMap::with_capacity(32),
            moves: vec![],
        }
    }

    pub fn default() -> Self {
        let mut board = Self {
            pieces: HashMap::with_capacity(32),
            moves: vec![],
        };

        for x in 0..board.width() {
            board.spawn(
                &BoardPosition::new(x, 1),
                PieceType::Pawn,
                PieceColor::White,
            );
            board.spawn(
                &BoardPosition::new(x, 6),
                PieceType::Pawn,
                PieceColor::Black,
            );
        }
        for x in [0, 7] {
            board.spawn(
                &BoardPosition::new(x, 0),
                PieceType::Rook,
                PieceColor::White,
            );
            board.spawn(
                &BoardPosition::new(x, 7),
                PieceType::Rook,
                PieceColor::Black,
            );
        }
        for x in [1, 6] {
            board.spawn(
                &BoardPosition::new(x, 0),
                PieceType::Knight,
                PieceColor::White,
            );
            board.spawn(
                &BoardPosition::new(x, 7),
                PieceType::Knight,
                PieceColor::Black,
            );
        }
        for x in [2, 5] {
            board.spawn(
                &BoardPosition::new(x, 0),
                PieceType::Bishop,
                PieceColor::White,
            );
            board.spawn(
                &BoardPosition::new(x, 7),
                PieceType::Bishop,
                PieceColor::Black,
            );
        }
        board.spawn(
            &BoardPosition::new(4, 0),
            PieceType::King,
            PieceColor::White,
        );
        board.spawn(
            &BoardPosition::new(4, 7),
            PieceType::King,
            PieceColor::Black,
        );
        board.spawn(
            &BoardPosition::new(3, 0),
            PieceType::Queen,
            PieceColor::White,
        );
        board.spawn(
            &BoardPosition::new(3, 7),
            PieceType::Queen,
            PieceColor::Black,
        );
        board
    }

    pub fn with_pieces(pieces: Vec<BoardPiece>) -> Self {
        let mut pieces_map = HashMap::with_capacity(32);
        for board_piece in pieces {
            pieces_map.insert(
                board_piece.pos().clone(),
                PieceFactory::build(
                    board_piece.piece().piece_type().clone(),
                    board_piece.piece().color().clone(),
                ),
            );
        }
        Self {
            pieces: pieces_map,
            moves: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }
    pub fn spawn(&mut self, position: &BoardPosition, piece_type: PieceType, color: PieceColor) {
        self.pieces
            .insert(position.clone(), PieceFactory::build(piece_type, color));
    }
    pub fn despawn(&mut self, position: &BoardPosition) {
        self.pieces.remove(position);
    }
    pub fn piece_at(&self, position: &BoardPosition) -> Option<&Box<dyn Piece>> {
        self.pieces.get(position)
    }

    fn force_move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) {
        let piece = self.pieces.remove(from);
        if let Some(from_piece) = piece {
            self.pieces.insert(to.clone(), from_piece);
        }
    }

    pub fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) -> BoardSideEffects {
        let mut board_side_effects = BoardSideEffects { takes: vec![] };
        if !self.is_valid_move(from, to) {
            return board_side_effects;
        }
        if let Some(p) = self.piece_at(from) {
            board_side_effects.takes = p.takes(self, from, to);
        }
        if let Some(p) = self.pieces.remove(from) {
            for takes in board_side_effects.takes.iter() {
                self.pieces.remove(takes);
            }
            self.moves.push(BoardMove::new(
                p.piece_type().clone(),
                from.clone(),
                to.clone(),
            ));
            self.pieces.insert(to.clone(), p);
        }
        board_side_effects
    }

    pub fn get_possible_moves(&self, from: &BoardPosition) -> Vec<BoardPosition> {
        let piece = self.pieces.get(from);
        return match piece {
            None => vec![],
            Some(piece) => {
                let mut moves: Vec<BoardPosition> = piece
                    .get_all_moves(&self, from)
                    .into_iter()
                    .filter(|pos| {
                        let mut prediction_board = self.clone();
                        prediction_board.force_move_piece(from, &pos);
                        !prediction_board.is_checked(piece.color())
                    })
                    .collect();
                if piece.piece_type() == &PieceType::King && !self.is_checked(piece.color()) {
                    if let Some(piece) = self.piece_at(&BoardPosition::new(0, 0)) {
                        if piece.piece_type() == &PieceType::Rook {
                            moves.push(BoardPosition::new(2, 0));
                        }
                    }
                    if let Some(piece) = self.piece_at(&BoardPosition::new(self.width() - 1, 0)) {
                        if piece.piece_type() == &PieceType::Rook {
                            moves.push(BoardPosition::new(6, 0));
                        }
                    }
                }
                moves
            }
        };
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
    pub fn is_pos_valid(&self, position: &(i8, i8)) -> bool {
        position.0 >= 0
            && position.0 < self.width() as i8
            && position.1 >= 0
            && position.1 < self.length() as i8
    }
    pub fn pos_is_occupied_with_color(&self, pos: &BoardPosition, color: &PieceColor) -> bool {
        return match self.piece_at(pos) {
            None => true,
            Some(piece) => piece.is_opponent(color),
        };
    }

    pub fn is_checked(&self, color: &PieceColor) -> bool {
        let mut king_pos: Option<&BoardPosition> = None;
        let opponent_moves = self
            .pieces
            .iter()
            .filter(|(pos, piece)| {
                if Self::is_king(piece, color) {
                    king_pos = Some(pos);
                }
                piece.is_opponent(color)
            })
            .map(|(pos, piece)| piece.get_all_moves(&self, pos))
            .flatten()
            .collect::<Vec<BoardPosition>>();
        return match king_pos {
            None => false,
            Some(pos) => opponent_moves.contains(pos),
        };
    }
    pub fn is_mated(&self, color: &PieceColor) -> bool {
        if !self.is_checked(color) {
            return false;
        }
        let possible_moves = self.get_moves_for_color(color);
        possible_moves.is_empty()
    }

    pub fn is_draw(&self) -> bool {
        let colors = [PieceColor::Black, PieceColor::White];
        colors
            .iter()
            .any(|color| self.get_moves_for_color(color).is_empty())
    }

    pub fn active_turn(&self) -> &PieceColor {
        let turns = [&PieceColor::White, &PieceColor::Black];
        turns[self.moves.len() % turns.len()]
    }

    pub fn is_valid_move(&self, from: &BoardPosition, to: &BoardPosition) -> bool {
        if let Some(piece) = self.pieces.get(from) {
            if piece.color() != self.active_turn() {
                return false;
            }
        }
        let moves = self.get_possible_moves(from);
        return moves.contains(to);
    }

    fn get_moves_for_color(&self, color: &PieceColor) -> Vec<BoardPosition> {
        let possible_moves = self
            .pieces
            .iter()
            .filter(|(_, piece)| !piece.is_opponent(color))
            .map(|(pos, _)| self.get_possible_moves(pos))
            .flatten()
            .collect::<Vec<BoardPosition>>();
        possible_moves
    }

    fn is_king(piece: &Box<dyn Piece>, color: &PieceColor) -> bool {
        piece.piece_type() == &PieceType::King && piece.color() == color
    }
}

#[cfg(test)]
mod chess_board_tests {
    use crate::board::CheckerBoard;
    use crate::board_piece::BoardPiece;
    use crate::board_pos;
    use crate::board_position::BoardPosition;
    use crate::pieces::color::PieceColor;
    use crate::pieces::piece_type::PieceType;
    use std::str::FromStr;

    #[test]
    fn new_board_is_empty() {
        let board = CheckerBoard::new();
        assert_eq!(board.is_empty(), true);
    }

    #[test]
    fn it_can_spawn_pieces() {
        let mut board = CheckerBoard::new();
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, PieceType::Pawn, PieceColor::Black);
        let piece = board.piece_at(&position).unwrap();
        assert_eq!(piece.color(), &PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::Pawn);
        assert_eq!(board.is_empty(), false);
    }

    #[test]
    fn it_can_despawn_pieces() {
        let mut board = CheckerBoard::new();
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, PieceType::Pawn, PieceColor::Black);
        board.despawn(&position);
        let piece = board.piece_at(&position);
        assert!(piece.is_none());
        assert_eq!(board.is_empty(), true);
    }

    #[test]
    fn it_can_force_move_pieces() {
        let mut board = CheckerBoard::new();
        let position = BoardPosition::new(0, 0);
        board.spawn(&position, PieceType::Pawn, PieceColor::White);
        let new_position = BoardPosition::new(1, 1);
        board.force_move_piece(&position, &new_position);
        let piece_at_old_pos = board.piece_at(&position);
        assert!(piece_at_old_pos.is_none());
        let piece_at_new_pos = board.piece_at(&new_position).unwrap();
        assert_eq!(piece_at_new_pos.piece_type(), &PieceType::Pawn);
        assert_eq!(piece_at_new_pos.color(), &PieceColor::White);
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

    #[test]
    fn out_of_left_edge_is_not_valid() {
        let board = CheckerBoard::new();
        assert!(!board.is_pos_valid(&(-1, 0)));
    }

    #[test]
    fn out_of_right_edge_is_not_valid() {
        let board = CheckerBoard::new();
        assert!(!board.is_pos_valid(&(8, 0)));
    }

    #[test]
    fn out_of_bottom_edge_is_not_valid() {
        let board = CheckerBoard::new();
        assert!(!board.is_pos_valid(&(0, -1)));
    }

    #[test]
    fn out_of_top_edge_is_not_valid() {
        let board = CheckerBoard::new();
        assert!(!board.is_pos_valid(&(0, 8)));
    }

    #[test]
    fn pos_is_not_occupied_with_color_when_space_is_empty() {
        let board = CheckerBoard::new();
        assert!(board.pos_is_occupied_with_color(&board_pos!("a4"), &PieceColor::White));
    }
    #[test]
    fn pos_is_not_occupied_with_color_when_opponent_occupies_space() {
        let pieces = vec![BoardPiece::build(PieceType::Pawn, PieceColor::Black, "a4")];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(board.pos_is_occupied_with_color(&board_pos!("a4"), &PieceColor::White));
    }
    #[test]
    fn pos_is_occupied_with_color() {
        let pieces = vec![BoardPiece::build(PieceType::Pawn, PieceColor::White, "a4")];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(!board.pos_is_occupied_with_color(&board_pos!("a4"), &PieceColor::White));
    }
    #[test]
    fn empty_board_no_one_is_checked() {
        let board = CheckerBoard::new();
        assert!(!board.is_checked(&PieceColor::White));
        assert!(!board.is_checked(&PieceColor::Black));
    }
    #[test]
    fn king_not_being_attacked_board_is_not_checked() {
        let pieces = vec![
            BoardPiece::build(PieceType::Pawn, PieceColor::White, "e3"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "e4"),
        ];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(!board.is_checked(&PieceColor::Black));
    }
    #[test]
    fn king_attacked_board_is_checked() {
        let pieces = vec![
            BoardPiece::build(PieceType::Pawn, PieceColor::White, "e4"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "d5"),
        ];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(board.is_checked(&PieceColor::Black));
    }

    #[test]
    fn king_attacked_with_a_way_out_is_not_checkmated() {
        let pieces = vec![
            BoardPiece::build(PieceType::Pawn, PieceColor::White, "e4"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "d5"),
        ];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(!board.is_mated(&PieceColor::Black));
    }

    #[test]
    fn king_attacked_with_no_way_out_is_checkmated() {
        let pieces = vec![
            BoardPiece::build(PieceType::Pawn, PieceColor::White, "a7"),
            BoardPiece::build(PieceType::Pawn, PieceColor::White, "b6"),
            BoardPiece::build(PieceType::King, PieceColor::White, "a6"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "a8"),
        ];
        let mut board = CheckerBoard::with_pieces(pieces);
        board.move_piece(&board_pos!("b6"), &board_pos!("b7"));
        assert!(board.is_mated(&PieceColor::Black));
    }

    #[test]
    fn king_un_attacked_with_no_way_out_is_not_checkmate() {
        let pieces = vec![
            BoardPiece::build(PieceType::King, PieceColor::White, "h1"),
            BoardPiece::build(PieceType::Pawn, PieceColor::Black, "h2"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "g3"),
        ];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(!board.is_mated(&PieceColor::White));
    }
    #[test]
    fn king_un_attacked_with_no_way_out_is_draw() {
        let pieces = vec![
            BoardPiece::build(PieceType::King, PieceColor::White, "h1"),
            BoardPiece::build(PieceType::Pawn, PieceColor::Black, "h2"),
            BoardPiece::build(PieceType::King, PieceColor::Black, "g3"),
        ];
        let board = CheckerBoard::with_pieces(pieces);
        assert!(board.is_draw());
    }

    #[test]
    fn default_board_has_all_white_pawns() {
        let board = CheckerBoard::default();
        for x in 0..board.width() {
            let piece = board.piece_at(&BoardPosition::new(x, 1)).unwrap();
            assert_eq!(piece.color(), &PieceColor::White);
            assert_eq!(piece.piece_type(), &PieceType::Pawn);
        }
    }

    #[test]
    fn default_board_has_all_black_pawns() {
        let board = CheckerBoard::default();
        for x in 0..board.width() {
            let piece = board.piece_at(&BoardPosition::new(x, 6)).unwrap();
            assert_eq!(piece.color(), &PieceColor::Black);
            assert_eq!(piece.piece_type(), &PieceType::Pawn);
        }
    }
    #[test]
    fn default_board_has_all_white_rooks() {
        let board = CheckerBoard::default();
        let rook_positions = [board_pos!("a1"), board_pos!("h1")];
        assert_all_pos_have_pieces(
            board,
            rook_positions.into_iter(),
            &PieceType::Rook,
            &PieceColor::White,
        );
    }

    #[test]
    fn default_board_has_all_black_rooks() {
        let board = CheckerBoard::default();
        let rook_positions = [board_pos!("a8"), board_pos!("h8")];
        assert_all_pos_have_pieces(
            board,
            rook_positions.into_iter(),
            &PieceType::Rook,
            &PieceColor::Black,
        );
    }

    #[test]
    fn default_board_has_all_white_knights() {
        let board = CheckerBoard::default();
        let knight_positions = [board_pos!("b1"), board_pos!("g1")];
        assert_all_pos_have_pieces(
            board,
            knight_positions.into_iter(),
            &PieceType::Knight,
            &PieceColor::White,
        );
    }

    #[test]
    fn default_board_has_all_black_knights() {
        let board = CheckerBoard::default();
        let knight_positions = [board_pos!("b8"), board_pos!("g8")];
        assert_all_pos_have_pieces(
            board,
            knight_positions.into_iter(),
            &PieceType::Knight,
            &PieceColor::Black,
        );
    }

    #[test]
    fn default_board_has_all_white_bishops() {
        let board = CheckerBoard::default();
        let bishop_positions = [board_pos!("c1"), board_pos!("f1")];
        assert_all_pos_have_pieces(
            board,
            bishop_positions.into_iter(),
            &PieceType::Bishop,
            &PieceColor::White,
        );
    }

    #[test]
    fn default_board_has_all_black_bishops() {
        let board = CheckerBoard::default();
        let bishop_positions = [board_pos!("c8"), board_pos!("f8")];
        assert_all_pos_have_pieces(
            board,
            bishop_positions.into_iter(),
            &PieceType::Bishop,
            &PieceColor::Black,
        );
    }

    #[test]
    fn default_board_has_white_king() {
        let board = CheckerBoard::default();
        let piece = board.piece_at(&board_pos!("e1")).unwrap();
        assert_eq!(piece.color(), &PieceColor::White);
        assert_eq!(piece.piece_type(), &PieceType::King);
    }
    #[test]
    fn default_board_has_black_king() {
        let board = CheckerBoard::default();
        let piece = board.piece_at(&board_pos!("e8")).unwrap();
        assert_eq!(piece.color(), &PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::King);
    }

    #[test]
    fn default_board_has_white_queen() {
        let board = CheckerBoard::default();
        let piece = board.piece_at(&board_pos!("d1")).unwrap();
        assert_eq!(piece.color(), &PieceColor::White);
        assert_eq!(piece.piece_type(), &PieceType::Queen);
    }
    #[test]
    fn default_board_has_black_queen() {
        let board = CheckerBoard::default();
        let piece = board.piece_at(&board_pos!("d8")).unwrap();
        assert_eq!(piece.color(), &PieceColor::Black);
        assert_eq!(piece.piece_type(), &PieceType::Queen);
    }

    #[test]
    fn first_turn_is_whites() {
        let board = CheckerBoard::default();
        assert_eq!(board.active_turn(), &PieceColor::White)
    }

    #[test]
    fn black_goes_second() {
        let mut board = CheckerBoard::default();
        board.move_piece(&board_pos!("e2"), &board_pos!("e4"));
        assert_eq!(board.active_turn(), &PieceColor::Black)
    }

    #[test]
    fn white_goes_third() {
        let mut board = CheckerBoard::default();
        board.move_piece(&board_pos!("e2"), &board_pos!("e4"));
        board.move_piece(&board_pos!("e7"), &board_pos!("e5"));
        assert_eq!(board.active_turn(), &PieceColor::White)
    }

    #[test]
    fn first_cant_move_black_pieces() {
        let mut board = CheckerBoard::default();
        board.move_piece(&board_pos!("e7"), &board_pos!("e5"));
        assert!(board.piece_at(&board_pos!("e5")).is_none())
    }

    #[test]
    fn incorrect_move_is_invalid() {
        let board = CheckerBoard::default();
        assert!(!board.is_valid_move(&board_pos!("e2"), &board_pos!("e7")));
    }

    #[test]
    fn out_of_order_move_is_invalid() {
        let board = CheckerBoard::default();
        assert!(!board.is_valid_move(&board_pos!("e7"), &board_pos!("e6")));
    }

    #[test]
    fn can_not_move_if_move_is_invalid() {
        let mut board = CheckerBoard::default();
        board.move_piece(&board_pos!("e7"), &board_pos!("e6"));
        assert!(board.piece_at(&board_pos!("e6")).is_none());
        board.move_piece(&board_pos!("e2"), &board_pos!("c4"));
        assert!(board.piece_at(&board_pos!("c4")).is_none());
    }

    #[test]
    fn move_piece_returns_empty_list_of_takes_when_no_takes_happen() {
        let mut board = CheckerBoard::default();
        let side_effects = board.move_piece(&board_pos!("e7"), &board_pos!("e6"));
        assert!(side_effects.takes.is_empty());
    }
    #[test]
    fn side_effect_takes_removes_piece_from_board() {
        let d4 = BoardPiece::build(PieceType::Pawn, PieceColor::Black, "d4");
        let c2 = BoardPiece::build(PieceType::Pawn, PieceColor::White, "c2");
        let pieces = vec![d4, c2];
        let mut board = CheckerBoard::with_pieces(pieces);
        board.move_piece(&board_pos!["c2"], &board_pos!["c4"]);
        board.move_piece(&board_pos!("d4"), &board_pos!("c3"));
        assert!(board.piece_at(&board_pos!("c4")).is_none());
    }

    fn assert_all_pos_have_pieces(
        board: CheckerBoard,
        rook_positions: impl Iterator<Item = BoardPosition>,
        piece_type: &PieceType,
        color: &PieceColor,
    ) {
        for pos in rook_positions {
            let piece = board.piece_at(&pos).unwrap();
            assert_eq!(piece.color(), color);
            assert_eq!(piece.piece_type(), piece_type);
        }
    }
}
