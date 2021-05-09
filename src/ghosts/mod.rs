use bevy::prelude::*;

use crate::common::Movement;
use crate::common::Position;
use crate::events::{EnergizerEaten, GhostPassedTunnel};
use crate::ghosts::components::{Ghost, Target};
use crate::ghosts::mover::Mover;
use crate::ghosts::spawner::Spawner;
use crate::ghosts::state_set::StateSetPlugin;
use crate::ghosts::target_set::TargetSetPlugin;
use crate::map::board::Board;

pub mod components;
mod mover;
mod spawner;
mod state_set;
mod target_set;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(TargetSetPlugin)
            .add_plugin(StateSetPlugin)
            .add_startup_system(spawn_ghosts.system())
            .add_system(move_ghosts.system())
            .add_system(ghost_passed_tunnel.system())
            .add_system(reverse_movement_when_pacman_ate_energizer.system());
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

fn reverse_movement_when_pacman_ate_energizer(
    mut event_reader: EventReader<EnergizerEaten>,
    mut query: Query<&mut Movement, With<Ghost>>,
) {
    for _ in event_reader.iter() {
        for mut movement in query.iter_mut() {
            movement.reverse();
        }
    }
}