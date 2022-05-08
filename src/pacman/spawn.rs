use bevy::prelude::*;
use crate::common::MoveDirection;

use crate::constants::PACMAN_DIMENSION;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::FieldType::PacManSpawn;
use crate::pacman::Pacman;
use crate::speed::SpeedByLevel;

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let start_position = board.position_of_type(PacManSpawn).clone();
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("FFEE00").unwrap(),
                custom_size: Some(pacman_dimension),
                ..default()
            },
            transform: Transform::from_translation(board.coordinates_of_position(&start_position)),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(MoveDirection::Up)
        .insert(speed_by_level.get_pacman_speed_by_level(&level))
        .insert(start_position);
}