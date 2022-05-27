use std::time::Duration;

use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::energizer::EnergizerEaten;
use crate::ghosts::schedule::ScheduleChanged;
use crate::ghosts::target::Target;
use crate::pacman::Pacman;
use crate::common::MoveDirection::*;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, DotCounter, Ghost, GhostType, Inky, Pinky};
use crate::level::Level;
use crate::ghosts::schedule::Schedule;
use crate::state_skip_if;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_system(update_frightened_state)
                    .with_system(update_spawned_state)
                    .with_system(update_chase_and_scatter_state)
                    .with_system(update_eaten_state::<Blinky>)
                    .with_system(update_eaten_state::<Pinky>)
                    .with_system(update_eaten_state::<Inky>)
                    .with_system(update_eaten_state::<Clyde>)
                    .with_system(update_frightened_timer)
                    .with_system(set_frightened_when_pacman_ate_energizer)
                    .with_system(set_eaten_when_hit_by_pacman)
                    .label(StateSetter)
            )
        ;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub struct StateSetter;

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
    mut query: Query<(&mut MoveDirection, &mut State, &Transform, &DotCounter)>,
) {
    for (mut direction, mut state, transform, dot_counter) in query.iter_mut() {
        state_skip_if!(state != State::Spawned);

        if dot_counter.is_active() { continue; }

        let coordinates = transform.translation;

        if coordinates == ghost_house.coordinates_in_front_of_entrance() {
            *state = schedule.current_state();
            *direction = Left;
        }
    }
}

fn update_chase_and_scatter_state(
    mut event_reader: EventReader<ScheduleChanged>,
    mut query: Query<(&mut MoveDirection, &mut Target, &mut State, &Transform), With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    for event in event_reader.iter() {
        for (mut direction, mut target, mut state, transform) in query.iter_mut() {
            state_skip_if!(state != State::Scatter | State::Chase);

            *state = **event;

            let target_coordinates = if target.is_set() {
                target.get()
            } else {
                transform.translation
            };

            let target_position = Position::from(target_coordinates);
            let coordinates_ghost_came_from = Vec3::from(target_position.get_neighbour_in_direction(&direction.opposite()).position);

            direction.reverse();
            target.set(coordinates_ghost_came_from);
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
        state_skip_if!(state != State::Frightened);
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
        state_skip_if!(state != State::Eaten);

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
    mut query: Query<(&mut MoveDirection, &mut Target, &mut State, &Transform), With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (mut direction, mut target, mut state, transform) in query.iter_mut() {
        state_skip_if!(state != State::Scatter | State::Chase);

        let target_coordinates = if target.is_set() {
            target.get()
        } else {
            transform.translation
        };
        let target_position = Position::from(target_coordinates);
        let coordinates_ghost_came_from = Vec3::from(target_position.get_neighbour_in_direction(&direction.opposite()).position);

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
                state_skip_if!(state != State::Frightened);
                *state = State::Eaten;
            }
        }
    }
}

#[macro_export]
macro_rules! state_skip_if {
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