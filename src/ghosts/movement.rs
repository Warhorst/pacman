use bevy::prelude::*;

use crate::common;
use crate::common::{Movement, Position};
use crate::common::MoveDirection::*;
use crate::common::Movement::*;
use crate::constants::GHOST_SPEED;
use crate::ghosts::target::Target;
use crate::map::board::Board;

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_ghost);
    }
}

fn move_ghost(
    mut commands: Commands,
    time: Res<Time>,
    board: Res<Board>,
    mut query: Query<(Entity, &Movement, &mut Position, &Target, &mut Transform)>,
) {
    for (entity, movement, mut position, target, mut transform) in query.iter_mut() {
        let mut coordinates = &mut transform.translation;
        let delta_seconds = time.delta_seconds();
        let direction = match *movement {
            Idle => return,
            Moving(dir) => dir
        };

        let target_coordinates = board.coordinates_of_position(target);
        move_in_direction(&mut coordinates, delta_seconds, &direction);
        limit_movement(&mut coordinates, &direction, &target_coordinates);
        if *coordinates == target_coordinates {
            commands.entity(entity).remove::<Target>();
        }
        *position = board.position_of_coordinates(coordinates)
    }
}

fn move_in_direction(coordinates: &mut Vec3, delta_seconds: f32, direction: &common::MoveDirection) {
    let (x, y) = get_direction_modifiers(direction);
    coordinates.x += delta_seconds * x * GHOST_SPEED;
    coordinates.y += delta_seconds * y * GHOST_SPEED;
}

fn get_direction_modifiers(direction: &common::MoveDirection) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// The ghost should not move over its target.
fn limit_movement(coordinates: &mut Vec3, direction: &common::MoveDirection, target_coordinates: &Vec3) {
    match direction {
        Up => coordinates.y = coordinates.y.min(target_coordinates.y),
        Down => coordinates.y = coordinates.y.max(target_coordinates.y),
        Left => coordinates.x = coordinates.x.max(target_coordinates.x),
        Right => coordinates.x = coordinates.x.min(target_coordinates.x),
    }
}