use bevy::prelude::*;
use crate::animation::Animations;

use crate::ghosts::movement::MovePlugin;
use crate::ghosts::schedule::SchedulePlugin;
use crate::ghosts::spawn::spawn_ghosts;
use crate::ghosts::state::StatePlugin;
use crate::ghosts::target::{Target, TargetPlugin};
use crate::ghosts::textures::update_ghost_appearance;
use crate::tunnels::GhostPassedTunnel;
use crate::life_cycle::LifeCycle::*;

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
            .add_plugin(MovePlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(StatePlugin)
            .add_plugin(SchedulePlugin)
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_ghosts)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(ghost_passed_tunnel)
                    .with_system(update_ghost_appearance::<Blinky>)
                    .with_system(update_ghost_appearance::<Pinky>)
                    .with_system(update_ghost_appearance::<Inky>)
                    .with_system(update_ghost_appearance::<Clyde>)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanDying).with_system(despawn_ghosts)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(despawn_ghosts)
            )
        ;
    }
}

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

fn despawn_ghosts(
    mut commands: Commands,
    query: Query<Entity, With<Ghost>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
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