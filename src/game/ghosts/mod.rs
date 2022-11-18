use bevy::prelude::*;
use crate::game_assets::loaded_assets::LoadedAssets;

use crate::game::ghosts::movement::MovePlugin;
use crate::game::ghosts::spawn::spawn_ghosts;
use crate::game::target::Target;
use crate::game::ghosts::textures::{start_animation, update_ghost_appearance};
use crate::game::interactions::{EGhostEaten, LPacmanGhostHitDetection};
use crate::game_state::GameState::*;
use crate::game::map::tunnel::GhostPassedTunnel;

pub mod movement;
pub mod spawn;
mod textures;

pub (in crate::game) struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MovePlugin)
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_ghosts)
            )
            .add_system_set(
                SystemSet::on_enter(Running).with_system(start_animation)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(ghost_passed_tunnel)
                    // .with_system(update_ghost_appearance)
                    .with_system(play_ghost_eaten_sound_when_ghost_was_eaten.after(LPacmanGhostHitDetection))
            )
            .add_system_set(
                SystemSet::on_inactive_update(InGame).with_system(update_ghost_appearance)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanDying).with_system(despawn_ghosts)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(despawn_ghosts)
            )
            .add_system_set(
                SystemSet::on_enter(GhostEatenPause).with_system(set_currently_eaten_ghost_invisible)
            )
            .add_system_set(
                SystemSet::on_exit(GhostEatenPause)
                    .with_system(remove_currently_eaten_ghost)
                    .with_system(set_currently_eaten_ghost_visible)
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

fn remove_currently_eaten_ghost(
    mut commands: Commands
) {
    commands.remove_resource::<CurrentlyEatenGhost>()
}

fn set_currently_eaten_ghost_invisible(
    currently_eaten_ghost: Res<CurrentlyEatenGhost>,
    mut query: Query<(Entity, &mut Visibility), With<Ghost>>
) {
    for (entity, mut vis) in &mut query {
        if **currently_eaten_ghost == entity {
            vis.is_visible = false
        }
    }
}

fn set_currently_eaten_ghost_visible(
    currently_eaten_ghost: Res<CurrentlyEatenGhost>,
    mut query: Query<(Entity, &mut Visibility), With<Ghost>>
) {
    for (entity, mut vis) in &mut query {
        if **currently_eaten_ghost == entity {
            vis.is_visible = true
        }
    }
}

fn play_ghost_eaten_sound_when_ghost_was_eaten(
    loaded_assets: Res<LoadedAssets>,
    audio: Res<Audio>,
    mut event_reader: EventReader<EGhostEaten>
) {
    if event_reader.iter().count() > 0 {
        audio.play(loaded_assets.get_handle("sounds/ghost_eaten.ogg"));
    }
}

#[derive(Copy, Clone, Component, Eq, PartialEq, Hash)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde
}

/// Resource that holds the entity id of the ghost that is currently eaten by pacman
#[derive(Deref, Resource)]
pub struct CurrentlyEatenGhost(pub Entity);