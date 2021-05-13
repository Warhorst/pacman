use std::time::Duration;

use bevy::prelude::*;

use crate::common::Position;
use crate::energizer::EnergizerEaten;
use crate::ghosts::Ghost;
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

pub(super) struct GhostEaten {
    pub entity: Entity,
}

/// The schedule of a ghost determines the state the ghost has after a certain time passed
/// on a certain level.
/// The last phase of a schedule will be active until the level ends, even if its timer is finished.
pub struct Schedule {
    phases: Vec<Phase>,
}

impl Schedule {
    pub fn new(phases: Vec<Phase>) -> Self {
        Schedule { phases }
    }

    pub fn state_after_tick(&mut self, elapsed_time: Duration) -> State {
        self.current_phase_mut().progress(elapsed_time);
        if self.current_phase().is_finished() {
            self.start_next_phase()
        }
        self.current_phase().active_state
    }

    pub fn current_state(&self) -> State {
        self.current_phase().active_state
    }

    fn current_phase(&self) -> &Phase {
        &self.phases[0]
    }

    fn current_phase_mut(&mut self) -> &mut Phase {
        &mut self.phases[0]
    }

    fn start_next_phase(&mut self) {
        if self.phases.len() > 1 {
            self.phases.remove(0);
        }
    }
}

/// A Phase is a time range where a specific state for a specific ghost is active.
pub struct Phase {
    active_state: State,
    remaining_time: Timer,
}

impl Phase {
    pub fn new(active_state: State, duration: f32) -> Self {
        Phase {
            active_state,
            remaining_time: Timer::from_seconds(duration, false),
        }
    }

    pub fn progress(&mut self, elapsed_time: Duration) {
        self.remaining_time.tick(elapsed_time);
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_time.finished()
    }
}

pub struct StateSetPlugin;

impl Plugin for StateSetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<GhostEaten>()
            .insert_resource(FrightenedTimer::new())
            .add_system(set_frightened_next_state.system())
            .add_system(set_spawned_next_state.system())
            .add_system(set_chase_and_scatter_next_state.system())
            .add_system(set_eaten_next_state.system())
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

fn set_frightened_next_state(
    time: Res<Time>,
    mut frightened_timer: ResMut<FrightenedTimer>,
    mut query: Query<(&mut State, &Schedule), With<Ghost>>,
) {
    frightened_timer.tick(time.delta());

    for (mut state, schedule) in query.iter_mut() {
        if *state != Frightened { continue; }

        if frightened_timer.is_finished() {
            *state = schedule.current_state()
        }
    }
}

fn set_spawned_next_state(
    time: Res<Time>,
    board: Res<Board>,
    mut query: Query<(&mut State, &mut Schedule, &Position), With<Ghost>>,
) {
    for (mut state, mut schedule, position) in query.iter_mut() {
        if *state != Spawned { continue; }

        if board.type_of_position(position) == &GhostWall {
            *state = update_and_get_state(time.delta(), &mut schedule)
        }
    }
}

fn set_chase_and_scatter_next_state(
    time: Res<Time>,
    mut query: Query<(&mut State, &mut Schedule), With<Ghost>>,
) {
    for (mut state, mut schedule) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        *state = update_and_get_state(time.delta(), &mut schedule)
    }
}

fn set_eaten_next_state(
    board: Res<Board>,
    mut query: Query<(&mut State, &Position)>
) {
    for (mut state, position) in query.iter_mut() {
        if board.type_of_position(position) == &GhostSpawn {
            *state = Spawned
        }
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

fn update_and_get_state(elapsed_time: Duration, schedule: &mut Schedule) -> State {
    schedule.state_after_tick(elapsed_time)
}