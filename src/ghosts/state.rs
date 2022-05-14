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
use crate::level::Level;

use self::State::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_frightened_state)
            .add_system(update_spawned_state)
            .add_system(set_chase_and_scatter_next_state)
            .add_system(update_eaten_state)
            .add_system(update_frightened_timer)
            .add_system(set_frightened_when_pacman_ate_energizer)
            .add_system(set_frightened_when_pacman_ate_energizer_and_ghost_has_no_target)
            .add_system(set_eaten_when_hit_by_pacman);
    }
}

/// The different states of a ghost
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
#[derive(Copy, Clone, Component, Debug, PartialOrd, PartialEq)]
pub enum State {
    Chase,
    Scatter,
}

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

fn update_spawned_state(
    mut commands: Commands,
    schedule: Res<Schedule>,
    board: Res<Board>,
    mut query: Query<(Entity, &mut State, &Position), (With<Ghost>, With<Spawned>, Without<Frightened>, Without<Eaten>)>,
) {
    for (entity, mut state, position) in query.iter_mut() {
        if board.type_of_position(position) == &GhostWall {
            commands.entity(entity).remove::<Spawned>();
            *state = schedule.current_state()
        }
    }
}

fn set_chase_and_scatter_next_state(
    schedule: Res<Schedule>,
    mut query: Query<&mut State, (With<Ghost>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    for mut state in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        if *state != schedule.current_state() {
            *state = schedule.current_state();
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
            Some(ref timer)  => timer.is_finished(),
            _ => true
        };

        if frightened_time_over {
            commands.entity(entity).remove::<Frightened>();
        }
    }
}

fn update_eaten_state(
    mut commands: Commands,
    board: Res<Board>,
    query: Query<(Entity, &Position), (With<Eaten>, Without<Frightened>, Without<Spawned>)>,
) {
    for (entity, position) in query.iter() {
        if board.type_of_position(position) == &GhostSpawn {
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
        },
        Some(ref mut t) => t.tick(time.delta()),
        _ => return
    }
}

fn set_frightened_when_pacman_ate_energizer(
    mut commands: Commands,
    level: Res<Level>,
    event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(Entity, &State, &mut MoveDirection, &mut Target), (With<Ghost>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (entity, state, mut direction, mut target) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

        commands.entity(entity).insert(Frightened);

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
    mut commands: Commands,
    level: Res<Level>,
    event_reader: EventReader<EnergizerEaten>,
    mut query: Query<(Entity, &State, &mut MoveDirection), (With<Ghost>, Without<Target>, Without<Frightened>, Without<Eaten>, Without<Spawned>)>,
) {
    if event_reader.is_empty() { return; }

    commands.insert_resource(FrightenedTimer::start(&level));

    for (entity, state, mut direction) in query.iter_mut() {
        if *state != Chase && *state != Scatter { continue; }

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