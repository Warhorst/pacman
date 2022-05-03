use std::ops::DerefMut;

use bevy::prelude::*;

use crate::common::{MoveComponents, Movement, Position};
use crate::common::Direction::*;
use crate::common::Movement::*;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::ghosts::state::State::*;
use crate::map::board::Board;
use crate::pacman::mover::Mover;
use crate::pacman::spawner::Spawner;

mod mover;
mod spawner;

#[derive(Component)]
pub struct Pacman;

/// Fired when pacman was killed by a ghost.
pub struct PacmanKilled;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PacmanKilled>()
            .add_startup_system(spawn_pacman)
            .add_system(move_pacman)
            .add_system(set_direction)
            .add_system(ghost_hits_pacman);
    }
}

fn spawn_pacman(commands: Commands, board: Res<Board>) {
    Spawner::new(commands, &board).spawn()
}

fn move_pacman(time: Res<Time>,
               board: Res<Board>,
               mut query: Query<MoveComponents, With<Pacman>>) {
    for (mut transform, mut position, mut movement) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds(),
                   movement.deref_mut(),
                   position.deref_mut(),
                   &mut transform.translation)
            .move_pacman()
    }
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>,
                 mut query: Query<&mut Movement, With<Pacman>>) {
    for mut movement in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *movement = Moving(Left)
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *movement = Moving(Right)
        }

        if keyboard_input.pressed(KeyCode::Up) {
            *movement = Moving(Up)
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *movement = Moving(Down)
        }
    }
}

fn ghost_hits_pacman(
    mut commands: Commands,
    mut event_writer: EventWriter<PacmanKilled>,
    pacman_query: Query<(Entity, &Position), With<Pacman>>,
    ghost_query: Query<(&Position, &State), With<Ghost>>,
) {
    for (pacman_entity, pacman_position) in pacman_query.iter() {
        for (ghost_position, state) in ghost_query.iter() {
            if pacman_position == ghost_position && !vec![&Frightened, &Eaten].contains(&state) {
                commands.entity(pacman_entity).despawn();
                event_writer.send(PacmanKilled)
            }
        }
    }
}