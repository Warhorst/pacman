use bevy::prelude::*;

use crate::ghosts::movement::MovePlugin;
use crate::ghosts::schedule::SchedulePlugin;
use crate::ghosts::spawn::spawn_ghosts;
use crate::ghosts::state::{StatePlugin, StateSetter};
use crate::ghosts::target::{Target, TargetPlugin};
use crate::ghosts::textures::{Animation, update_animation, update_ghost_appearance};
use crate::tunnels::GhostPassedTunnel;
use crate::common::Direction;
use crate::ghost_house::GhostHouse;
use crate::ghosts::state::State;
use crate::ghosts::state::State::Spawned;
use crate::pacman::PacmanKilled;

pub mod movement;
pub mod spawn;
pub mod state;
pub mod target;
mod schedule;
mod textures;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Animation::new())
            .add_plugin(MovePlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(StatePlugin)
            .add_plugin(SchedulePlugin)
            .add_startup_system(spawn_ghosts)
            .add_system(ghost_passed_tunnel)
            .add_system(update_ghost_appearance::<Blinky>)
            .add_system(update_ghost_appearance::<Pinky>)
            .add_system(update_ghost_appearance::<Inky>)
            .add_system(update_ghost_appearance::<Clyde>)
            .add_system(update_animation)
            .add_system_set(
                SystemSet::new()
                    .with_system(reset_ghosts_when_pacman_was_killed::<Blinky>)
                    .with_system(reset_ghosts_when_pacman_was_killed::<Pinky>)
                    .with_system(reset_ghosts_when_pacman_was_killed::<Inky>)
                    .with_system(reset_ghosts_when_pacman_was_killed::<Clyde>)
                    .before(StateSetter)
            )

        ;
    }
}

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

fn ghost_passed_tunnel(
    mut event_reader: EventReader<GhostPassedTunnel>,
    mut query: Query<(Entity, &mut Target), With<Ghost>>,
) {
    for event in event_reader.iter() {
        for (entity, mut target) in query.iter_mut() {
            if entity == **event {
                target.clear();
            }
        }
    }
}

fn reset_ghosts_when_pacman_was_killed<G: GhostType + Component + 'static>(
    mut event_reader: EventReader<PacmanKilled>,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(&mut Direction, &mut State, &mut Target, &mut Transform), With<G>>
) {
    for _ in event_reader.iter() {
        for (mut direction, mut state, mut target, mut transform) in query.iter_mut() {
            *direction = ghost_house.spawn_direction_of::<G>();
            target.clear();
            *state = Spawned;
            let spawn_coordinates = ghost_house.spawn_coordinates_of::<G>();
            transform.translation = spawn_coordinates;
        }
    }
}