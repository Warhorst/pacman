use std::time::Duration;

use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::energizer::EnergizerEaten;
use crate::ghosts::schedule::ScheduleChanged;
use crate::ghosts::target::Target_;
use crate::pacman::Pacman;
use crate::common::MoveDirection::*;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::level::Level;
use crate::ghosts::schedule::Schedule;
use crate::skip_if;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_frightened_state)
            .add_system(update_spawned_state)
            .add_system(update_chase_and_scatter_state)
            .add_system(update_eaten_state::<Blinky>)
            .add_system(update_eaten_state::<Pinky>)
            .add_system(update_eaten_state::<Inky>)
            .add_system(update_eaten_state::<Clyde>)
            .add_system(update_frightened_timer)
            .add_system(set_frightened_when_pacman_ate_energizer)
            .add_system(set_eaten_when_hit_by_pacman)
            .add_system(reverse_when_schedule_changed)
        ;
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Scatter,
    Chase,
    Frightened,
    Eaten,
    Spawned,
}

pub struct FrightenedTimer {
    timer: Timer,
}

impl FrightenedTimer {
    /// Ghost are frightened for the full time at level 1.
    /// Their time gets reduced every level until level 19, were they aren't frightened at all.
    ///
    /// This is only speculation. It is unclear how the time a ghost is frightened
    /// gets calculated.
    pub fn start(level: &Level) -> Self {
        let level = **level as f32 - 1.0;
        let time = f32::max(8.0 - level * (8.0 / 18.0), 0.0);

        FrightenedTimer {
            timer: Timer::from_seconds(time, false)
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }
}

/// Update the spawned state. A ghost is no longer spawned if he stands in front of
/// the ghost house. When he left the ghost house, he always turns to the right.
fn update_spawned_state(
    schedule: Res<Schedule>,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(&mut MoveDirection, &mut State, &Transform)>,
) {
    for (mut direction, mut state, transform) in query.iter_mut() {
        skip_if!(state != State::Spawned);
        let coordinates = transform.translation;

        if coordinates == ghost_house.coordinates_in_front_of_entrance() {
            *state = schedule.current_state();
            *direction = Left;
        }
    }
}

fn update_chase_and_scatter_state(
    mut event_reader: EventReader<ScheduleChanged>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    for event in event_reader.iter() {
        for mut state in query.iter_mut() {
            skip_if!(state != State::Scatter | State::Chase);
            *state = **event;
        }
    }
}

fn update_frightened_state(
    schedule: Res<Schedule>,
    frightened_timer: Option<Res<FrightenedTimer>>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    let frightened_time_over = match frightened_timer {
        Some(ref timer) => timer.is_finished(),
        _ => true
    };

    for mut state in query.iter_mut() {
        skip_if!(state != State::Frightened);
        if frightened_time_over {
            *state = schedule.current_state();
        }
    }
}

fn update_eaten_state<G: Component + GhostType + 'static>(
    ghost_house: Res<GhostHouse>,
    mut query: Query<(&Transform, &mut State), With<G>>,
) {
    for (transform, mut state) in query.iter_mut() {
        skip_if!(state != State::Eaten);

        let coordinates = transform.translation;

        if coordinates == ghost_house.respawn_coordinates_of::<G>() {
            *state = State::Spawned
        }
    }
}

fn update_frightened_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Option<ResMut<FrightenedTimer>>,
) {
    match timer {
        Some(ref t) if t.is_finished() => {
            commands.remove_resource::<FrightenedTimer>()
        }
        Some(ref mut t) => t.tick(time.delta()),
        _ => return
    }
}

fn set_frightened_when_pacman_ate_energizer(
    mut commands: Commands,
    level: Res<Level>,
    event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(&mut MoveDirection, &mut Target_, &mut State, &Transform), With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (mut direction, mut target, mut state, transform) in query.iter_mut() {
        skip_if!(state != State::Scatter | State::Chase);

        let target_coordinates = if target.is_set() {
            target.get()
        } else {
            transform.translation
        };

        // TODO: refactor
        let coordinates_ghost_came_from = match *direction {
            Up => Vec3::new(target_coordinates.x, target_coordinates.y - 1.0, 0.0),
            Down => Vec3::new(target_coordinates.x, target_coordinates.y + 1.0, 0.0),
            Left => Vec3::new(target_coordinates.x + 1.0, target_coordinates.y, 0.0),
            Right => Vec3::new(target_coordinates.x - 1.0, target_coordinates.y, 0.0)
        };

        *state = State::Frightened;
        direction.reverse();
        target.set(coordinates_ghost_came_from);
    }
}

fn set_eaten_when_hit_by_pacman(
    mut ghost_query: Query<(&Position, &mut State), With<Ghost>>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (ghost_position, mut state) in ghost_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            if ghost_position == pacman_position {
                skip_if!(state != State::Frightened);
                *state = State::Eaten;
            }
        }
    }
}

// TODO: why two systems?
fn reverse_when_schedule_changed(
    event_reader: EventReader<ScheduleChanged>,
    mut query: Query<(&mut MoveDirection, &mut Target_, &State, &Transform), With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    for (mut direction, mut target, state, transform) in query.iter_mut() {
        skip_if!(state != State::Scatter | State::Chase);

        let target_coordinates = if target.is_set() {
            target.get()
        } else {
            transform.translation
        };

        // TODO: refactor
        let coordinates_ghost_came_from = match *direction {
            Up => Vec3::new(target_coordinates.x, target_coordinates.y - 1.0, 0.0),
            Down => Vec3::new(target_coordinates.x, target_coordinates.y + 1.0, 0.0),
            Left => Vec3::new(target_coordinates.x + 1.0, target_coordinates.y, 0.0),
            Right => Vec3::new(target_coordinates.x - 1.0, target_coordinates.y, 0.0)
        };

        direction.reverse();
        target.set(coordinates_ghost_came_from);
    }
}

#[macro_export]
macro_rules! skip_if {
    ($state:ident = $pattern:pat) => {
        if let $pattern = *$state { continue; }
    };

    ($state:ident != $pattern:pat) => {
        match *$state {
            $pattern => (),
            _ => continue
        }
    };
}