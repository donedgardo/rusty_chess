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
use crate::board_position_marker::{add_board_pos_markers_sprite, BoardPositionMarker};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_mod_picking::prelude::{Drop, Listener, On, Pickable, Pointer};
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
        let pos_transform = board_ui_factory.get_pos_transform(&pos);
        let id = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("board_position_empty.png"),
                    transform: pos_transform.clone(),
                    ..default()
                },
                Pickable::default(),
                BoardPosComponent(pos.clone()),
                // TODO: Duplication almost identical to
                // board ui factory create board piece entity drop
                On::<Pointer<Drop>>::run(
                    |event: Listener<Pointer<Drop>>,
                     mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
                     mut board_ui_factory: ResMut<BoardUiFactory>,
                     board_piece_query: Query<(Entity, &BoardPieceComponent)>,
                     board_pos_query: Query<(Entity, &BoardPosComponent)>,
                     texture_query: Query<&mut TextureAtlas>,
                     marker_query: Query<Entity, With<BoardPositionMarker>>| {
                        let from = BoardUiFactory::get_pos(event.dropped, &board_piece_query);
                        let to = BoardUiFactory::get_pos(event.target, &board_pos_query);
                        board_ui_factory.move_pieces(
                            event.dropped,
                            &mut commands,
                            board_piece_query,
                            texture_query,
                            from,
                            to,
                            &asset_server,
                            &mut texture_atlas_layouts,
                        );
                        BoardUiFactory::remove_all_markers(&mut commands, &marker_query);
                    },
                ),
            ))
            .id();
        board_ui_factory.add_board_pos_entity(&pos, id);
        board_ui_factory.create_board_piece_entity(
            &mut commands,
            &asset_server,
            &mut texture_atlas_layouts,
            &pos,
        );
    }
}
