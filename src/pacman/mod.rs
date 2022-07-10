use bevy::prelude::*;

use crate::common::Direction;
use crate::common::position::ToPosition;
use crate::common::Direction::*;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::lives::Life;
use crate::pacman::spawn::{PacmanSpawn, spawn_pacman};
use crate::pacman::movement::PacmanMovementPlugin;
use crate::pacman::textures::{Animation, update_animation, update_pacman_appearance};

mod movement;
mod spawn;
mod textures;

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
            .insert_resource(Animation::new())
            .add_plugin(PacmanMovementPlugin)
            .add_event::<PacmanKilled>()
            .add_event::<PacmanEatsGhost>()
            .add_startup_system(spawn_pacman)
            .add_system(set_direction_based_on_keyboard_input)
            .add_system(update_pacman_appearance.after(set_direction_based_on_keyboard_input))
            .add_system(pacman_hits_ghost)
            .add_system(reset_pacman_when_he_died_and_has_lives)
            .add_system(update_animation)
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

fn pacman_hits_ghost(
    mut killed_event_writer: EventWriter<PacmanKilled>,
    mut eat_event_writer: EventWriter<PacmanEatsGhost>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in pacman_query.iter() {
        for (entity, ghost_transform, state) in ghost_query.iter() {
            if pacman_transform.pos() == ghost_transform.pos() {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(PacmanKilled)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(PacmanEatsGhost(entity))
                }
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