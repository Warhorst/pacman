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
    let mut bundle = OrthographicCameraBundle::new_2d();
    bundle.transform.translation = calculate_camera_position(board.width, board.height);
    commands.spawn().insert_bundle(bundle);
}

fn calculate_camera_position(width: usize, height: usize) -> Vec3 {
    let x = width as f32 * FIELD_DIMENSION / 2.0;
    let y = height as f32 * FIELD_DIMENSION / 2.0;
    Vec3::new(x, y, 0.0)
}