use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;

use crate::common::MoveDirection;
use crate::common::MoveDirection::*;
use crate::common::Position;
use crate::constants::{PACMAN_DIMENSION, WALL_DIMENSION};
use crate::is;
use crate::map::board::Board;
use crate::map::Element;
use crate::pacman::{Pacman, Stop};
use crate::speed::Speed;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MoveComponents<'a> {
    direction: &'a MoveDirection,
    position: &'a mut Position,
    transform: &'a mut Transform,
    speed: &'a Speed
}

pub(in crate::pacman) fn move_pacman_if_not_stopped(
    board: Res<Board>,
    time: Res<Time>,
    mut query: Query<MoveComponents, (With<Pacman>, Without<Stop>)>
) {
    let delta_seconds = time.delta_seconds();

    for mut move_components in query.iter_mut() {
        let mut new_coordinates = calculate_new_coordinates(&mut move_components, delta_seconds);
        let new_position = Position::from(&new_coordinates);

        if is_going_to_collide_with_obstacle(&board, &move_components.direction, &new_position, &new_coordinates) {
            process_collision(&move_components.direction, &new_position, &mut new_coordinates)
        } else {
            center_position(&move_components.direction, &new_position, &mut new_coordinates)
        }

        move_components.transform.translation = new_coordinates;
        *move_components.position = new_position;
    }
}

/// Calculate pacmans new coordinates on the window based on his speed and the time.
fn calculate_new_coordinates(move_components: &mut MoveComponentsItem, delta_seconds: f32) -> Vec3 {
    let (x, y) = get_modifiers_for_direction(move_components.direction);
    let mut new_coordinates = move_components.transform.translation;
    new_coordinates.x += delta_seconds * x * **move_components.speed;
    new_coordinates.y += delta_seconds * y * **move_components.speed;
    new_coordinates
}

fn get_modifiers_for_direction(direction: &MoveDirection) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// Determine if pacman will collide with an obstacle if he is going further in his current direction.
fn is_going_to_collide_with_obstacle(board: &Board, direction: &MoveDirection, new_position: &Position, new_coordinates: &Vec3) -> bool {
    match board.position_in_direction(new_position, direction) {
        Some(pos) if position_is_obstacle(board, &pos) => true,
        Some(pos) => !are_coordinates_in_field_center(direction, &pos, new_coordinates, PACMAN_DIMENSION),
        None => true
    }
}

/// Determines if pacmans current coordinates are in the center of his current position. The center of the position is
/// its middle point with the width/height of the accumulated distance between pacman and the walls.
/// Assumes pacman is larger than a wall.
pub fn are_coordinates_in_field_center(direction: &MoveDirection, position: &Position, coordinates: &Vec3, entity_dimension: f32) -> bool {
    let position_coordinates = Vec3::from(position);
    let entity_wall_distance = match entity_dimension > WALL_DIMENSION {
        true => entity_dimension - WALL_DIMENSION,
        false => 0.0
    };
    match direction {
        Left | Right => {
            let y_start = position_coordinates.y - entity_wall_distance;
            let y_end = position_coordinates.y + entity_wall_distance;
            coordinates.y >= y_start && coordinates.y <= y_end
        }
        Up | Down => {
            let x_start = position_coordinates.x - entity_wall_distance;
            let x_end = position_coordinates.x + entity_wall_distance;
            coordinates.x >= x_start && coordinates.x <= x_end
        }
    }
}

/// Tells if the given position is an obstacle for pacman.
fn position_is_obstacle(board: &Board, position: &Position) -> bool {
    board.position_matches_filter(position, is!(Element::Wall {..} | Element::GhostHouseEntrance {..} | Element::InvisibleWall))
}

/// Limit pacmans movement if he reached an obstacle and stop him.
fn process_collision(direction: &MoveDirection, new_position: &Position, new_coordinates: &mut Vec3) {
    let field_coordinates = Vec3::from(new_position);
    limit_movement(direction, &field_coordinates, new_coordinates);
}

/// Because the next field is an obstacle, pacman can not go beyond his current field.
fn limit_movement(direction: &MoveDirection, field_coordinates: &Vec3, new_coordinates: &mut Vec3) {
    match direction {
        Up => new_coordinates.y = new_coordinates.y.min(field_coordinates.y),
        Down => new_coordinates.y = new_coordinates.y.max(field_coordinates.y),
        Left => new_coordinates.x = new_coordinates.x.max(field_coordinates.x),
        Right => new_coordinates.x = new_coordinates.x.min(field_coordinates.x)
    }
}

/// Center pacmans current position in the middle of his current field.
/// The purpose of this method is to keep equally sized gaps to the hallway pacman is currently passing.
fn center_position(direction: &MoveDirection, new_position: &Position, new_coordinates: &mut Vec3) {
    let position_coordinates = Vec3::from(new_position);
    match direction {
        Up | Down => new_coordinates.x = position_coordinates.x,
        Left | Right => new_coordinates.y = position_coordinates.y
    }
}