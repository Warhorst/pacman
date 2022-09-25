use bevy::prelude::*;
use crate::animation::Animations;
use crate::game_assets::loaded_assets::LoadedAssets;

use crate::life_cycle::LifeCycle::*;
use crate::pacman::edible_eaten::EdibleEatenPlugin;
use crate::pacman::spawn::spawn_pacman;
use crate::pacman::movement::{InputBuffer, move_pacman, set_direction_based_on_keyboard_input};
use crate::pacman::textures::{start_animation, update_pacman_appearance};

mod movement;
mod spawn;
mod textures;
mod edible_eaten;

/// Marker component for a pacman entity.
#[derive(Component)]
pub struct Pacman;

/// Fired when pacman died.
pub struct EPacmanDead;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EPacmanDead>()
            .add_plugin(EdibleEatenPlugin)
            .insert_resource(InputBuffer(None))
            .add_system_set(
                SystemSet::on_enter(Ready)
                    .with_system(spawn_pacman)
            )
            .add_system_set(
                SystemSet::on_enter(Running)
                    .with_system(start_animation)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(move_pacman)
                    .with_system(set_direction_based_on_keyboard_input)
                    .with_system(update_pacman_appearance.after(set_direction_based_on_keyboard_input))
            )
            .add_system_set(
                SystemSet::on_enter(PacmanHit).with_system(stop_animation)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanDying)
                    .with_system(play_the_dying_animation)
                    .with_system(play_the_dying_sound)
            )
            .add_system_set(
                SystemSet::on_update(PacmanDying).with_system(check_if_pacman_finished_dying)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanDead).with_system(despawn_pacman)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(stop_animation)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(despawn_pacman)
            )
            .add_system_set(
                SystemSet::on_enter(GhostEatenPause).with_system(set_invisible)
            )
            .add_system_set(
                SystemSet::on_exit(GhostEatenPause).with_system(set_visible)
            )
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
    audio: Res<Audio>,
    loaded_assets: Res<LoadedAssets>,
) {
    audio.play(loaded_assets.get_handle("sounds/dying.ogg"));
}

fn check_if_pacman_finished_dying(
    mut event_writer: EventWriter<EPacmanDead>,
    query: Query<&Animations, With<Pacman>>,
) {
    for animations in query.iter() {
        if animations.current().is_completely_finished() {
            event_writer.send(EPacmanDead)
        }
    }
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
        vis.is_visible = false
    }
}

fn set_visible(
    mut query: Query<&mut Visibility, With<Pacman>>
) {
    for mut vis in &mut query {
        vis.is_visible = true
    }
}