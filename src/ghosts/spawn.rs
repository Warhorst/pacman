use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection::Up;
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::{Blinky, Clyde, Ghost, Inky, Pinky};
use crate::ghosts::state::Spawned;
use crate::is;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::Element::*;
use crate::speed::SpeedByLevel;

pub fn spawn_ghosts(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    spawn_ghost(&mut commands, board.get_position_matching(is!(BlinkySpawn)), &level, &speed_by_level, Color::hex("FF0000").unwrap(), Blinky);
    spawn_ghost(&mut commands, board.get_position_matching(is!(PinkySpawn)), &level, &speed_by_level, Color::hex("FFB8FF").unwrap(), Pinky);
    spawn_ghost(&mut commands, board.get_position_matching(is!(InkySpawn)), &level, &speed_by_level, Color::hex("00FFFF").unwrap(), Inky);
    spawn_ghost(&mut commands, board.get_position_matching(is!(ClydeSpawn)), &level, &speed_by_level, Color::hex("FFB852").unwrap(), Clyde)
}

fn spawn_ghost(
    commands: &mut Commands,
    position: &Position,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    color: Color,
    ghost_type: impl Component
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::from(position)),
            ..Default::default()
        })
        .insert(Ghost)
        .insert(ghost_type)
        .insert(*position)
        .insert(Up)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(Spawned);
}