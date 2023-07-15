use std::fmt::Formatter;
use bevy::ecs::event::Event;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use crate::game::direction::Direction;
use crate::game::position::Position;
use crate::game::edibles::energizer::EnergizerOver;
use crate::game_state::GameState::*;
use crate::game::target::Target;
use crate::game::ghosts::Ghost;
use crate::game::schedule::Schedule;
use crate::game::interactions::{EEnergizerEaten, EGhostEaten, LPacmanEnergizerHitDetection, LPacmanGhostHitDetection};
use crate::game::helper::XYEqual;
use crate::game::ghosts::Ghost::{Blinky, Pinky};
use crate::game::state::State::*;
use crate::game::map::ghost_house::GhostSpawn;

pub(in crate::game) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_state
                .after(LPacmanGhostHitDetection)
                .after(LPacmanEnergizerHitDetection)
                .in_set(StateSetter)
                .run_if(in_state(Running)),
            )
            .add_systems(Update, update_state_on_eaten_pause.run_if(in_state(GhostEatenPause)))
        ;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct StateSetter;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Scatter,
    Chase,
    Frightened,
    Eaten,
    Spawned,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct StateUpdateComponents<'a> {
    entity: Entity,
    ghost: &'a Ghost,
    state: &'a mut State,
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
}

fn update_state(
    schedule: Res<Schedule>,
    energizer_over_events: EventReader<EnergizerOver>,
    energizer_eaten_events: EventReader<EEnergizerEaten>,
    ghost_eaten_events: EventReader<EGhostEaten>,
    spawns_query: Query<&GhostSpawn>,
    mut query: Query<StateUpdateComponents, With<Ghost>>,
) {
    let energizer_eaten = energizer_eaten(energizer_eaten_events);
    let energizer_over = energizer_over(energizer_over_events);
    let ghost_eaten_events = collect_events(ghost_eaten_events);

    for mut components in &mut query {
        if ghost_eaten(components.entity, &ghost_eaten_events) {
            *components.state = Eaten;
            continue;
        }

        if energizer_eaten && matches!(*components.state, Chase | Scatter) {
            process_energizer_eaten(&mut components);
            continue;
        }

        match *components.state {
            Spawned => process_spawned(&schedule, &mut components, &spawns_query),
            Scatter | Chase => process_scatter_chase(&schedule, &mut components),
            Frightened => process_frightened(&schedule, energizer_over, &mut components),
            Eaten => process_eaten(&mut components, &spawns_query),
        }
    }
}

fn update_state_on_eaten_pause(
    schedule: Res<Schedule>,
    spawns_query: Query<&GhostSpawn>,
    mut query: Query<StateUpdateComponents, With<Ghost>>,
) {
    for mut components in &mut query {
        match *components.state {
            Spawned => process_spawned(&schedule, &mut components, &spawns_query),
            Eaten => process_eaten(&mut components, &spawns_query),
            _ => continue
        }
    }
}

fn collect_events<'a, E: Copy + Event>(mut event_reader: EventReader<E>) -> Vec<E> {
    event_reader.iter().map(|e| *e).collect()
}

fn energizer_eaten(mut events: EventReader<EEnergizerEaten>) -> bool {
    events.iter().count() > 0
}

fn energizer_over(mut events: EventReader<EnergizerOver>) -> bool {
    events.iter().count() > 0
}

fn ghost_eaten(entity: Entity, eaten_events: &Vec<EGhostEaten>) -> bool {
    eaten_events
        .iter()
        .filter(|e| e.0 == entity)
        .count() > 0
}

fn process_energizer_eaten(
    components: &mut StateUpdateComponentsItem
) {
    let target_coordinates = if components.target.is_set() {
        components.target.get()
    } else {
        components.transform.translation
    };
    let target_position = Position::from_vec(&target_coordinates);
    let coordinates_ghost_came_from = target_position.get_neighbour_in_direction(&components.direction.opposite()).position.to_vec(0.0);

    *components.state = Frightened;
    components.direction.reverse();
    components.target.set(coordinates_ghost_came_from);
}

fn process_spawned(
    schedule: &Schedule,
    components: &mut StateUpdateComponentsItem,
    spawns_query: &Query<&GhostSpawn>,
) {
    let blinky_spawn = spawns_query.iter().filter(|spawn| spawn.ghost == Blinky).next().expect("blinky should have a spawn");

    let coordinates = components.transform.translation;
    if coordinates.xy_equal(&blinky_spawn.coordinates) {
        *components.state = schedule.current_state();
        *components.direction = blinky_spawn.spawn_direction;
    }
}

/// If the current schedule is different to the ghosts state, the new state is the current schedule and
/// the ghost reverses his location.
fn process_scatter_chase(
    schedule: &Schedule,
    components: &mut StateUpdateComponentsItem,
) {
    let schedule_state = schedule.current_state();

    if let (Chase, Scatter) | (Scatter, Chase) = (*components.state, schedule_state) {
        *components.state = schedule_state;

        let target_coordinates = if components.target.is_set() {
            components.target.get()
        } else {
            components.transform.translation
        };

        let target_position = Position::from_vec(&target_coordinates);
        let coordinates_ghost_came_from = target_position.get_neighbour_in_direction(&components.direction.opposite()).position.to_vec(0.0);

        components.direction.reverse();
        components.target.set(coordinates_ghost_came_from);
    }
}

fn process_frightened(
    schedule: &Schedule,
    energizer_over: bool,
    components: &mut StateUpdateComponentsItem,
) {
    if energizer_over {
        *components.state = schedule.current_state()
    }
}

fn process_eaten(
    components: &mut StateUpdateComponentsItem,
    spawns_query: &Query<&GhostSpawn>,
) {
    let respawn = spawns_query
        .iter()
        .filter(|spawn| match *components.ghost {
            Blinky => spawn.ghost == Pinky,
            _ => spawn.ghost == *components.ghost
        })
        .next()
        .expect("every ghost should have a spawn");
    let coordinates = components.transform.translation;

    if coordinates.xy_equal(&respawn.coordinates) {
        *components.state = Spawned
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}