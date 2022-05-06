use std::time::Duration;

use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::energizer::EnergizerEaten;
use crate::ghosts::Ghost;
use crate::ghosts::schedule::Schedule;
use crate::ghosts::target::Target;
use crate::map::board::Board;
use crate::map::FieldType::{GhostSpawn, GhostWall};
use crate::pacman::Pacman;
use crate::common::MoveDirection::*;

use self::State::*;

/// The different states of a ghost
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
#[derive(Copy, Clone, Component, Debug, PartialOrd, PartialEq)]
pub enum State {
    Spawned,
    Chase,
    Scatter,
    Eaten,
    Frightened,
}

pub struct StateSetPlugin;

impl Plugin for StateSetPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FrightenedTimer::new())
            .add_system(set_frightened_next_state)
            .add_system(set_spawned_next_state)
            .add_system(set_chase_and_scatter_next_state)
            .add_system(set_eaten_next_state)
            .add_system(update_frightened_timer)
            .add_system(set_frightened_when_pacman_ate_energizer)
            .add_system(set_frightened_when_pacman_ate_energizer_and_ghost_has_no_target)
            .add_system(set_eaten_when_hit_by_pacman);
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
    mut query: Query<&mut State, With<Ghost>>,
) {
    for mut state in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        if *state != schedule.current_state() {
            *state = schedule.current_state();
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
    event_reader: EventReader<EnergizerEaten>,
    mut frightened_timer: ResMut<FrightenedTimer>,
    mut query: Query<(&mut State, &mut MoveDirection, &mut Target), With<Ghost>>,
) {
    if event_reader.is_empty() { return; }
    frightened_timer.start();

    for (mut state, mut direction, mut target) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        *state = Frightened;

        let position_ghost_came_from = match *direction {
            Up => Position::new(target.x(), target.y() - 1),
            Down => Position::new(target.x(), target.y() + 1),
            Left => Position::new(target.x() + 1, target.y()),
            Right => Position::new(target.x() - 1, target.y())
        };

        direction.reverse();
        *target = Target(position_ghost_came_from);
    }
}

fn set_frightened_when_pacman_ate_energizer_and_ghost_has_no_target(
    event_reader: EventReader<EnergizerEaten>,
    mut frightened_timer: ResMut<FrightenedTimer>,
    mut query: Query<(&mut State, &mut MoveDirection), (With<Ghost>, Without<Target>)>,
) {
    if event_reader.is_empty() { return; }
    frightened_timer.start();

    for (mut state, mut direction) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        *state = Frightened;
        direction.reverse();
    }
}

fn set_eaten_when_hit_by_pacman(
    mut ghost_query: Query<(&Position, &mut State), With<Ghost>>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (ghost_position, mut state) in ghost_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            if *state == Frightened && ghost_position == pacman_position {
                *state = Eaten;
            }
        }
    }
}