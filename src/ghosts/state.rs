use std::time::Duration;

use bevy::prelude::*;

use crate::common::Position;
use crate::energizer::EnergizerEaten;
use crate::ghosts::Ghost;
use crate::ghosts::schedule::Schedule;
use crate::map::board::Board;
use crate::map::FieldType::{GhostSpawn, GhostWall};
use crate::pacman::Pacman;

use self::State::*;

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
    Chase,
    Scatter,
    Eaten,
    Frightened,
}

/// Event
/// Send when a ghost was eaten by pacman.
/// TODO: Why here?
pub(super) struct GhostEaten {
    pub entity: Entity,
}

/// Event
/// Send when a phase change occurred that indicates that ghost that change on schedule shall turn around.
pub(super) struct SchedulePhaseChanged {
    pub entity: Entity,
}

pub struct StateSetPlugin;

impl Plugin for StateSetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<GhostEaten>()
            .add_event::<SchedulePhaseChanged>()
            .insert_resource(FrightenedTimer::new())
            .add_system(set_frightened_next_state.system())
            .add_system(set_spawned_next_state.system())
            .add_system(set_chase_and_scatter_next_state.system())
            .add_system(set_eaten_next_state.system())
            .add_system(update_frightened_timer.system())
            .add_system(set_frightened_when_pacman_ate_energizer.system())
            .add_system(set_eaten_when_hit_by_pacman.system());
    }
}

pub struct FrightenedTimer {
    timer: Timer,
}

impl FrightenedTimer {
    pub fn new() -> Self {
        FrightenedTimer {
            timer: Timer::from_seconds(5.0, false)
        }
    }

    pub fn start(&mut self) {
        self.timer.reset()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    // Interesting: calling 'self.finished()' does not just crash, it returns true at some point
    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }
}

fn set_spawned_next_state(
    schedule: Res<Schedule>,
    board: Res<Board>,
    mut query: Query<(&mut State, &Position), With<Ghost>>,
) {
    for (mut state, position) in query.iter_mut() {
        if *state != Spawned { continue; }

        if board.type_of_position(position) == &GhostWall {
            *state = schedule.current_state()
        }
    }
}

fn set_chase_and_scatter_next_state(
    schedule: Res<Schedule>,
    mut event_writer: EventWriter<SchedulePhaseChanged>,
    mut query: Query<(Entity, &mut State), With<Ghost>>,
) {
    for (entity, mut state) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        if *state != schedule.current_state() {
            *state = schedule.current_state();
            event_writer.send(SchedulePhaseChanged { entity })
        }
    }
}

fn set_frightened_next_state(
    schedule: Res<Schedule>,
    frightened_timer: Res<FrightenedTimer>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    for mut state in query.iter_mut() {
        if *state != Frightened { continue; }

        if frightened_timer.is_finished() {
            *state = schedule.current_state()
        }
    }
}

fn set_eaten_next_state(
    board: Res<Board>,
    mut query: Query<(&mut State, &Position)>,
) {
    for (mut state, position) in query.iter_mut() {
        if board.type_of_position(position) == &GhostSpawn {
            *state = Spawned
        }
    }
}

fn update_frightened_timer(
    time: Res<Time>,
    mut timer: ResMut<FrightenedTimer>,
) {
    if !timer.is_finished() {
        timer.tick(time.delta())
    }
}

fn set_frightened_when_pacman_ate_energizer(
    mut event_reader: EventReader<EnergizerEaten>,
    mut frightened_timer: ResMut<FrightenedTimer>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    for _ in event_reader.iter() {
        frightened_timer.start();
        for mut state in query.iter_mut() {
            if *state != Eaten {
                *state = Frightened;
            }
        }
    }
}

fn set_eaten_when_hit_by_pacman(
    mut event_writer: EventWriter<GhostEaten>,
    mut ghost_query: Query<(Entity, &Position, &mut State), With<Ghost>>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, ghost_position, mut state) in ghost_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            if *state == Frightened && ghost_position == pacman_position {
                *state = Eaten;
                event_writer.send(GhostEaten { entity })
            }
        }
    }
}