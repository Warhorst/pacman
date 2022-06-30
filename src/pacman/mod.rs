use bevy::prelude::*;

use crate::common::{Direction, Position};
use crate::common::Direction::*;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::lives::Life;
use crate::pacman::spawn::{PacmanSpawn, spawn_pacman};
use crate::map::Rotation;
use crate::pacman::movement::PacmanMovementPlugin;

mod movement;
mod spawn;

/// Marker component for a pacman entity.
#[derive(Component)]
pub struct Pacman;

/// Fired when pacman was killed by a ghost.
pub struct PacmanKilled;

/// Fired when Pacman ate a ghost in frightened state.
#[derive(Deref)]
pub struct PacmanEatsGhost(Entity);

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PacmanMovementPlugin)
            .add_event::<PacmanKilled>()
            .add_event::<PacmanEatsGhost>()
            .add_startup_system(spawn_pacman)
            .add_system(set_direction_based_on_keyboard_input)
            .add_system(change_appearance_when_direction_changed.after(set_direction_based_on_keyboard_input))
            .add_system(pacman_hits_ghost)
            .add_system(reset_pacman_when_he_died_and_has_lives)
        ;
    }
}

fn set_direction_based_on_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Pacman>>,
) {
    for mut direction in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *direction = Left
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *direction = Right
        }

        if keyboard_input.pressed(KeyCode::Up) {
            *direction = Up
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *direction = Down
        }
    }
}

fn change_appearance_when_direction_changed(
    mut query: Query<(&Direction, &mut Transform), (With<Pacman>, Changed<Direction>)>
) {
    for (direction, mut transform) in query.iter_mut() {
        match direction {
            Up => transform.rotation = Rotation::D270.quat_z(),
            Down => transform.rotation = Rotation::D90.quat_z(),
            Left => transform.rotation = Rotation::D180.quat_z(),
            Right => transform.rotation = Rotation::D0.quat_z(),
        }
    }
}

fn pacman_hits_ghost(
    mut killed_event_writer: EventWriter<PacmanKilled>,
    mut eat_event_writer: EventWriter<PacmanEatsGhost>,
    pacman_query: Query<&Position, With<Pacman>>,
    ghost_query: Query<(Entity, &Position, &State), With<Ghost>>,
) {
    for pacman_position in pacman_query.iter() {
        for (entity, ghost_position, state) in ghost_query.iter() {
            if pacman_position != ghost_position { continue; }

            if let State::Scatter | State::Chase = state {
                killed_event_writer.send(PacmanKilled)
            }

            if let State::Frightened = state {
                eat_event_writer.send(PacmanEatsGhost(entity))
            }
        }
    }
}

fn reset_pacman_when_he_died_and_has_lives(
    pacman_spawn: Res<PacmanSpawn>,
    mut event_reader: EventReader<PacmanKilled>,
    live_query: Query<&Life>,
    mut pacman_query: Query<&mut Transform, With<Pacman>>,
) {
    for _ in event_reader.iter() {
        if live_query.iter().count() == 0 { return; }

        for mut transform in pacman_query.iter_mut() {
            transform.translation = **pacman_spawn
        }
    }
}