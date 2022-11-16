use bevy::prelude::*;
use crate::constants::FIELD_DIMENSION;
use crate::life_cycle::LifeCycle::Start;
use crate::game::map::Map;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_camera)
            )
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
            transform: Transform::from_translation(Vec3::new((map.width as f32 * FIELD_DIMENSION) / 2.0, (map.height as f32 * FIELD_DIMENSION) / 2.0, 1000.0)),
            ..default()
        },
        UiCameraConfig { show_ui: true }
    ));
}