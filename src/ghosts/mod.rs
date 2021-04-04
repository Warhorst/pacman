use bevy::prelude::*;

use components::Schedule;

use crate::common::Movement;
use crate::common::Position;
use crate::events::{EnergizerEaten, GhostPassedTunnel};
use crate::ghosts::components::{Ghost, Target};
use crate::ghosts::mover::Mover;
use crate::ghosts::spawner::Spawner;
use crate::ghosts::state_setter::StateSetter;
use crate::ghosts::target_setter::TargetSetter;
use crate::map::board::Board;
use crate::pacman::Pacman;

use self::components::State;
use self::components::State::*;

pub mod components;
mod target_setter;
mod mover;
mod spawner;
mod state_setter;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ghosts.system())
            .add_system(set_target.system())
            .add_system(update_position.system())
            .add_system(update_state.system())
            .add_system(move_ghosts.system())
            .add_system(ghost_passed_tunnel.system())
            .add_system(make_ghosts_vulnerable.system());
    }
}

fn spawn_ghosts(commands: &mut Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    Spawner::new(commands, &board, &mut materials).spawn()
}

fn move_ghosts(time: Res<Time>, board: Res<Board>, mut query: Query<(&Movement, &mut Target, &mut Transform), With<Ghost>>) {
    for (movement, mut target, mut transform) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds(),
                   movement,
                   &mut target,
                   &mut transform.translation)
            .move_ghost();
    }
}

/// TODO: Position is useless as component, remove
fn update_position(board: Res<Board>, mut query: Query<(&mut Position, &Transform), With<Ghost>>) {
    for (mut position, transform) in query.iter_mut() {
        *position = board.position_of_coordinates(&transform.translation);
    }
}

fn update_state(time: Res<Time>, board: Res<Board>, mut query: Query<(&Position, &mut State, &mut Schedule), With<Ghost>>) {
    for (position, mut state, mut schedule) in query.iter_mut() {
        StateSetter::new(&mut state, position, &mut schedule, &board, time.delta_seconds()).set_next_state();
    }
}

fn set_target(board: Res<Board>,
              mut ghost_query: Query<(&Ghost, &Position, &mut Target, &mut Movement, &State)>,
              pacman_query: Query<&Position, With<Pacman>>) {
    for (ghost, ghost_position, mut target, mut movement, state) in ghost_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            TargetSetter::new(&board, &ghost_position, &mut movement, &mut target, &state, &ghost, pacman_position).set_target()
        }
    }
}

fn ghost_passed_tunnel(mut ghost_passed_event_reader: Local<EventReader<GhostPassedTunnel>>,
                       ghost_passed_events: Res<Events<GhostPassedTunnel>>,
                       mut query: Query<(Entity, &mut Target), With<Ghost>>) {
    for event in ghost_passed_event_reader.iter(&ghost_passed_events) {
        for (entity, mut target) in query.iter_mut() {
            if entity == event.entity {
                target.clear()
            }
        }
    }
}

fn make_ghosts_vulnerable(mut energizer_eaten_event_reader: Local<EventReader<EnergizerEaten>>,
                          energizer_eaten_events: Res<Events<EnergizerEaten>>,
                          board: Res<Board>,
                          mut query: Query<(&mut Target, &mut Movement, &mut State, &Transform), With<Ghost>>) {
    for _ in energizer_eaten_event_reader.iter(&energizer_eaten_events) {
        for (mut target, mut movement, mut state, transform) in query.iter_mut() {
            target.clear();
            movement.reverse();
            *state = Frightened;

            // TODO this seems like general target troubleshooting and should be moved to a more general system.
            if board.coordinates_directing_to_center(movement.get_direction(), transform.translation) {
                target.set_to(board.position_of_coordinates(&transform.translation))
            }
        }
    }
}