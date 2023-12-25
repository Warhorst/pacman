use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use bevy::sprite::collide_aabb::collide;
use pad::{Position, Direction};
use pad::Direction::*;

use crate::constants::{FIELD_DIMENSION, FIELD_SIZE, PACMAN_Z, WALL_DIMENSION};
use crate::game::direction::MovementDirection;
use crate::game::map::Wall;
use crate::game::pacman::edible_eaten::EdibleEatenStop;
use crate::game::pacman::Pacman;
use crate::game::speed::Speed;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct MoveComponents<'a> {
    direction: &'a MovementDirection,
    transform: &'a mut Transform,
    speed: &'a Speed,
}

pub(in crate::game) fn move_pacman_new(
    time: Res<Time>,
    wall_query: Query<&Transform, (With<Wall>, Without<Pacman>)>,
    mut pacman_query: Query<MoveComponents, (With<Pacman>, Without<EdibleEatenStop>)>,
) {
    for mut move_components in &mut pacman_query {
        let new_coordinates = calculate_new_coordinates(&move_components, time.delta_seconds());

        for transform in &wall_query {
            if collide(new_coordinates, Vec2::splat(FIELD_SIZE), transform.translation, Vec2::splat(WALL_DIMENSION)).is_some() {
                move_components.transform.translation = Position::from_vec3(new_coordinates, FIELD_DIMENSION).to_vec3(FIELD_DIMENSION, PACMAN_Z);
                return;
            }
        }

        move_components.transform.translation = new_coordinates;
    }
}

/// Calculate pacmans new coordinates on the window based on his speed and the time.
fn calculate_new_coordinates(move_components: &MoveComponentsItem, delta_seconds: f32) -> Vec3 {
    let (x, y) = get_modifiers_for_direction(move_components.direction);
    let mut new_coordinates = move_components.transform.translation;
    new_coordinates.x += delta_seconds * x * **move_components.speed;
    new_coordinates.y += delta_seconds * y * **move_components.speed;
    new_coordinates
}

fn get_modifiers_for_direction(direction: &Direction) -> (f32, f32) {
    match direction {
        YP => (0.0, 1.0),
        YM => (0.0, -1.0),
        XM => (-1.0, 0.0),
        XP => (1.0, 0.0),
        _ => (0.0, 0.0)
    }
}

pub(in crate::game) fn set_direction_based_on_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    mut pacman_query: Query<(&Transform, &mut MovementDirection), With<Pacman>>,
    wall_query: Query<&Transform, With<Wall>>
) {
    for (transform, mut direction) in &mut pacman_query {
        let position = Position::from_vec3(transform.translation, FIELD_DIMENSION);
        let wished_direction = get_wished_direction(&keyboard_input, &input_buffer);

        if let Some(dir) = wished_direction {
            let position_center = position.to_vec3(FIELD_DIMENSION, PACMAN_Z);
            let position_in_direction = position.neighbour_in_direction(dir);
            let position_in_direction_is_wall = wall_query.iter().any(|transform| Position::from_vec3(transform.translation, FIELD_DIMENSION) == position_in_direction);

            if position_in_direction_is_wall || !is_centered_enough(transform.translation, dir, position_center) {
                input_buffer.0 = Some(dir)
            } else {
                **direction = dir;
                input_buffer.0 = None;
            }
        }
    }
}

/// Return the direction pacman should move to next. If no matching keyboard key was pressed, return the last buffered input.
fn get_wished_direction(keyboard_input: &Input<KeyCode>, input_buffer: &InputBuffer) -> Option<Direction> {
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        return Some(XM);
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        return Some(XP);
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        return Some(YP);
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        return Some(YM);
    }

    **input_buffer
}

pub (in crate::game) fn reset_input_buffer(
    mut input_buffer: ResMut<InputBuffer>
) {
    input_buffer.0 = None;
}

/// Return if pacman is near enough to his currents position center to move to an orthogonal position.
fn is_centered_enough(coordinates: Vec3, direction: Direction, position_coordinates: Vec3) -> bool {
    let (x, y) = (coordinates.x, coordinates.y);
    let (posx, posy) = (position_coordinates.x, position_coordinates.y);
    let max_distance = FIELD_SIZE * 0.25;

    match direction {
        YP | YM => x >= posx - max_distance && x <= posx + max_distance,
        XP | XM => y >= posy - max_distance && y <= posy + max_distance,
        _ => panic!("invalid direction")
    }
}

/// Saves the wished direction pacman should move to next.
#[derive(Deref, DerefMut, Resource)]
pub struct InputBuffer(pub Option<Direction>);