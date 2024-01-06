use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::pacman::edible_eaten::EdibleEatenStop;

use crate::core::prelude::*;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct MoveComponents<'a> {
    direction: &'a Dir,
    transform: &'a mut Transform,
    speed: &'a Speed,
}

pub(in crate::game) fn move_pacman(
    time: Res<Time>,
    wall_query: Query<&Transform, (With<Wall>, Without<Pacman>)>,
    mut pacman_query: Query<MoveComponents, (With<Pacman>, Without<EdibleEatenStop>)>,
) {
    for mut move_components in &mut pacman_query {
        let new_coordinates = calculate_new_coordinates(&move_components, time.delta_seconds());

        for transform in &wall_query {
            if collide(new_coordinates, Vec2::splat(FIELD_SIZE), transform.translation, Vec2::splat(WALL_DIMENSION)).is_some() {
                move_components.transform.translation = Pos::from_vec3(new_coordinates).to_vec3(PACMAN_Z);
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

fn get_modifiers_for_direction(direction: &Dir) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0),
    }
}

pub(in crate::game) fn set_direction_based_on_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    mut pacman_query: Query<(&Transform, &mut Dir), With<Pacman>>,
    wall_query: Query<&Transform, With<Wall>>
) {
    for (transform, mut direction) in &mut pacman_query {
        let position = Pos::from_vec3(transform.translation);
        let wished_direction = get_wished_direction(&keyboard_input, &input_buffer);

        if let Some(dir) = wished_direction {
            let position_center = position.to_vec3(PACMAN_Z);
            let position_in_direction = position.neighbour_in_direction(dir);
            let position_in_direction_is_wall = wall_query.iter().any(|transform| Pos::from_vec3(transform.translation) == position_in_direction);

            if position_in_direction_is_wall || !is_centered_enough(transform.translation, dir, position_center) {
                input_buffer.0 = Some(dir)
            } else {
                *direction = dir;
                input_buffer.0 = None;
            }
        }
    }
}

/// Return the direction pacman should move to next. If no matching keyboard key was pressed, return the last buffered input.
fn get_wished_direction(keyboard_input: &Input<KeyCode>, input_buffer: &InputBuffer) -> Option<Dir> {
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        return Some(Left);
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        return Some(Right);
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        return Some(Up);
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        return Some(Down);
    }

    **input_buffer
}

pub (in crate::game) fn reset_input_buffer(
    mut input_buffer: ResMut<InputBuffer>
) {
    input_buffer.0 = None;
}

/// Return if pacman is near enough to his currents position center to move to an orthogonal position.
fn is_centered_enough(coordinates: Vec3, direction: Dir, position_coordinates: Vec3) -> bool {
    let (x, y) = (coordinates.x, coordinates.y);
    let (posx, posy) = (position_coordinates.x, position_coordinates.y);
    let max_distance = FIELD_SIZE * 0.25;

    match direction {
        Up | Down => x >= posx - max_distance && x <= posx + max_distance,
        Left | Right => y >= posy - max_distance && y <= posy + max_distance,
    }
}

/// Saves the wished direction pacman should move to next.
#[derive(Deref, DerefMut, Resource)]
pub struct InputBuffer(pub Option<Dir>);