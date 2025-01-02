mod board;
mod board_move;
mod board_piece;
mod board_position;
mod board_position_marker;
mod board_side_effects;
mod board_ui_factory;
mod pieces;

use crate::board::CheckerBoard;
use crate::board_position::BoardPosition;
use crate::board_position_marker::add_board_pos_markers_sprite;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_mod_picking::{low_latency_window_plugin, DefaultPickingPlugins};
use board_ui_factory::BoardUiFactory;

//TODO:
// * Game Loop (Restart after game over)
// * Season cycles
// * Seasonal Pieces
// * AI easy
// * Sounds
// * Title Screen
// * AI - Hard Monte carlo tree search
// * - Bug Dropping outside board should return piece.
//   --- (Fix by only allowing board to move inside of board)

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
    GameOver,
}
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(low_latency_window_plugin()),
        DefaultPickingPlugins,
    ));
    #[cfg(feature = "debug")]
    {
        use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
        use bevy_mod_picking::debug::DebugPickingMode;
        app.add_plugins((
            WorldInspectorPlugin::new(),
            StateInspectorPlugin::<GameState>::default(),
        ));
        app.register_type::<GameState>();
        app.insert_resource(DebugPickingMode::Normal);
    }
    let board = CheckerBoard::default();
    app.insert_resource(BoardUiFactory::new(68.5, 72., board))
        .init_state::<GameState>()
        .add_systems(Startup, setup_game)
        .add_systems(OnEnter(GameState::InGame), setup)
        .add_systems(Update, add_board_pos_markers_sprite);

    app.run();
}

fn setup_game(
    asset_server: Res<AssetServer>,
    mut board_ui_factory: ResMut<BoardUiFactory>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let pieces_sprite = asset_server.load("pieces.png");
    board_ui_factory.add_pieces_sprite_handle(pieces_sprite);

    //  TODO: Maybe move this options  inside board_ui_factory
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(54), 6, 2, None, None);
    let texture_layout = texture_atlas_layouts.add(layout);
    board_ui_factory.add_pieces_texture_atlas(texture_layout);

    let empty_pos_sprite = asset_server.load("board_position_empty.png");
    board_ui_factory.add_empty_pos_sprite_handle(empty_pos_sprite);

    let board_sprite = asset_server.load("board.png");
    board_ui_factory.add_board_sprite_handle(board_sprite);
    next_state.set(GameState::InGame);
}

#[derive(Component, Clone)]
struct BoardPosComponent(BoardPosition);

impl WithBoardPosition for BoardPosComponent {
    fn pos(&self) -> &BoardPosition {
        &self.0
    }
}

#[derive(Component, Clone)]
struct BoardPieceComponent(BoardPosition);

impl WithBoardPosition for BoardPieceComponent {
    fn pos(&self) -> &BoardPosition {
        &self.0
    }
}

pub trait WithBoardPosition {
    fn pos(&self) -> &BoardPosition;
}

fn setup(mut commands: Commands, mut board_ui_factory: ResMut<BoardUiFactory>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 500.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: board_ui_factory.get_board_sprite(),
        ..Default::default()
    });
    for pos in board_ui_factory.get_pos_iter() {
        board_ui_factory.create_empty_board_position(&mut commands, &pos);
        board_ui_factory.create_board_piece(&mut commands, &pos);
    }
}

#[cfg(test)]
mod game_state_tests {
    use crate::board::{chess_board_tests, CheckerBoard};
    use crate::board_ui_factory::BoardUiFactory;
    use crate::GameState;
    use bevy::prelude::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    #[ignore]
    fn when_game_is_not_over_still_in_game_state() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<GameState>();
        let board = CheckerBoard::default();
        let board_ui_factory = BoardUiFactory::new(10., 10., board);
        app.insert_resource(board_ui_factory);
        let state = app.world().resource::<State<GameState>>();
        assert_eq!(state, &GameState::InGame);
    }

    #[test]
    #[ignore]
    fn when_game_is_over_state_equals_game_over() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<GameState>();
        let board = chess_board_tests::create_game_with_black_mated();
        let board_ui_factory = BoardUiFactory::new(10., 10., board);
        app.insert_resource(board_ui_factory);
        // TODO: need to move to trigger state update :(
        let state = app.world().resource::<State<GameState>>();
        assert_eq!(state, &GameState::GameOver);
    }
}
