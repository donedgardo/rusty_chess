use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::board_position_marker::BoardPositionMarker;
use crate::pieces::color::PieceColor;
use crate::pieces::piece_type::PieceType;
use crate::{BoardPieceComponent, WithBoardPosition};
use bevy::prelude::{BuildChildren, Commands, Component, Entity, Query, Resource, Transform, With};
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct BoardUiFactory {
    pos_width: f32,
    pos_height: f32,
    pub board: CheckerBoard,
    pos_entities: HashMap<BoardPosition, Entity>,
    piece_entities: HashMap<BoardPosition, Entity>,
}

impl BoardUiFactory {
    pub fn new(pos_width: f32, pos_height: f32, board: CheckerBoard) -> Self {
        Self {
            pos_width,
            pos_height,
            board,
            pos_entities: HashMap::with_capacity(64),
            piece_entities: HashMap::with_capacity(32),
        }
    }
    pub fn get_pos_transform(&self, pos: &BoardPosition) -> Transform {
        Transform::from_xyz(
            (self.pos_width * (pos.x() as f32 - 4.)) + (self.pos_width / 2.),
            (self.pos_height * (pos.y() as f32 - 4.)) + (self.pos_height / 2.),
            1.,
        )
    }

    pub fn get_pos_iter(&self) -> impl Iterator<Item = BoardPosition> {
        let mut board_positions =
            Vec::with_capacity((self.board.width() * self.board.length()) as usize);
        for x in 0..self.board.width() {
            for y in 0..self.board.length() {
                board_positions.push(BoardPosition::new(x, y))
            }
        }
        board_positions.into_iter()
    }

    pub fn add_board_pos_entity(&mut self, pos: &BoardPosition, entity: Entity) {
        self.pos_entities.insert(pos.clone(), entity);
    }

    pub fn get_pos_entity(&self, pos: &BoardPosition) -> Option<&Entity> {
        self.pos_entities.get(pos)
    }

    pub fn add_markers_to_possible_board_moves(
        &self,
        pos: &BoardPosition,
        commands: &mut Commands,
    ) {
        let possible_moves = self.board.get_possible_moves(pos);
        for board_position in possible_moves {
            match self.pos_entities.get(&board_position) {
                None => return,
                Some(entity) => {
                    commands.entity(*entity).with_children(|parent| {
                        parent.spawn(BoardPositionMarker);
                    });
                }
            }
        }
    }

    pub fn get_sprite_index(&self, pos: &BoardPosition) -> Option<usize> {
        return match self.board.piece_at(pos) {
            None => None,
            Some(piece) => {
                return match piece.color() {
                    PieceColor::White => match piece.piece_type() {
                        PieceType::Pawn => Some(6),
                        PieceType::Knight => Some(9),
                        PieceType::King => Some(10),
                        PieceType::Rook => Some(7),
                        PieceType::Bishop => Some(8),
                        PieceType::Queen => Some(11),
                    },
                    PieceColor::Black => match piece.piece_type() {
                        PieceType::Pawn => Some(0),
                        PieceType::Knight => Some(3),
                        PieceType::King => Some(4),
                        PieceType::Rook => Some(1),
                        PieceType::Bishop => Some(2),
                        PieceType::Queen => Some(5),
                    },
                }
            }
        };
    }
    pub fn get_piece_entity_at(&self, pos: &BoardPosition) -> Option<&Entity> {
        self.piece_entities.get(pos)
    }

    pub fn add_piece_entity(&mut self, pos: &BoardPosition, entity: Entity) {
        self.piece_entities.insert(pos.clone(), entity);
    }

    // not-tested
    pub fn move_pieces(
        &mut self,
        piece_entity: Entity,
        mut commands: &mut Commands,
        pieces_query: Query<(Entity, &BoardPieceComponent)>,
        from: Option<BoardPosition>,
        to: Option<BoardPosition>,
    ) {
        if let (Some(from), Some(to)) = (from, to) {
            if !self.board.is_valid_move(&from, &to) {
                self.move_entity(piece_entity, &mut commands, &from);
            } else {
                let side_effects = self.board.move_piece(&from, &to);
                self.remove_all_taken_pieces(&mut commands, pieces_query, side_effects.takes);
                self.move_piece_to(piece_entity, &mut commands, &to);
            }
        }
    }

