use bevy::prelude::*;
use bevy::utils::Duration;

use components::Schedule;

use crate::common::Movement;
use crate::common::Position;
use crate::events::{EnergizerEaten, GhostPassedTunnel};
use crate::ghosts::components::{Ghost, Target};
use crate::ghosts::mover::Mover;
use crate::ghosts::spawner::Spawner;
use crate::ghosts::state_setter::StateSetter;
use crate::ghosts::target_set::TargetSetPlugin;
use crate::map::board::Board;

use self::components::State;
use self::components::State::*;

pub mod components;
mod mover;
mod spawner;
mod state_setter;
mod target_set;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(FrightenedTimer::new())
            .add_plugin(TargetSetPlugin)
            .add_startup_system(spawn_ghosts.system())
            .add_system(update_state.system())
            .add_system(move_ghosts.system())
            .add_system(ghost_passed_tunnel.system())
            .add_system(make_ghosts_vulnerable.system());
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

fn spawn_ghosts(commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    Spawner::new(commands, &board, &mut materials).spawn()
}

fn move_ghosts(time: Res<Time>,
               board: Res<Board>,
               mut query: Query<(&Movement, &mut Position, &mut Target, &mut Transform), With<Ghost>>) {
    for (movement, mut position, mut target, mut transform) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds(),
                   movement,
                   &mut position,
                   &mut target,
                   &mut transform.translation)
            .move_ghost();
    }
}

fn update_state(time: Res<Time>,
                board: Res<Board>,
                mut frightened_timer: ResMut<FrightenedTimer>,
                mut query: Query<(&Position, &mut State, &mut Schedule), With<Ghost>>) {
    if !frightened_timer.is_finished() {
        frightened_timer.tick(time.delta());
    }

    for (position, mut state, mut schedule) in query.iter_mut() {
        StateSetter::new(&mut state, position, &mut schedule, &board, &mut frightened_timer, time.delta()).set_next_state();
    }
}

fn ghost_passed_tunnel(mut event_reader: EventReader<GhostPassedTunnel>,
                       mut query: Query<(Entity, &mut Target), With<Ghost>>) {
    for event in event_reader.iter() {
        for (entity, mut target) in query.iter_mut() {
            if entity == event.entity {
                target.clear()
            }
        }
    }
}

fn make_ghosts_vulnerable(mut event_reader: EventReader<EnergizerEaten>,
                          mut frightened_timer: ResMut<FrightenedTimer>,
                          mut query: Query<(&mut Target, &mut Movement, &mut State), With<Ghost>>) {
    let mut event_received = false;

    for _ in event_reader.iter() {
        for (mut target, mut movement, mut state) in query.iter_mut() {
            event_received = true;
            target.clear();
            movement.reverse();
            *state = Frightened;
        }
    }

    if event_received {
        frightened_timer.start()
    }
}