use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;

use crate::edibles::energizer::EnergizerTimer;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};
use crate::ghosts::state::State;
use crate::level::Level;
use crate::pacman::Pacman;
use crate::edibles::dots::EatenDots;
use crate::life_cycle::LifeCycle;
use crate::map::board::Board;
use crate::specs_per_level::SpecsPerLevel;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(LifeCycle::Running)
                    .with_system(update_pacman_speed)
                    .with_system(update_blinky_speed)
                    .with_system(update_ghost_speed::<Pinky>)
                    .with_system(update_ghost_speed::<Inky>)
                    .with_system(update_ghost_speed::<Clyde>)
            )
        ;
    }
}

/// The current speed of a moving entity
#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct Speed(pub f32);

fn update_ghost_speed<G: GhostType + Component>(
    board: Res<Board>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<(&Transform, &mut Speed, &State), With<G>>
) {
    let ghost_speed = dimensions.ghost_base_speed();

    for (transform, mut speed, state) in query.iter_mut() {
        let spec = specs_per_level.get_for(&level);

        if *state == State::Eaten {
            *speed = Speed(ghost_speed * 2.0)
        } else if board.position_is_tunnel(&dimensions.trans_to_pos(transform)) {
            *speed = Speed(ghost_speed * spec.ghost_tunnel_speed_modifier);
        } else if *state == State::Frightened {
            *speed = Speed(ghost_speed * spec.ghost_frightened_speed_modifier)
        } else {
            *speed = Speed(ghost_speed * spec.ghost_normal_speed_modifier)
        }
    }
}

/// Blinkys speed is set differently, as he has the elroy mode. He
/// gets two speed bonuses, depending on the remaining dots on the board.
/// The amount of dots to trigger elroy depends on the current level.
fn update_blinky_speed(
    board: Res<Board>,
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<(&Transform, &mut Speed, &State), With<Blinky>>
) {
    let ghost_speed = dimensions.ghost_base_speed();

    for (transform, mut speed, state) in query.iter_mut() {
        let spec = specs_per_level.get_for(&level);
        let remaining_dots = eaten_dots.get_remaining();

        if *state == State::Eaten {
            *speed = Speed(ghost_speed * 2.0)
        } else if board.position_is_tunnel(&dimensions.trans_to_pos(transform)) {
            *speed = Speed(ghost_speed * spec.ghost_tunnel_speed_modifier);
        } else if *state == State::Frightened {
            *speed = Speed(ghost_speed * spec.ghost_frightened_speed_modifier)
        } else if remaining_dots <= spec.elroy_2_dots_left {
            *speed = Speed(ghost_speed * spec.elroy_2_speed_modifier)
        } else if remaining_dots <= spec.elroy_1_dots_left {
            *speed = Speed(ghost_speed * spec.elroy_1_speed_modifier)
        } else {
            *speed = Speed(ghost_speed * spec.ghost_normal_speed_modifier)
        }
    }
}

fn update_pacman_speed(
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    energizer_timer: Option<Res<EnergizerTimer>>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<&mut Speed, With<Pacman>>,
) {
    let pacman_speed = dimensions.pacman_base_speed();

    for mut speed in query.iter_mut() {
        let spec = specs_per_level.get_for(&level);

        if energizer_timer.is_some() {
            *speed = Speed(pacman_speed * spec.pacman_frightened_speed_modifier);
        } else {
            *speed = Speed(pacman_speed * spec.pacman_normal_speed_modifier);
        }
    }
}