    // not-tested
    pub fn move_entity(&self, entity: Entity, commands: &mut Commands, to: &BoardPosition) {
        let transform = self.get_pos_transform(&to);
        commands.entity(entity).insert(transform);
    }
    //not tested
    pub fn move_piece_to(&self, entity: Entity, mut commands: &mut Commands, to: &BoardPosition) {
        self.move_entity(entity, &mut commands, &to);
        commands
            .entity(entity)
            .insert(BoardPieceComponent(to.clone()));
    }
    //not tested
    pub fn remove_all_taken_pieces(
        &mut self,
        commands: &mut Commands,
        pieces_query: Query<(Entity, &BoardPieceComponent)>,
        takes: Vec<BoardPosition>,
    ) {
        for (entity, board_piece) in pieces_query.iter() {
            for takes in takes.iter() {
                if takes == &board_piece.0 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    // not tested
    pub fn remove_all_markers(
        commands: &mut Commands,
        marker_query: &Query<Entity, With<BoardPositionMarker>>,
    ) {
        for marker in marker_query.iter() {
            commands.entity(marker).despawn();
        }
    }

    // not tested
    pub fn get_pos<T: Component + WithBoardPosition>(
        entity: Entity,
        query: &Query<(Entity, &T)>,
    ) -> Option<BoardPosition> {
        query
            .get(entity)
            .ok()
            .and_then(|(_, component)| Some(component.pos().clone()))
    }
}

#[cfg(test)]
mod board_positions_test {
    use crate::board::CheckerBoard;
    use crate::board_pos;
    use crate::board_position_marker::BoardPositionMarker;
    use crate::board_ui_factory::BoardUiFactory;
    use bevy::prelude::{App, Transform};
    use std::str::FromStr;

    #[test]
    fn it_calculates_a8_board_position_correctly() {
        let board = CheckerBoard::new();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        let a8 = board_pos!("a8");
        let board_vector_pos = board_ui_factory.get_pos_transform(&a8);
        assert_eq!(board_vector_pos, Transform::from_xyz(-239.75, 252.0, 1.))
    }

    #[test]
    fn it_calculates_h1_board_position_correctly() {
        let board = CheckerBoard::new();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        let h1 = board_pos!("h1");
        let board_vector_pos = board_ui_factory.get_pos_transform(&h1);
        assert_eq!(board_vector_pos, Transform::from_xyz(239.75, -252., 1.))
    }

    #[test]
    fn it_creates_all_board_positions() {
        let board = CheckerBoard::new();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_pos_iter().collect::<Vec<_>>().len(),
            64
        );
    }

    #[test]
    fn it_can_add_entity_for_board_positions() {
        let board = CheckerBoard::new();
        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let pos = board_pos!("a1");
        let mut board_ui_factory = create_board_ui_factory(68.5, 72., board);
        board_ui_factory.add_board_pos_entity(&pos, entity);
        assert_eq!(board_ui_factory.get_pos_entity(&pos), Some(&entity));
    }

    #[test]
    fn it_adds_board_position_marker_component_to_possible_move_positions() {
        let board = CheckerBoard::default();
        let mut app = App::new();
        let mut board_ui_factory = create_board_ui_factory(68.5, 72., board);

        for pos in ["a2", "a3", "a4"] {
            let entity = app.world_mut().spawn(Transform::default()).id();
            let pos = board_pos!(pos);
            board_ui_factory.add_board_pos_entity(&pos, entity);
        }

        let mut commands = app.world_mut().commands();
        board_ui_factory.add_markers_to_possible_board_moves(&board_pos!("a2"), &mut commands);
        app.update();

        let board_marker_count = app
            .world_mut()
            .query::<&BoardPositionMarker>()
            .iter(app.world())
            .len();
        assert_eq!(board_marker_count, 2);
    }

    #[test]
    fn sprite_sheet_index_returns_none_when_pos_is_empty() {
        let pos = board_pos!("a1");
        let pieces = vec![];
        let board = CheckerBoard::with_pieces(pieces);
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(board_ui_factory.get_sprite_index(&pos), None);
    }
    #[test]
    fn sprite_sheet_returns_index_for_white_pawn() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("a2")),
            Some(6)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_white_knight() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("b1")),
            Some(9)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_white_rook() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("a1")),
            Some(7)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_white_king() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("e1")),
            Some(10)
        );
    }

    #[test]
    fn sprite_sheet_returns_index_for_black_pawn() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("a7")),
            Some(0)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_black_knight() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("b8")),
            Some(3)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_black_rook() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("a8")),
            Some(1)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_black_king() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("e8")),
            Some(4)
        );
    }

    #[test]
    fn sprite_sheet_returns_index_for_white_bishop() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("c1")),
            Some(8)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_black_bishop() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("c8")),
            Some(2)
        );
    }

    #[test]
    fn sprite_sheet_returns_index_for_white_queen() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("d1")),
            Some(11)
        );
    }
    #[test]
    fn sprite_sheet_returns_index_for_black_queen() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        assert_eq!(
            board_ui_factory.get_sprite_index(&board_pos!("d8")),
            Some(5)
        );
    }

    #[test]
    fn can_get_piece_entity() {
        let board = CheckerBoard::default();
        let board_ui_factory = create_board_ui_factory(68.5, 72., board);
        let pos = board_pos!("a2");
        assert_eq!(board_ui_factory.get_piece_entity_at(&pos), None);
    }

    #[test]
    fn can_add_entity_for_piece() {
        let board = CheckerBoard::default();
        let mut board_ui_factory = create_board_ui_factory(68.5, 72., board);
        let mut app = App::new();
        let entity = app.world_mut().spawn(Transform::default()).id();
        let pos = board_pos!("a2");
        board_ui_factory.add_piece_entity(&pos, entity);
        assert_eq!(board_ui_factory.get_piece_entity_at(&pos), Some(&entity));
    }

    fn create_board_ui_factory(width: f32, height: f32, board: CheckerBoard) -> BoardUiFactory {
        BoardUiFactory::new(width, height, board)
    }
}
