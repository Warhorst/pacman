use bevy::prelude::*;
use crate::animation::Animations;
use crate::board_dimensions::BoardDimensions;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::life_cycle::LifeCycle::*;
use crate::map::board::Board;
use crate::pacman::edible_eaten::EdibleEatenPlugin;
use crate::pacman::ghost_eaten::GhostEatenPlugin;
use crate::pacman::spawn::spawn_pacman;
use crate::pacman::movement::move_pacman;
use crate::pacman::textures::{start_animation, update_pacman_appearance};

mod movement;
mod spawn;
mod textures;
mod edible_eaten;
mod ghost_eaten;

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
            .add_plugin(GhostEatenPlugin)
            .insert_resource(InputBuffer(None))
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_pacman)
            )
            .add_system_set(
                SystemSet::on_enter(Running).with_system(start_animation)
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
                SystemSet::on_enter(PacmanDying).with_system(play_the_dying_animation)
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
        ;
    }
}

fn set_direction_based_on_keyboard_input(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    mut query: Query<(&Transform, &mut Direction), With<Pacman>>,
) {
    for (transform, mut direction) in query.iter_mut() {
        let position = dimensions.vec_to_pos(&transform.translation);
        let wished_direction = get_wished_direction(&keyboard_input, &input_buffer);

        if let Some(dir) = wished_direction {
            let position_in_direction = position.neighbour_position(&dir);

            if board.position_is_wall_or_entrance(&position_in_direction) {
                input_buffer.0 = Some(dir)
            } else {
                *direction = dir;
                input_buffer.0 = None;
            }
        }
    }
}

/// Return the direction pacman should move to next. If no matching keyboard key was pressed, return the last buffered input.
fn get_wished_direction(keyboard_input: &Input<KeyCode>, input_buffer: &InputBuffer) -> Option<Direction> {
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        return Some(Left)
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        return Some(Right)
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        return Some(Up)
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        return Some(Down)
    }

    **input_buffer
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

fn check_if_pacman_finished_dying(
    mut event_writer: EventWriter<EPacmanDead>,
    query: Query<&Animations, With<Pacman>>
) {
    for animations in query.iter() {
        if animations.current().is_completely_finished() {
            event_writer.send(EPacmanDead)
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

/// Saves the wished direction pacman should move to next.
#[derive(Deref, DerefMut)]
struct InputBuffer(Option<Direction>);