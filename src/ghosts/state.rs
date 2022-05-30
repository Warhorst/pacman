use std::time::Duration;

use bevy::prelude::*;

use crate::common::{Direction, Position};
use crate::energizer::EnergizerEaten;
use crate::ghosts::schedule::ScheduleChanged;
use crate::ghosts::target::Target;
use crate::pacman::Pacman;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, DotCounter, Ghost, GhostType, Inky, Pinky};
use crate::level::Level;
use crate::ghosts::schedule::Schedule;
use crate::state_skip_if;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FrightenedTimerNew::new())
            .add_system_set(
                SystemSet::new()
                    .with_system(update_frightened_state)
                    .with_system(update_spawned_state)
                    .with_system(update_chase_and_scatter_state)
                    .with_system(update_eaten_state::<Blinky>)
                    .with_system(update_eaten_state::<Pinky>)
                    .with_system(update_eaten_state::<Inky>)
                    .with_system(update_eaten_state::<Clyde>)
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

pub struct FrightenedTimerNew {
    timer: Option<Timer>
}

impl FrightenedTimerNew {
    pub fn new() -> Self {
        FrightenedTimerNew {
            timer: None
        }
    }

    /// Ghost are frightened for the full time at level 1.
    /// Their time gets reduced every level until level 19, were they aren't frightened at all.
    ///
    /// This is only speculation. It is unclear how the time a ghost is frightened
    /// gets calculated.
    pub fn start(&mut self, level: &Level) {
        let level = **level as f32 - 1.0;
        let time = f32::max(8.0 - level * (8.0 / 18.0), 0.0);
        self.timer = Some(Timer::from_seconds(time, false))
    }

    pub fn tick(&mut self, delta: Duration) {
        if let Some(ref mut t) = self.timer {
            t.tick(delta);
        }

        if self.is_finished() {
            self.timer = None
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.timer {
            Some(ref t) => t.finished(),
            None => true
        }
    }
}

/// Update the spawned state. A ghost is no longer spawned if he stands in front of
/// the ghost house. When he left the ghost house, he always turns to the right.
fn update_spawned_state(
    schedule: Res<Schedule>,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(&mut Direction, &mut State, &Transform, &DotCounter)>,
) {
    for (mut direction, mut state, transform, dot_counter) in query.iter_mut() {
        state_skip_if!(state != State::Spawned);

        if dot_counter.is_active() { continue; }

        let coordinates = transform.translation;

        if coordinates == ghost_house.coordinates_in_front_of_entrance() {
            *state = schedule.current_state();
            *direction = ghost_house.entrance_direction.rotate_left();
        }
    }
}

fn update_chase_and_scatter_state(
    mut event_reader: EventReader<ScheduleChanged>,
    mut query: Query<(&mut Direction, &mut Target, &mut State, &Transform), With<Ghost>>,
) {
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
    time: Res<Time>,
    schedule: Res<Schedule>,
    mut frightened_timer: ResMut<FrightenedTimerNew>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    frightened_timer.tick(time.delta());

    for mut state in query.iter_mut() {
        state_skip_if!(state != State::Frightened);
        if frightened_timer.is_finished() {
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

fn set_frightened_when_pacman_ate_energizer(
    level: Res<Level>,
    mut frightened_timer: ResMut<FrightenedTimerNew>,
    mut event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(&mut Direction, &mut Target, &mut State, &Transform), With<Ghost>>,
) {
    for _ in event_reader.iter() {
        frightened_timer.start(&level);

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
    ($components:ident.$state:ident = $pattern:pat) => {
        if let $pattern = *$components.$state { continue; }
    };

    ($state:ident = $pattern:pat) => {
        if let $pattern = *$state { continue; }
    };

    ($components:ident.$state:ident != $pattern:pat) => {
        match *$components.$state {
            $pattern => (),
            _ => continue
        }
    };

    ($state:ident != $pattern:pat) => {
        match *$state {
            $pattern => (),
            _ => continue
        }
    };
}