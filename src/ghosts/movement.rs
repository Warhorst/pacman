use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::common::MoveDirection::*;
use crate::ghosts::target::Target;
use crate::map::board::Board;
use crate::speed::Speed;

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
    mut query: Query<(Entity, &MoveDirection, &mut Position, &Target, &mut Transform, &Speed)>,
) {
    for (entity, direction, mut position, target, mut transform, speed) in query.iter_mut() {
        let mut coordinates = &mut transform.translation;
        let delta_seconds = time.delta_seconds();
        let target_coordinates = board.coordinates_of_position(target);
        move_in_direction(&mut coordinates, delta_seconds, &direction, speed);
        limit_movement(&mut coordinates, &direction, &target_coordinates);

        // TODO maybe move this to the target plugin to keep things simple
        if *coordinates == target_coordinates {
            commands.entity(entity).remove::<Target>();
        }

        *position = board.position_of_coordinates(coordinates)
    }
}

fn move_in_direction(coordinates: &mut Vec3, delta_seconds: f32, direction: &MoveDirection, speed: &Speed) {
    let (x, y) = get_direction_modifiers(direction);
    coordinates.x += delta_seconds * x * **speed;
    coordinates.y += delta_seconds * y * **speed;
}

fn get_direction_modifiers(direction: &MoveDirection) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// The ghost should not move over its target.
fn limit_movement(coordinates: &mut Vec3, direction: &MoveDirection, target_coordinates: &Vec3) {
    match direction {
        Up => coordinates.y = coordinates.y.min(target_coordinates.y),
        Down => coordinates.y = coordinates.y.max(target_coordinates.y),
        Left => coordinates.x = coordinates.x.max(target_coordinates.x),
        Right => coordinates.x = coordinates.x.min(target_coordinates.x),
    }
}