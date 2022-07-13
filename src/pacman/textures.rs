use bevy::prelude::*;

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::map::Rotation;

pub (in crate::pacman) fn update_pacman_appearance(
    mut query: Query<(&Direction, &mut Transform), With<Pacman>>
) {
    for (direction, mut transform) in query.iter_mut() {
        match direction {
            Up => transform.rotation = Rotation::D90.quat_z(),
            Down => transform.rotation = Rotation::D270.quat_z(),
            Left => transform.rotation = Rotation::D0.quat_z(),
            Right => transform.rotation = Rotation::D180.quat_z(),
        }
    }
}