mod board;
mod board_move;
mod board_piece;
mod board_position;
mod board_ui_factory;
mod game;
mod pieces;

use crate::board::CheckerBoard;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use board_ui_factory::BoardUiFactory;

fn main() {
    let board = CheckerBoard::default();
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .insert_resource(BoardUiFactory::new(68.5, 72., board))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board_ui_factory: Res<BoardUiFactory>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        let shape = board_ui_factory.get_shape();
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape)),
                material: materials.add(Color::srgba(0., 0., 0., 0.)),
                transform: pos_transform,
                ..default()
            })
            .with_children(|parent| {
                if let Some(index) = board_ui_factory.get_sprite_index(&pos) {
                    let texture = asset_server.load("pieces.png");
                    let layout = TextureAtlasLayout::from_grid(UVec2::splat(54), 6, 2, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    parent.spawn((
                        SpriteBundle {
                            texture,
                            ..default()
                        },
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index,
                        },
                    ));
                }
            });
    }
}
