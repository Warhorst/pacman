use bevy::prelude::*;

use crate::ghosts::components::Ghost;
use crate::ghosts::movement::MovePlugin;
use crate::ghosts::spawner::Spawner;
use crate::ghosts::state::StateSetPlugin;
use crate::ghosts::target::{Target, TargetSetPlugin};
use crate::map::board::Board;
use crate::tunnels::GhostPassedTunnel;

pub mod components;
pub mod movement;
pub mod spawner;
pub mod state;
pub mod target;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(MovePlugin)
            .add_plugin(TargetSetPlugin)
            .add_plugin(StateSetPlugin)
            .add_startup_system(spawn_ghosts.system())
            .add_system(ghost_passed_tunnel.system());
    }
}

fn spawn_ghosts(commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    Spawner::new(commands, &board, &mut materials).spawn()
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