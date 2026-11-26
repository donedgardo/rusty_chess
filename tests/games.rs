//! Integration tests: whole games played through the public board API,
//! the same calls the Bevy game makes when a piece is dropped on a square.

use rusty_chess::board::CheckerBoard;
use rusty_chess::board_position::BoardPosition;
use rusty_chess::pieces::color::PieceColor;

fn pos(square: &str) -> BoardPosition {
    square.parse().expect("valid square")
}

/// Plays moves written as "e2 e4" from the starting position.
fn play(moves: &[&str]) -> CheckerBoard {
    let mut board = CheckerBoard::default();
    for m in moves {
        let (from, to) = m.split_once(' ').expect("move is 'from to'");
        assert!(
            board.is_valid_move(&pos(from), &pos(to)),
            "{m} should be a legal move"
        );
        board.move_piece(&pos(from), &pos(to));
    }
    board
}

#[test]
fn a_new_game_starts_with_white_to_move_and_nobody_in_check() {
    let board = CheckerBoard::default();

    assert_eq!(board.active_turn(), &PieceColor::White);
    assert!(!board.is_checked(&PieceColor::White));
    assert!(!board.is_checked(&PieceColor::Black));
    assert!(!board.is_game_over());
}

#[test]
fn turns_alternate_after_each_move() {
    let board = play(&["e2 e4"]);
    assert_eq!(board.active_turn(), &PieceColor::Black);

    let board = play(&["e2 e4", "e7 e5"]);
    assert_eq!(board.active_turn(), &PieceColor::White);
}

#[test]
fn a_player_cannot_move_out_of_turn() {
    let mut board = CheckerBoard::default();

    assert!(!board.is_valid_move(&pos("e7"), &pos("e5")));
    board.move_piece(&pos("e7"), &pos("e5"));

    assert!(board.piece_at(&pos("e7")).is_some());
    assert!(board.piece_at(&pos("e5")).is_none());
    assert_eq!(board.active_turn(), &PieceColor::White);
}

#[test]
fn fools_mate_checkmates_white_in_two_moves() {
    let board = play(&["f2 f3", "e7 e5", "g2 g4", "d8 h4"]);

    assert!(board.is_mated(&PieceColor::White));
    assert!(!board.is_mated(&PieceColor::Black));
    assert!(board.is_game_over());
}

#[test]
fn scholars_mate_checkmates_black_in_four_moves() {
    let board = play(&[
        "e2 e4", "e7 e5", "f1 c4", "b8 c6", "d1 h5", "g8 f6", "h5 f7",
    ]);

    assert!(board.is_mated(&PieceColor::Black));
    assert!(board.is_game_over());
}

#[test]
fn a_capture_removes_the_taken_piece() {
    let mut board = play(&["e2 e4", "d7 d5"]);

    let effects = board.move_piece(&pos("e4"), &pos("d5"));

    assert_eq!(effects.takes, vec![pos("d5")]);
    let piece = board.piece_at(&pos("d5")).expect("pawn on d5");
    assert_eq!(piece.color(), &PieceColor::White);
    assert!(board.piece_at(&pos("e4")).is_none());
}

#[test]
fn a_king_in_check_must_get_out_of_check() {
    // 1. e4 f5 2. Qh5+ : the only legal reply is g6.
    let board = play(&["e2 e4", "f7 f5", "d1 h5"]);

    assert!(board.is_checked(&PieceColor::Black));
    assert!(!board.is_mated(&PieceColor::Black));
    assert!(!board.is_valid_move(&pos("a7"), &pos("a6")));
    assert!(board.is_valid_move(&pos("g7"), &pos("g6")));
}
