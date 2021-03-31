use bevy::prelude::*;

use crate::common;
use crate::common::Direction::*;
use crate::common::Movement;
use crate::common::Movement::*;
use crate::constants::GHOST_SPEED;
use crate::ghosts::Target;
use crate::map::board::Board;

/// Moves a ghost to his next position.
pub (in crate::ghosts) struct Mover<'a> {
    board: &'a Board,
    delta_seconds: f32,
    movement: &'a Movement,
    target: &'a mut Target,
    ghost_coordinates: &'a mut Vec3,
}

impl <'a> Mover<'a> {
    pub fn new(board: &'a Board, delta_seconds: f32, movement: &'a Movement, target: &'a mut Target, ghost_coordinates: &'a mut Vec3) -> Self {
        Mover { board, delta_seconds, movement, target, ghost_coordinates }
    }

    pub fn move_ghost(&mut self) {
        if self.target.is_not_set() {
            return
        }

        let direction = match self.movement {
            Idle => return,
            Moving(dir) => dir
        };

        let target_coordinates = self.board.coordinates_of_position(self.target.get_position());
        self.move_in_direction(&direction);
        self.limit_movement(&direction, &target_coordinates);
        if *self.ghost_coordinates == target_coordinates {
            self.target.clear();
        }
    }

    fn move_in_direction(&mut self, direction: &common::Direction) {
        let (x, y) = self.get_direction_modifiers(direction);
        self.ghost_coordinates.x += self.delta_seconds * x * GHOST_SPEED;
        self.ghost_coordinates.y += self.delta_seconds * y * GHOST_SPEED;
    }

    fn get_direction_modifiers(&self, direction: &common::Direction) -> (f32, f32) {
        match direction {
            Up => (0.0, 1.0),
            Down => (0.0, -1.0),
            Left => (-1.0, 0.0),
            Right => (1.0, 0.0)
        }
    }

    /// The ghost should not move over its target.
    fn limit_movement(&mut self, direction: &common::Direction, target_coordinates: &Vec3) {
        match direction {
            Up => self.ghost_coordinates.y = self.ghost_coordinates.y.min(target_coordinates.y),
            Down => self.ghost_coordinates.y = self.ghost_coordinates.y.max(target_coordinates.y),
            Left => self.ghost_coordinates.x = self.ghost_coordinates.x.max(target_coordinates.x),
            Right => self.ghost_coordinates.x = self.ghost_coordinates.x.min(target_coordinates.x),
        }
    }
}