use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection::*;
use crate::constants::GHOST_DIMENSION;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, Inky, Pinky};
use crate::ghosts::state::State;
use crate::level::Level;
use crate::map::board::Board;
use crate::speed::SpeedByLevel;

pub fn spawn_ghosts(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let ghost_house = GhostHouse::new(&board);
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Blinky>(), &level, &speed_by_level, Color::hex("FF0000").unwrap(), Blinky);
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Pinky>(), &level, &speed_by_level, Color::hex("FFB8FF").unwrap(), Pinky);
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Inky>(), &level, &speed_by_level, Color::hex("00FFFF").unwrap(), Inky);
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Clyde>(), &level, &speed_by_level, Color::hex("FFB852").unwrap(), Clyde);
    commands.insert_resource(ghost_house)
}

fn spawn_ghost(
    commands: &mut Commands,
    spawn_coordinates: Vec3,
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
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        })
        .insert(Ghost)
        .insert(ghost_type)
        .insert(Position::from(&spawn_coordinates))
        .insert(Left)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(State::Spawned);
}