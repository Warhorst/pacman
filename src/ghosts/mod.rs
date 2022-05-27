use bevy::prelude::*;

use crate::ghosts::movement::MovePlugin;
use crate::ghosts::schedule::SchedulePlugin;
use crate::ghosts::spawn::spawn_ghosts;
use crate::ghosts::state::StatePlugin;
use crate::ghosts::target::{Target_, TargetPlugin};
use crate::tunnels::GhostPassedTunnel;

pub mod movement;
pub mod spawn;
pub mod state;
pub mod target;
mod schedule;

/// Used to mark every ghost.
#[derive(Component, Eq, PartialEq)]
pub struct Ghost;

/// Marks every ghost.
pub trait GhostType {}

#[derive(Copy, Clone, Component)]
pub struct Blinky;

#[derive(Copy, Clone, Component)]
pub struct Pinky;

#[derive(Copy, Clone, Component)]
pub struct Inky;

#[derive(Copy, Clone, Component)]
pub struct Clyde;

impl GhostType for Blinky {}

impl GhostType for Pinky {}

impl GhostType for Inky {}

impl GhostType for Clyde {}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MovePlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(StatePlugin)
            .add_plugin(SchedulePlugin)
            .add_startup_system(spawn_ghosts)
            .add_system(ghost_passed_tunnel);
    }
}

fn ghost_passed_tunnel(
    mut event_reader: EventReader<GhostPassedTunnel>,
    mut query: Query<(Entity, &mut Target_), With<Ghost>>,
) {
    for event in event_reader.iter() {
        for (entity, mut target) in query.iter_mut() {
            if entity == **event {
                target.clear();
            }
        }
    }
}