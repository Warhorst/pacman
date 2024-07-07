use std::fmt::Formatter;

use bevy::ecs::event::Event;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::prelude::*;

pub(in crate::game) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                update_state
                    .in_set(SetState)
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                Update,
                update_state_on_eaten_pause
                    .in_set(SetState)
                    .run_if(in_state(Game(GhostEatenPause))))
        ;
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
struct StateUpdateComponents<'a> {
    entity: Entity,
    ghost: &'a Ghost,
    state: &'a mut GhostState,
    target: &'a mut Target,
    direction: &'a mut Dir,
    transform: &'a Transform,
}

fn update_state(
    schedule: Res<GhostSchedule>,
    energizer_over_events: EventReader<EnergizerOver>,
    energizer_eaten_events: EventReader<EnergizerWasEaten>,
    ghost_eaten_events: EventReader<GhostWasEaten>,
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
    schedule: Res<GhostSchedule>,
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
    event_reader.read().copied().collect()
}

fn energizer_eaten(mut events: EventReader<EnergizerWasEaten>) -> bool {
    events.read().count() > 0
}

fn energizer_over(mut events: EventReader<EnergizerOver>) -> bool {
    events.read().count() > 0
}

fn ghost_eaten(entity: Entity, eaten_events: &[GhostWasEaten]) -> bool {
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
    let target_position = Pos::from_vec3(target_coordinates);
    let coordinates_ghost_came_from = target_position.neighbour_in_direction(components.direction.opposite()).to_vec3(0.0);

    *components.state = Frightened;
    *components.direction = components.direction.opposite();
    components.target.set(coordinates_ghost_came_from);
}

fn process_spawned(
    schedule: &GhostSchedule,
    components: &mut StateUpdateComponentsItem,
    spawns_query: &Query<&GhostSpawn>,
) {
    let blinky_spawn = spawns_query
        .iter()
        .find(|spawn| spawn.ghost == Blinky)
        .expect("blinky should have a spawn");

    let coordinates = components.transform.translation;
    if coordinates.xy_equal(&blinky_spawn.coordinates) {
        *components.state = schedule.current_state();
        *components.direction = blinky_spawn.spawn_direction;
    }
}

/// If the current schedule is different to the ghosts state, the new state is the current schedule and
/// the ghost reverses his location.
fn process_scatter_chase(
    schedule: &GhostSchedule,
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

        let target_position = Pos::from_vec3(target_coordinates);
        let coordinates_ghost_came_from = target_position.neighbour_in_direction(components.direction.opposite()).to_vec3(0.0);

        *components.direction = components.direction.opposite();
        components.target.set(coordinates_ghost_came_from);
    }
}

fn process_frightened(
    schedule: &GhostSchedule,
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
        .find(|spawn| match *components.ghost {
            Blinky => spawn.ghost == Pinky,
            _ => spawn.ghost == *components.ghost
        })
        .expect("every ghost should have a spawn");
    let coordinates = components.transform.translation;

    if coordinates.xy_equal(&respawn.coordinates) {
        *components.state = Spawned
    }
}

impl std::fmt::Display for GhostState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}