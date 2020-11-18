use bevy::prelude::*;

use crate::common;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::Position;
use crate::constants::{PACMAN_DIMENSION, PACMAN_SPEED};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::pacman::Movement;
use crate::pacman::Movement::*;

/// Moves pacman to his next position.
pub struct Mover<'a> {
    board: &'a Board,
    delta_seconds: f32,
    movement: &'a mut Movement,
    pacman_position: &'a mut Position,
    pacman_coordinates: &'a mut Vec3
}

impl<'a> Mover<'a> {
    pub fn new(board: &'a Board, delta_seconds: f32, movement: &'a mut Movement, pacman_position: &'a mut Position, pacman_coordinates: &'a mut Vec3) -> Self {
        Mover { board, delta_seconds, movement, pacman_position, pacman_coordinates }
    }

    /// Move pacman by updating his movement, position and coordinates.
    /// Do not move him if he is currently idle.
    pub fn move_pacman(&mut self) {
        let direction = match *self.movement {
            Idle => return,
            Moving(dir) => dir
        };

        // *self.pacman_position = self.board.position_of_coordinates(self.pacman_coordinates);
        let mut new_coordinates = self.calculate_new_coordinates(&direction);
        let new_position = self.board.position_of_coordinates(&new_coordinates);
        let mut new_movement = Moving(direction);

        if self.is_going_to_collide_with_obstacle(&direction, &new_position, &new_coordinates) {
            self.process_collision(&direction, &new_position, &mut new_coordinates, &mut new_movement)
        } else {
            self.center_position(&direction, &new_position, &mut new_coordinates)
        }

        *self.pacman_coordinates = new_coordinates;
        *self.pacman_position = new_position;
        *self.movement = new_movement
    }

    /// Calculate pacmans new coordinates on the window based on his speed and the time.
    fn calculate_new_coordinates(&self, direction: &common::Direction) -> Vec3 {
        let (x, y) = self.get_modifiers_for_direction(direction);
        let mut new_coordinates = *self.pacman_coordinates;
        *new_coordinates.x_mut() += self.delta_seconds * x * PACMAN_SPEED;
        *new_coordinates.y_mut() += self.delta_seconds * y * PACMAN_SPEED;
        new_coordinates
    }

    fn get_modifiers_for_direction(&self, direction: &Direction) -> (f32, f32) {
        match direction {
            Up => (0.0, 1.0),
            Down => (0.0, -1.0),
            Left => (-1.0, 0.0),
            Right => (1.0, 0.0)
        }
    }

    /// Determine if pacman will collide with an obstacle if he is going further in his current direction.
    fn is_going_to_collide_with_obstacle(&self, direction: &common::Direction, new_position: &Position, new_coordinates: &Vec3) -> bool {
        match self.board.position_in_direction(new_position, direction) {
            Some(pos) if self.position_is_obstacle(&pos) => true,
            Some(pos) => !self.board.are_coordinates_in_field_center(direction, &pos, new_coordinates, PACMAN_DIMENSION),
            None => true
        }
    }

    /// Tells if the given position is an obstacle for pacman.
    fn position_is_obstacle(&self, position: &Position) -> bool {
        match self.board.type_of_position(position) {
            Wall | GhostWall => true,
            _ => false
        }
    }

    /// Limit pacmans movement if he reached an obstacle and stop him.
    fn process_collision(&self, direction: &common::Direction, new_position: &Position, new_coordinates: &mut Vec3, new_movement: &mut Movement) {
        let field_coordinates = self.board.coordinates_of_position(new_position);
        self.limit_movement(direction, &field_coordinates, new_coordinates);
        self.stop_if_at_border(direction, &field_coordinates, new_coordinates, new_movement)
    }

    /// Because the next field is an obstacle, pacman can not go beyond his current field.
    fn limit_movement(&self, direction: &Direction, field_coordinates: &Vec3, new_coordinates: &mut Vec3) {
        match direction {
            Up => *new_coordinates.y_mut() = new_coordinates.y().min(field_coordinates.y()),
            Down => *new_coordinates.y_mut() = new_coordinates.y().max(field_coordinates.y()),
            Left => *new_coordinates.x_mut() = new_coordinates.x().max(field_coordinates.x()),
            Right => *new_coordinates.x_mut() = new_coordinates.x().min(field_coordinates.x())
        }
    }

    /// If pacman is at a border, he can not go further and stop.
    fn stop_if_at_border(&self, direction: &Direction, field_coordinates: &Vec3, new_coordinates: &mut Vec3, movement: &mut Movement) {
        match direction {
            Up | Down => if field_coordinates.y() == new_coordinates.y() {
                *movement = Idle
            }
            ,
            Left | Right => if field_coordinates.x() == new_coordinates.x() {
                *movement = Idle
            }
        }
    }

    /// Center pacmans current position in the middle of his current field.
    /// The purpose of this method is to keep equally sized gaps to the hallway pacman is currently passing.
    fn center_position(&self, direction: &common::Direction, new_position: &Position, new_coordinates: &mut Vec3) {
        let position_coordinates = self.board.coordinates_of_position(new_position);
        match direction {
            Up | Down => *new_coordinates.x_mut() = position_coordinates.x(),
            Left | Right => *new_coordinates.y_mut() = position_coordinates.y()
        }
    }
}