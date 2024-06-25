struct ChessGame;

impl ChessGame {
    pub fn new() -> Self {
        Self {}
    }
    pub fn is_over(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod chess_game_tests {
    use crate::game::ChessGame;

    #[test]
    fn new_game_is_not_over() {
        let game = ChessGame::new();
        assert_eq!(game.is_over(), false);
    }

}
