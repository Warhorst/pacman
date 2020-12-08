use bevy::prelude::*;

use State::*;

use crate::common::Movement;
use crate::common::Position;
use crate::events::GhostPassedTunnel;
use crate::ghosts::mover::Mover;
use crate::ghosts::spawner::Spawner;
use crate::ghosts::target_set_strategy::{ScatterStrategy, SpawnedStrategy};
use crate::ghosts::target_setter::TargetSetter;
use crate::map::board::Board;
use crate::map::FieldType::*;

mod target_setter;
mod mover;
mod spawner;
mod target_set_strategy;
mod state;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Target {
    target: Option<Position>
}

impl Target {
    pub fn new() -> Self {
        Target { target: None }
    }

    pub fn is_set(&self) -> bool {
        self.target.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    pub fn set_to(&mut self, position: Position) {
        self.target = Some(position)
    }

    pub fn get_position(&self) -> &Position {
        &self.target.as_ref().expect("The target should be set at this point")
    }

    pub fn clear(&mut self) {
        self.target = None
    }
}

/// The different states of a ghost
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum State {
    Spawned,
    // Chase,
    Scatter,
    // Eaten,
    // Frightened,
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ghosts.system())
            .add_system(set_target.system())
            .add_system(update_position.system())
            .add_system(update_state.system())
            .add_system(move_ghosts.system())
            .add_system(ghost_passed_tunnel.system());
    }
}

fn spawn_ghosts(commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    Spawner::new(commands, &board, &mut materials).spawn()
}

fn move_ghosts(time: Res<Time>, board: Res<Board>, mut query: Query<With<Ghost, (&Movement, &mut Target, &mut Transform)>>) {
    for (movement, mut target, mut transform) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds,
                   movement,
                   &mut target,
                   &mut transform.translation)
            .move_ghost();
    }
}

fn update_position(board: Res<Board>, mut query: Query<With<Ghost, (&mut Position, &Transform)>>) {
    for (mut position, transform) in query.iter_mut() {
        *position = board.position_of_coordinates(&transform.translation);
    }
}

fn update_state(board: Res<Board>, mut query: Query<With<Ghost, (&Position, &mut State)>>) {
    for (position, mut state) in query.iter_mut() {
        if *state == Spawned && *board.type_of_position(position) == GhostWall {
            *state = Scatter
        }
    }
}

/// Set the ghosts target if he does not have one.
fn set_target(board: Res<Board>, mut query: Query<(&Ghost, &Position, &mut Target, &mut Movement, &State)>) {
    for (ghost, position, mut target, mut movement, state) in query.iter_mut() {
        let owned_movement = movement.clone();
        let mut target_setter = TargetSetter::new(&mut target, &mut movement);
        match state {
            Spawned => target_setter.set_target(SpawnedStrategy::new(&board, &position, owned_movement)),
            Scatter => target_setter.set_target(ScatterStrategy::new(&board, &position, owned_movement, &ghost)),
        }
    }
}

fn ghost_passed_tunnel(mut ghost_passed_event_reader: Local<EventReader<GhostPassedTunnel>>,
                       ghost_passed_events: Res<Events<GhostPassedTunnel>>,
                       mut query: Query<With<Ghost, (Entity, &mut Target)>>) {
    for event in ghost_passed_event_reader.iter(&ghost_passed_events) {
        for (entity, mut target) in query.iter_mut() {
            if entity == event.entity {
                target.clear()
            }
        }
    }
}