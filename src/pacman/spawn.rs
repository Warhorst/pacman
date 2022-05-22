use bevy::prelude::*;

use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::Element::PacManSpawn;
use crate::pacman::Pacman;
use crate::speed::SpeedByLevel;

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let start_position = board.get_position_matching(is!(PacManSpawn)).clone();
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("FFEE00").unwrap(),
                custom_size: Some(pacman_dimension),
                ..default()
            },
            transform: Transform::from_translation(Vec3::from(&start_position)),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(speed_by_level.for_pacman(&level).normal)
        .insert(start_position);
}