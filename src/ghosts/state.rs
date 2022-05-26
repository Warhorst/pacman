use std::time::Duration;

use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::energizer::EnergizerEaten;
use crate::ghosts::schedule::ScheduleChanged;
use crate::ghosts::target::Target;
use crate::pacman::Pacman;
use crate::common::MoveDirection::*;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::level::Level;
use crate::ghosts::schedule::Schedule;
use crate::ghosts::schedule::State::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_frightened_state)
            .add_system(update_spawned_state)
            .add_system(set_chase_or_scatter_state_when_not_set)
            .add_system(update_chase_and_scatter_state)
            .add_system(update_eaten_state::<Blinky>)
            .add_system(update_eaten_state::<Pinky>)
            .add_system(update_eaten_state::<Inky>)
            .add_system(update_eaten_state::<Clyde>)
            .add_system(update_frightened_timer)
            .add_system(set_frightened_when_pacman_ate_energizer)
            .add_system(set_frightened_when_pacman_ate_energizer_and_ghost_has_no_target)
            .add_system(set_eaten_when_hit_by_pacman)
            .add_system(reverse_when_schedule_changed)
            .add_system(reverse_when_schedule_changed_and_ghost_has_no_target)
        ;
    }
}

/// Indicates that a ghost is currently moving to its home
#[derive(Component)]
pub struct Scatter;

/// Indicates that a ghost is currently hunting pacman
#[derive(Component)]
pub struct Chase;

/// Indicates that a ghost is currently frightened.
#[derive(Component)]
pub struct Frightened;

/// Indicates that a ghost was eaten by pacman
#[derive(Component)]
pub struct Eaten;

/// Indicates that a ghost just started in the ghost house
#[derive(Component)]
pub struct Spawned;

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

// TODO: This might fail if a ghost is on the ghost wall and a schedule change happens -> he turns around and is trapped.
/// Update the spawned state. A ghost is no longer spawned if he stands in front of
/// the ghost house. When he left the ghost house, he always turns to the right.
fn update_spawned_state(
    mut commands: Commands,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(Entity, &mut MoveDirection, &Transform), (With<Ghost>, With<Spawned>, Without<Frightened>, Without<Eaten>)>,
) {
    for (entity, mut direction, transform) in query.iter_mut() {
        let coordinates = transform.translation;

        if coordinates == ghost_house.coordinates_in_front_of_entrance() {
            commands.entity(entity).remove::<Spawned>();
            *direction = Left;
        }
    }
}

fn set_chase_or_scatter_state_when_not_set(
    mut commands: Commands,
    schedule: Res<Schedule>,
    query: Query<Entity, (With<Ghost>, Without<Scatter>, Without<Chase>)>,
) {
    for entity in query.iter() {
        match schedule.current_state() {
            ScatterState => commands.entity(entity).insert(Scatter),
            ChaseState => commands.entity(entity).insert(Chase)
        };
    }
}

fn update_chase_and_scatter_state(
    mut commands: Commands,
    mut event_reader: EventReader<ScheduleChanged>,
    query: Query<Entity, With<Ghost>>,
) {
    if event_reader.is_empty() { return; }

    for event in event_reader.iter() {
        for entity in query.iter() {
            match **event {
                ChaseState => commands.entity(entity).remove::<Scatter>().insert(Chase),
                ScatterState => commands.entity(entity).remove::<Chase>().insert(Scatter)
            };
        }
    }
}

fn update_frightened_state(
    mut commands: Commands,
    frightened_timer: Option<Res<FrightenedTimer>>,
    mut query: Query<Entity, (With<Ghost>, With<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    for entity in query.iter_mut() {
        let frightened_time_over = match frightened_timer {
            Some(ref timer) => timer.is_finished(),
            _ => true
        };

        if frightened_time_over {
            commands.entity(entity).remove::<Frightened>();
        }
    }
}

fn update_eaten_state<G: Component + GhostType + 'static>(
    mut commands: Commands,
    ghost_house: Res<GhostHouse>,
    query: Query<(Entity, &Transform), (With<G>, With<Eaten>, Without<Frightened>, Without<Spawned>)>,
) {
    for (entity, transform) in query.iter() {
        let coordinates = transform.translation;

        if coordinates == ghost_house.respawn_coordinates_of::<G>() {
            commands.entity(entity)
                .remove::<Eaten>()
                .insert(Spawned);
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
    mut query: Query<(Entity, &mut MoveDirection, &mut Target), (With<Ghost>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (entity, mut direction, mut target) in query.iter_mut() {
        commands.entity(entity).insert(Frightened);

        let coordinates_ghost_came_from = match *direction {
            Up => Vec3::new(target.x, target.y - 1.0, 0.0),
            Down => Vec3::new(target.x, target.y + 1.0, 0.0),
            Left => Vec3::new(target.x + 1.0, target.y, 0.0),
            Right => Vec3::new(target.x - 1.0, target.y, 0.0)
        };

        direction.reverse();
        *target = Target(coordinates_ghost_came_from);
    }
}

// TODO: Does this always work? (runs concurrent with target setters)
fn set_frightened_when_pacman_ate_energizer_and_ghost_has_no_target(
    mut commands: Commands,
    level: Res<Level>,
    event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(Entity, &mut MoveDirection), (With<Ghost>, Without<Target>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (entity, mut direction) in query.iter_mut() {
        commands.entity(entity).insert(Frightened);
        direction.reverse();
    }
}

fn set_eaten_when_hit_by_pacman(
    mut commands: Commands,
    ghost_query: Query<(Entity, &Position), (With<Ghost>, With<Frightened>, Without<Eaten>, Without<Spawned>)>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, ghost_position) in ghost_query.iter() {
        for pacman_position in pacman_query.iter() {
            if ghost_position == pacman_position {
                commands.entity(entity)
                    .remove::<Frightened>()
                    .insert(Eaten);
            }
        }
    }
}

fn reverse_when_schedule_changed(
    event_reader: EventReader<ScheduleChanged>,
    mut query: Query<(&mut MoveDirection, &mut Target), (With<Ghost>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    for (mut direction, mut target) in query.iter_mut() {
        let coordinates_ghost_came_from = match *direction {
            Up => Vec3::new(target.x, target.y - 1.0, 0.0),
            Down => Vec3::new(target.x, target.y + 1.0, 0.0),
            Left => Vec3::new(target.x + 1.0, target.y, 0.0),
            Right => Vec3::new(target.x - 1.0, target.y, 0.0)
        };

        direction.reverse();
        *target = Target(coordinates_ghost_came_from);
    }
}

// TODO: Does this always work? (runs concurrent with target setters)
fn reverse_when_schedule_changed_and_ghost_has_no_target(
    event_reader: EventReader<ScheduleChanged>,
    mut query: Query<&mut MoveDirection, (With<Ghost>, Without<Target>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    for mut direction in query.iter_mut() {
        direction.reverse();
    }
}