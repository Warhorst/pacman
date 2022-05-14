use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection::Up;
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::state::Spawned;
use crate::ghosts::state::State::Scatter;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::FieldType;
use crate::speed::SpeedByLevel;

pub fn spawn_ghosts(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let spawn_positions = board.positions_of_type(FieldType::GhostSpawn);
    spawn_ghost(&mut commands, &board, spawn_positions[0], &level, &speed_by_level, Blinky);
    spawn_ghost(&mut commands, &board, spawn_positions[1], &level, &speed_by_level, Pinky);
    spawn_ghost(&mut commands, &board, spawn_positions[2], &level, &speed_by_level, Inky);
    spawn_ghost(&mut commands, &board, spawn_positions[3], &level, &speed_by_level, Clyde)
}

fn spawn_ghost(
    commands: &mut Commands,
    board: &Board,
    position: &Position,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    ghost: Ghost
) {
    let color = match ghost {
        Blinky => Color::hex("FF0000").unwrap(),
        Pinky => Color::hex("FFB8FF").unwrap(),
        Inky => Color::hex("00FFFF").unwrap(),
        Clyde => Color::hex("FFB852").unwrap(),
    };
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            ..Default::default()
        })
        .insert(ghost)
        .insert(*position)
        .insert(Up)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(Scatter) // TODO this is a placeholder until the state enum is fully removed
        .insert(Spawned);
}