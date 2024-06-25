use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BoardPosition(u8, u8);

#[derive(Error, Debug)]
pub enum BoardPositionError {
    #[error("Invalid board position: {0}")]
    Invalid(String),
}

impl BoardPosition {
    pub fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            panic!("Board position out of bounds: {}:{}", x, y);
        }
        Self(x, y)
    }

    pub fn x(&self) -> u8 {
        self.0
    }
    pub fn y(&self) -> u8 {
        self.1
    }
}

impl FromStr for BoardPosition {
    type Err = BoardPositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .chars()
            .nth(0)
            .ok_or(BoardPositionError::Invalid(s.into()))?
            .to_lowercase()
            .next()
            .ok_or(BoardPositionError::Invalid(s.into()))? as u8
            - 'a' as u8;
        let y = s
            .chars()
            .nth(1)
            .ok_or(BoardPositionError::Invalid(s.into()))?
            .to_digit(10)
            .ok_or(BoardPositionError::Invalid(s.into()))? as u8
            - 1;
        Ok(BoardPosition::new(x, y))
    }
}

#[macro_export]
macro_rules! board_pos {
    ($s:expr) => {
        $crate::board_position::BoardPosition::from_str($s)
            .expect(&format!("Invalid board position: {}", $s))
    };
}

#[cfg(test)]
mod board_pos_tests {
    use crate::board_position::BoardPosition;
    use std::str::FromStr;

    #[test]
    #[should_panic]
    fn is_invalid_outside_x_board() {
        let _pos = BoardPosition::new(9, 0);
    }
    #[test]
    #[should_panic]
    fn is_invalid_outside_y_board() {
        let _pos = BoardPosition::new(0, 9);
    }

    #[test]
    fn is_valid_inside_board() {
        for x in 0..8 {
            for y in 0..8 {
                BoardPosition::new(x, y);
            }
        }
    }

    #[test]
    fn gets_x() {
        let pos = BoardPosition::new(0, 0);
        assert_eq!(pos.x(), 0);
    }

    #[test]
    fn gets_y() {
        let pos = BoardPosition::new(0, 2);
        assert_eq!(pos.y(), 2);
    }

    #[test]
    fn create_with_string() {
        let pos: BoardPosition = "a1".parse().unwrap();
        assert_eq!(pos, BoardPosition::new(0, 0));
        let pos: BoardPosition = "h8".parse().unwrap();
        assert_eq!(pos, BoardPosition::new(7, 7));
    }

    #[test]
    #[should_panic]
    fn x_out_of_bounds_create_with_string_should_panic() {
        let _pos = "i1".parse::<BoardPosition>();
    }
    #[test]
    #[should_panic]
    fn y_out_of_bounds_create_with_string_should_panic() {
        let _pos = "a9".parse::<BoardPosition>();
    }

    #[test]
    fn board_pos_macro_works() {
        let pos = board_pos!("a1");
        assert_eq!(pos, BoardPosition::new(0, 0));
        let pos: BoardPosition = board_pos!("h8");
        assert_eq!(pos, BoardPosition::new(7, 7));
    }

    #[test]
    #[should_panic]
    fn x_out_of_bounds_create_with_macro_should_panic() {
        let _pos = board_pos!("i1");
    }

    #[test]
    #[should_panic]
    fn y_out_of_bounds_create_with_macro_should_panic() {
        let _pos = board_pos!("a9");
    }
}
