use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::board_dimensions::BoardDimensions;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::position::Position;
use crate::constants::PACMAN_Z;
use crate::map::board::Board;
use crate::pacman::edible_eaten::EdibleEatenStop;
use crate::pacman::ghost_eaten::GhostEatenStop;
use crate::pacman::Pacman;
use crate::speed::Speed;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(in crate::pacman) struct MoveComponents<'a> {
    direction: &'a Direction,
    transform: &'a mut Transform,
    speed: &'a Speed,
}

pub(in crate::pacman) fn move_pacman(
    board: Res<Board>,
    time: Res<Time>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<MoveComponents, (With<Pacman>, Without<EdibleEatenStop>, Without<GhostEatenStop>)>,
) {
    let delta_seconds = time.delta_seconds();

    for mut move_components in query.iter_mut() {
        let mut new_coordinates = calculate_new_coordinates(&mut move_components, delta_seconds);
        let new_position = dimensions.vec_to_pos(&new_coordinates);

        if is_going_to_collide_with_obstacle(&board, &move_components.direction, move_components.transform.translation, &dimensions) {
            process_collision(&move_components.direction, &new_position, &mut new_coordinates, &dimensions)
        } else {
            center_position(&move_components.direction, &new_position, &mut new_coordinates, &dimensions)
        }

        move_components.transform.translation = new_coordinates;
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

fn get_modifiers_for_direction(direction: &Direction) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// Determine if pacman will collide with an obstacle if he is going further in his current direction.
fn is_going_to_collide_with_obstacle(board: &Board, direction: &Direction, coordinates: Vec3, dimensions: &BoardDimensions) -> bool {
    let pos_in_direction = dimensions.vec_to_pos(&coordinates).neighbour_position(&direction);
    position_is_obstacle(board, &pos_in_direction)
}

/// Tells if the given position is an obstacle for pacman.
///
/// Pacman cannot walk in walls or the ghost house entrance
fn position_is_obstacle(board: &Board, position: &Position) -> bool {
    board.position_is_wall(position) || board.position_is_ghost_house_entrance(position)
}

/// Limit pacmans movement if he reached an obstacle and stop him.
fn process_collision(direction: &Direction, new_position: &Position, new_coordinates: &mut Vec3, dimensions: &BoardDimensions) {
    let field_coordinates = dimensions.pos_to_vec(&new_position, PACMAN_Z);
    limit_movement(direction, &field_coordinates, new_coordinates);
}

/// Because the next field is an obstacle, pacman can not go beyond his current field.
fn limit_movement(direction: &Direction, field_coordinates: &Vec3, new_coordinates: &mut Vec3) {
    match direction {
        Up => new_coordinates.y = new_coordinates.y.min(field_coordinates.y),
        Down => new_coordinates.y = new_coordinates.y.max(field_coordinates.y),
        Left => new_coordinates.x = new_coordinates.x.max(field_coordinates.x),
        Right => new_coordinates.x = new_coordinates.x.min(field_coordinates.x)
    }
}

/// Center pacmans current position in the middle of his current field.
/// The purpose of this method is to keep equally sized gaps to the hallway pacman is currently passing.
fn center_position(direction: &Direction, new_position: &Position, new_coordinates: &mut Vec3, dimensions: &BoardDimensions) {
    let position_coordinates = dimensions.pos_to_vec(new_position, PACMAN_Z);
    match direction {
        Up | Down => new_coordinates.x = position_coordinates.x,
        Left | Right => new_coordinates.y = position_coordinates.y
    }
}