use bevy::prelude::*;

use crate::common::{Direction, Position};
use crate::energizer::{EnergizerEaten, EnergizerOver};
use crate::ghosts::schedule::ScheduleChanged;
use crate::ghosts::target::Target;
use crate::pacman::PacmanEatsGhost;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::ghosts::schedule::Schedule;
use crate::state_skip_if;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(update_frightened_state)
                .with_system(update_spawned_state::<Blinky>)
                .with_system(update_spawned_state::<Pinky>)
                .with_system(update_spawned_state::<Inky>)
                .with_system(update_spawned_state::<Clyde>)
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

/// Update the spawned state. A ghost is no longer spawned if he stands in front of
/// the ghost house. When he left the ghost house, he always turns to the right.
fn update_spawned_state<G: GhostType + Component + 'static>(
    schedule: Res<Schedule>,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(&mut Direction, &mut State, &Transform)>,
) {
    for (mut direction, mut state, transform) in query.iter_mut() {
        state_skip_if!(state != State::Spawned);

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
    schedule: Res<Schedule>,
    mut event_reader: EventReader<EnergizerOver>,
    mut query: Query<&mut State, With<Ghost>>,
) {
    for _ in event_reader.iter() {
        for mut state in query.iter_mut() {
            state_skip_if!(state != State::Frightened);
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
    mut event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(&mut Direction, &mut Target, &mut State, &Transform), With<Ghost>>,
) {
    for _ in event_reader.iter() {
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
    mut event_reader: EventReader<PacmanEatsGhost>,
    mut ghost_query: Query<(Entity, &mut State), With<Ghost>>,
) {
    for event in event_reader.iter() {
        for (entity, mut state) in ghost_query.iter_mut() {
            if entity != **event {
                continue;
            }

            state_skip_if!(state != State::Frightened);
            *state = State::Eaten;
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