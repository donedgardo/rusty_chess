use bevy::asset::AssetServer;
use bevy::prelude::{default, Added, Commands, Component, Entity, Query, Res, SpriteBundle};

#[derive(Component)]
pub struct BoardPositionMarker;

pub fn add_board_pos_markers_sprite(
    mut commands: Commands,
    query: Query<Entity, Added<BoardPositionMarker>>,
    asset_server: Res<AssetServer>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpriteBundle {
            texture: asset_server.load("board_position_marker.png"),
            ..default()
        });
    }
}
