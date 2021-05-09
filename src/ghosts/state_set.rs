use std::time::Duration;

use bevy::prelude::*;

use crate::common::Position;
use crate::events::EnergizerEaten;
use crate::ghosts::components::{Ghost, Schedule, State};
use crate::map::board::Board;
use crate::map::FieldType::GhostWall;

use super::components::State::*;

pub struct StateSetPlugin;

impl Plugin for StateSetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(FrightenedTimer::new())
            .add_system(set_frightened_next_state.system())
            .add_system(set_spawned_next_state.system())
            .add_system(set_chase_and_scatter_next_state.system())
            .add_system(set_frightened_when_pacman_ate_energizer.system());
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
        if !(*state == Chase || *state == Scatter) { continue; }

        *state = update_and_get_state(time.delta(), &mut schedule)
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
            *state = Frightened;
        }
    }
}

fn update_and_get_state(elapsed_time: Duration, schedule: &mut Schedule) -> State {
    schedule.state_after_tick(elapsed_time)
}