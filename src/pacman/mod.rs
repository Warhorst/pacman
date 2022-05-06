use std::ops::DerefMut;
use std::time::Duration;

use bevy::prelude::*;

use crate::common::{MoveComponents, MoveDirection, Position};
use crate::common::MoveDirection::*;
use crate::dots::DotEaten;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::ghosts::state::State::*;
use crate::map::board::Board;
use crate::pacman::mover::Mover;
use crate::pacman::spawner::Spawner;

mod mover;
mod spawner;

/// Marker component for a pacman entity.
#[derive(Component)]
pub struct Pacman;

/// Marker component to signal that pacman is currently stopped.
#[derive(Component)]
pub struct Stop;

/// Timer that tells how long pacman will be unable to move.
pub struct PacmanStopTimer {
    timer: Option<Timer>,
}

impl PacmanStopTimer {
    pub fn new() -> Self {
        PacmanStopTimer {
            timer: None
        }
    }

    pub fn start_for_dot(&mut self) {
        self.timer = Some(Timer::from_seconds(1.0 / 60.0, false))
    }

    pub fn tick(&mut self, delta: Duration) {
        if let Some(ref mut timer) = self.timer {
            timer.tick(delta);
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.timer {
            None => true,
            Some(ref t) => t.finished()
        }
    }
}

/// Fired when pacman was killed by a ghost.
pub struct PacmanKilled;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PacmanKilled>()
            .insert_resource(PacmanStopTimer::new())
            .add_startup_system(spawn_pacman)
            .add_system(move_pacman_if_not_stopped)
            .add_system(set_direction_based_on_keyboard_input)
            .add_system(pacman_hits_ghost_and_might_get_killed)
            .add_system(stop_pacman_when_a_dot_was_eaten)
            .add_system(update_pacman_stop_timer)
        ;
    }
}

fn spawn_pacman(commands: Commands, board: Res<Board>) {
    Spawner::new(commands, &board).spawn()
}

fn move_pacman_if_not_stopped(
    time: Res<Time>,
    board: Res<Board>,
    mut query: Query<MoveComponents, (With<Pacman>, Without<Stop>)>,
) {
    for (mut transform, mut position, mut movement) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds(),
                   movement.deref_mut(),
                   position.deref_mut(),
                   &mut transform.translation)
            .move_pacman()
    }
}

fn set_direction_based_on_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MoveDirection, With<Pacman>>,
) {
    for mut direction in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *direction = Left
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *direction = Right
        }

        if keyboard_input.pressed(KeyCode::Up) {
            *direction = Up
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *direction = Down
        }
    }
}

fn pacman_hits_ghost_and_might_get_killed(
    mut commands: Commands,
    mut event_writer: EventWriter<PacmanKilled>,
    pacman_query: Query<(Entity, &Position), With<Pacman>>,
    ghost_query: Query<(&Position, &State), With<Ghost>>,
) {
    for (pacman_entity, pacman_position) in pacman_query.iter() {
        for (ghost_position, state) in ghost_query.iter() {
            if pacman_position == ghost_position && !vec![&Frightened, &Eaten].contains(&state) {
                commands.entity(pacman_entity).despawn();
                event_writer.send(PacmanKilled)
            }
        }
    }
}

/// When pacman eats a dot, he will stop for a moment. This allows
/// the ghost to catch up on him if he continues to eat dots.
///
/// In the original arcade game, his movement update just skipped
/// for one frame, letting him stop for 1/60 second. This might work on
/// a frame locked arcade machine but will not have the desired
/// effect if the game runs on 144 FPS for example. Therefore, a timer
/// with a fixed duration is used instead.
fn stop_pacman_when_a_dot_was_eaten(
    mut commands: Commands,
    event_reader: EventReader<DotEaten>,
    mut pacman_stop_timer: ResMut<PacmanStopTimer>,
    query: Query<Entity, With<Pacman>>,
) {
    if event_reader.is_empty() { return; }

    for entity in query.iter() {
        pacman_stop_timer.start_for_dot();
        commands.entity(entity).insert(Stop);
    }
}

fn update_pacman_stop_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut pacman_stop_timer: ResMut<PacmanStopTimer>,
    query: Query<Entity, (With<Pacman>, With<Stop>)>,
) {
    pacman_stop_timer.tick(time.delta());

    if !pacman_stop_timer.is_finished() { return; }

    for entity in query.iter() {
        commands.entity(entity).remove::<Stop>();
    }
}