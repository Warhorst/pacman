use bevy::prelude::*;

use crate::common;
use crate::common::{Movement, Position};
use crate::common::Direction::*;
use crate::common::Movement::*;
use crate::constants::GHOST_SPEED;
use crate::energizer::EnergizerEaten;
use crate::ghosts::Ghost;
use crate::ghosts::target::Target;
use crate::map::board::Board;

/// Indicates that a ghost should turn around after reaching its target.
pub struct MovementReverseMarker {
    set: bool,
}

impl MovementReverseMarker {
    pub fn new() -> Self {
        MovementReverseMarker {
            set: false
        }
    }

    pub fn set(&mut self) {
        self.set = true
    }

    pub fn unset(&mut self) {
        self.set = false
    }

    pub fn is_set(&self) -> bool {
        self.set
    }
}

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(move_ghost.system())
            .add_system(mark_movement_to_reverse_after_pacman_ate_energizer.system());
    }
}

fn move_ghost(
    time: Res<Time>,
    board: Res<Board>,
    mut query: Query<(&mut Movement, &mut Position, &mut Target, &mut Transform, &mut MovementReverseMarker)>,
) {
    for (mut movement, mut position, mut target, mut transform, mut movement_reverse_marker) in query.iter_mut() {
        if target.is_not_set() { continue; }

        let mut coordinates = &mut transform.translation;
        let delta_seconds = time.delta_seconds();
        let direction = match *movement {
            Idle => return,
            Moving(dir) => dir
        };

        let target_coordinates = board.coordinates_of_position(target.get_position());
        move_in_direction(&mut coordinates, delta_seconds, &direction);
        limit_movement(&mut coordinates, &direction, &target_coordinates);
        if *coordinates == target_coordinates {
            target.clear();
            if movement_reverse_marker.is_set() {
                movement.reverse();
                movement_reverse_marker.unset();
            }
        }
        *position = board.position_of_coordinates(coordinates)
    }
}

fn mark_movement_to_reverse_after_pacman_ate_energizer(
    mut event_reader: EventReader<EnergizerEaten>,
    mut query: Query<&mut MovementReverseMarker, With<Ghost>>,
) {
    for _ in event_reader.iter() {
        for mut movement_reverse_marker in query.iter_mut() {
            movement_reverse_marker.set()
        }
    }
}

fn move_in_direction(coordinates: &mut Vec3, delta_seconds: f32, direction: &common::Direction) {
    let (x, y) = get_direction_modifiers(direction);
    coordinates.x += delta_seconds * x * GHOST_SPEED;
    coordinates.y += delta_seconds * y * GHOST_SPEED;
}

fn get_direction_modifiers(direction: &common::Direction) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// The ghost should not move over its target.
fn limit_movement(coordinates: &mut Vec3, direction: &common::Direction, target_coordinates: &Vec3) {
    match direction {
        Up => coordinates.y = coordinates.y.min(target_coordinates.y),
        Down => coordinates.y = coordinates.y.max(target_coordinates.y),
        Left => coordinates.x = coordinates.x.max(target_coordinates.x),
        Right => coordinates.x = coordinates.x.min(target_coordinates.x),
    }
}