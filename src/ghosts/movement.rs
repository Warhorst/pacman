use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::common::MoveDirection::*;
use crate::ghosts::target::{Target, TargetSetter};
use crate::speed::Speed;
use crate::target_skip_if;

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_ghost.after(TargetSetter));
    }
}

fn move_ghost(
    time: Res<Time>,
    mut query: Query<(&MoveDirection, &mut Position, &mut Target, &mut Transform, &Speed)>,
) {
    for (direction, mut position, mut target, mut transform, speed) in query.iter_mut() {
        target_skip_if!(target not set);
        let mut coordinates = &mut transform.translation;
        let delta_seconds = time.delta_seconds();
        let target_coordinates = target.get();
        move_in_direction(&mut coordinates, delta_seconds, &direction, speed);
        limit_movement(&mut coordinates, &direction, &target_coordinates);

        if on_target(*coordinates, target_coordinates, direction) {
            // Fix slight errors which might cause ghost to get stuck
            *coordinates = target_coordinates;
            target.clear();
        }

        *position = Position::from(coordinates)
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

fn on_target(coordinates: Vec3, target: Vec3, direction: &MoveDirection) -> bool {
    match direction {
        Up | Down => coordinates.y == target.y,
        Left | Right => coordinates.x == target.x
    }
}