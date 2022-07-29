use bevy::prelude::*;
use crate::animation::Animations;

use crate::common::Direction;
use crate::common::position::ToPosition;
use crate::common::Direction::*;
use crate::game_state::GameState;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::pacman::spawn::spawn_pacman;
use crate::pacman::movement::PacmanMovementPlugin;
use crate::pacman::textures::update_pacman_appearance;

mod movement;
mod spawn;
mod textures;

/// Marker component for a pacman entity.
#[derive(Component)]
pub struct Pacman;

/// Fired when pacman was hit by a ghost.
pub struct PacmanHit;

/// Fired when pacman died.
pub struct PacmanDead;

/// Fired when Pacman ate a ghost in frightened state.
#[derive(Deref)]
pub struct PacmanEatsGhost(Entity);

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PacmanMovementPlugin)
            .add_event::<PacmanHit>()
            .add_event::<PacmanDead>()
            .add_event::<PacmanEatsGhost>()
            .add_system_set(
                SystemSet::on_enter(GameState::Running).with_system(spawn_pacman)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(set_direction_based_on_keyboard_input)
                    .with_system(update_pacman_appearance.after(set_direction_based_on_keyboard_input))
                    .with_system(pacman_hits_ghost)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PacmanHit).with_system(stop_animation_when_hit)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PacmanDying).with_system(play_the_dying_animation)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PacmanDying).with_system(check_if_pacman_finished_dying)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PacmanDead).with_system(despawn_pacman)
            )
        ;
    }
}

fn set_direction_based_on_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Pacman>>,
) {
    for mut direction in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            *direction = Left
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            *direction = Right
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            *direction = Up
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            *direction = Down
        }
    }
}

fn pacman_hits_ghost(
    mut killed_event_writer: EventWriter<PacmanHit>,
    mut eat_event_writer: EventWriter<PacmanEatsGhost>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in pacman_query.iter() {
        for (entity, ghost_transform, state) in ghost_query.iter() {
            if pacman_transform.pos() == ghost_transform.pos() {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(PacmanHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(PacmanEatsGhost(entity))
                }
            }
        }
    }
}

fn stop_animation_when_hit(
    mut query: Query<&mut Animations, With<Pacman>>
) {
    for mut animations in query.iter_mut() {
        animations.current_mut().stop()
    }
}

fn play_the_dying_animation(
    mut query: Query<&mut Animations, With<Pacman>>
) {
    for mut animations in query.iter_mut() {
        animations.change_animation_to("dying")
    }
}

fn check_if_pacman_finished_dying(
    mut event_writer: EventWriter<PacmanDead>,
    query: Query<&Animations, With<Pacman>>
) {
    for animations in query.iter() {
        if animations.current().is_completely_finished() {
            event_writer.send(PacmanDead)
        }
    }
}

fn despawn_pacman(
    mut commands: Commands,
    query: Query<Entity, With<Pacman>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}