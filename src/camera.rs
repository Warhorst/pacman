use bevy::prelude::*;
use crate::constants::FIELD_DIMENSION;
use crate::map::board::Board;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
    board: Res<Board>,
) {
    let mut bundle = Camera2dBundle::default();
    modify_camera_x_y(&board, &mut bundle.transform.translation);
    commands.spawn().insert_bundle(bundle);
}

fn modify_camera_x_y(board: &Board, translation: &mut Vec3) {
    translation.x = board.width as f32 * FIELD_DIMENSION / 2.0;
    translation.y = board.height as f32 * FIELD_DIMENSION / 2.0;
}