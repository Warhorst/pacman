use bevy::prelude::*;
use crate::game_assets::animation::Animations;
use crate::game_assets::loaded_assets::LoadedAssets;

use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::pacman::edible_eaten::EdibleEatenPlugin;
use crate::game::pacman::spawn::spawn_pacman;
use crate::game::pacman::movement::{InputBuffer, move_pacman_new, reset_input_buffer, set_direction_based_on_keyboard_input};
use crate::game::pacman::textures::{start_animation, update_pacman_appearance};

mod movement;
mod spawn;
mod textures;
mod edible_eaten;

/// Marker component for a pacman entity.
#[derive(Component)]
pub struct Pacman;

pub(in crate::game) struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EdibleEatenPlugin)
            .insert_resource(InputBuffer(None))
            .add_systems(OnEnter(Game(Ready)), spawn_pacman)
            .add_systems(OnEnter(Game(Running)), start_animation)
            .add_systems(Update, (
                move_pacman_new,
                set_direction_based_on_keyboard_input,
                update_pacman_appearance.after(set_direction_based_on_keyboard_input)
            ).run_if(in_state(Game(Running))))
            .add_systems(OnEnter(Game(PacmanHit)), (
                stop_animation,
                reset_input_buffer
            ))
            .add_systems(OnEnter(Game(PacmanDying)), (
                play_the_dying_animation,
                play_the_dying_sound
            ))
            .add_systems(OnEnter(Game(PacmanDead)), despawn_pacman)
            .add_systems(OnEnter(Game(LevelTransition)), (
                stop_animation,
                reset_input_buffer
            ))
            .add_systems(OnExit(Game(LevelTransition)), despawn_pacman)
            .add_systems(OnEnter(Game(GhostEatenPause)), set_invisible)
            .add_systems(OnExit(Game(GhostEatenPause)), set_visible)
        ;
    }
}

fn stop_animation(
    mut query: Query<&mut Animations, With<Pacman>>
) {
    for mut animations in query.iter_mut() {
        animations.stop()
    }
}

fn play_the_dying_animation(
    mut query: Query<&mut Animations, With<Pacman>>
) {
    for mut animations in query.iter_mut() {
        animations.resume();
        animations.change_animation_to("dying")
    }
}

fn play_the_dying_sound(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    commands.spawn(
        AudioBundle {
            source: loaded_assets.get_handle("sounds/dying.ogg"),
            ..default()
        }
    );
}

fn despawn_pacman(
    mut commands: Commands,
    query: Query<Entity, With<Pacman>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

fn set_invisible(
    mut query: Query<&mut Visibility, With<Pacman>>
) {
    for mut vis in &mut query {
        *vis = Visibility::Hidden
    }
}

fn set_visible(
    mut query: Query<&mut Visibility, With<Pacman>>
) {
    for mut vis in &mut query {
        *vis = Visibility::Visible
    }
}