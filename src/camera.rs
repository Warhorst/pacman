use bevy::prelude::*;
use crate::constants::FIELD_SIZE;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::map::Map;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Game(Start)), spawn_camera)
            .add_systems(OnExit(Game(GameOver)), despawn_camera)
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
    map_query: Query<&Map>,
) {
    let map = map_query.single();

    commands.spawn((
        Name::new("GameCamera"),
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new((map.width as f32 * FIELD_SIZE) / 2.0, (map.height as f32 * FIELD_SIZE) / 2.0, 1000.0)),
            ..default()
        },
        UiCameraConfig { show_ui: true }
    ));
}

fn despawn_camera(
    mut commands: Commands,
    query: Query<Entity, With<Camera>>
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}