use bevy::prelude::*;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::life_cycle::LifeCycle::Start;

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
) {
    let mut bundle = Camera2dBundle::default();
    modify_camera_x_y(&mut bundle.transform.translation);
    commands.spawn().insert_bundle(bundle);
}

fn modify_camera_x_y(translation: &mut Vec3) {
    translation.x = WINDOW_WIDTH / 2.0;
    translation.y = WINDOW_HEIGHT / 2.0;
}