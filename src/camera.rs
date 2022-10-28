use bevy::prelude::*;
use crate::constants::FIELD_DIMENSION;
use crate::life_cycle::LifeCycle::Start;
use crate::map::board::Board;

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
    board: Res<Board>
) {
    let mut bundle = Camera2dBundle::default();
    modify_camera_x_y(&mut bundle.transform.translation, &board);
    commands.spawn().insert_bundle(bundle);
}

fn modify_camera_x_y(translation: &mut Vec3, board: &Board) {
    translation.x = (board.width as f32 / 2.0) * FIELD_DIMENSION;
    translation.y = (board.height as f32 / 2.0) * FIELD_DIMENSION
}