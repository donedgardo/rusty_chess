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

fn main() {
    let board = CheckerBoard::default();
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
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        use bevy_mod_picking::debug::DebugPickingMode;
        app.add_plugins(WorldInspectorPlugin::new());
        app.insert_resource(DebugPickingMode::Normal);
    }
    app.insert_resource(BoardUiFactory::new(68.5, 72., board))
        .add_systems(Startup, setup)
        .add_systems(Update, add_board_pos_markers_sprite);

    app.run();
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_ui_factory: ResMut<BoardUiFactory>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 500.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("board.png"),
        ..Default::default()
    });
    for pos in board_ui_factory.get_pos_iter() {
        board_ui_factory.create_empty_board_position(&mut commands, &asset_server, &pos);
        board_ui_factory.create_board_piece(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            &pos,
        );
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    InGame,
}

#[cfg(test)]
mod game_state_tests {
    use crate::GameState;
    use bevy::prelude::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn default_is_in_game() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<GameState>();
        let state = app.world().resource::<State<GameState>>();
        assert_eq!(state, &GameState::InGame);
    }
}
