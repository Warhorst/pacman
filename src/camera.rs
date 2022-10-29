use bevy::prelude::*;
use crate::constants::FIELD_DIMENSION;
use crate::life_cycle::LifeCycle::Start;
use crate::map::Map;

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
    map_query: Query<&Map>
) {
    let map = map_query.single();
    let mut bundle = Camera2dBundle::default();
    modify_camera_x_y(&mut bundle.transform.translation, map);
    commands.spawn().insert_bundle(bundle);
}

fn modify_camera_x_y(translation: &mut Vec3, map: &Map) {
    translation.x = (map.width as f32 * FIELD_DIMENSION) / 2.0;
    translation.y = (map.height as f32 * FIELD_DIMENSION) / 2.0
}