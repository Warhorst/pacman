use bevy::prelude::*;
use crate::dots::DotEaten;

use crate::ghosts::movement::MovePlugin;
use crate::ghosts::schedule::SchedulePlugin;
use crate::ghosts::spawn::spawn_ghosts;
use crate::ghosts::state::StatePlugin;
use crate::ghosts::target::{Target, TargetPlugin};
use crate::ghosts::textures::GhostTextures;
use crate::tunnels::GhostPassedTunnel;
use crate::common::Direction;
use crate::ghosts::state::State;

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
            .add_startup_system(spawn_ghosts)
            .add_system(ghost_passed_tunnel)
            .add_system(update_dot_counter_when_dot_eaten)
            .add_system(update_ghost_appearance::<Blinky>)
            .add_system(update_ghost_appearance::<Pinky>)
            .add_system(update_ghost_appearance::<Inky>)
            .add_system(update_ghost_appearance::<Clyde>)
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

/// The personal counter of every ghost to keep track of how many dots were eaten.
/// When reaching zero, a ghost can leave the ghost house.
#[derive(Component)]
pub struct DotCounter {
    max: u8,
    current: u8,
}

impl DotCounter {
    pub fn new(amount: u8) -> Self {
        DotCounter {
            max: amount,
            current: amount,
        }
    }

    pub fn decrease(&mut self) {
        if !self.is_done() {
            self.current -= 1
        }
    }

    pub fn is_done(&self) -> bool {
        self.current == 0
    }

    pub fn is_active(&self) -> bool {
        !self.is_done()
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

fn update_dot_counter_when_dot_eaten(
    mut event_reader: EventReader<DotEaten>,
    mut query: Query<&mut DotCounter>,
) {
    for _ in event_reader.iter() {
        for mut dot_counter in query.iter_mut() {
            dot_counter.decrease();
        }
    }
}

fn update_ghost_appearance<G: 'static + Component + GhostType>(
    ghost_textures: Res<GhostTextures>,
    mut query: Query<(&Direction, &State, &mut Handle<Image>), With<G>>
) {
    for (direction, state, mut texture) in query.iter_mut() {
        match state {
            State::Frightened => *texture = ghost_textures.get_frightened_texture(),
            State::Eaten => *texture = ghost_textures.get_eaten_texture(&direction),
            _ => *texture = ghost_textures.get_normal_texture_for::<G>(&direction)
        }
    }
}