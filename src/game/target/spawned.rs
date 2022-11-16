use bevy::prelude::*;
use crate::game::target::TargetSetter;
use crate::game::direction::Direction;
use crate::game::direction::Direction::*;
use crate::game::helper::XYEqual;
use crate::constants::FIELD_DIMENSION;
use crate::game::ghosts::Ghost::{Blinky, Pinky};

impl<'a, 'b, 'c> TargetSetter<'a, 'b, 'c> {
    /// Determine the next target coordinates for a ghost when in "Spawned" state.
    ///
    /// A ghost can only leave the house if their dot counter reached its predefined limit.
    /// When ready to leave, the ghost moves from its spawn to the house center, from the center to
    /// the entrance and from the entrance were ever his destiny leads him.
    ///
    /// If a ghost cannot leave the house yet, he just moves around, eager to leave and hunt pacman.
    pub fn set_spawned_target(&mut self) {
        if self.ghost_house_gate.ghost_can_leave_house(self.components.ghost) {
            self.leave_house()
        } else {
            self.bounce_around()
        }
    }

    /// If a ghost cannot leave the ghost house, he just moves around.
    fn bounce_around(&mut self) {
        let coordinates = self.components.transform.translation;
        let respawn = self.get_spawn(*self.components.ghost).coordinates;
        let above_respawn = self.coordinates_slightly_in_direction(respawn, self.get_spawn(Pinky).spawn_direction);
        let below_respawn = self.coordinates_slightly_in_direction(respawn, self.get_spawn(Pinky).spawn_direction.opposite());

        if coordinates.xy_equal(&respawn) {
            match *self.components.direction {
                dir if dir == self.get_spawn(Pinky).spawn_direction => self.components.target.set(above_respawn),
                _ => self.components.target.set(below_respawn)
            };
        } else if coordinates.xy_equal(&above_respawn) {
            self.components.target.set(below_respawn);
            *self.components.direction = self.get_spawn(Pinky).spawn_direction.opposite();
        } else if coordinates.xy_equal(&below_respawn) {
            self.components.target.set(above_respawn);
            *self.components.direction = self.get_spawn(Pinky).spawn_direction;
        }
    }

    fn leave_house(&mut self) {
        if self.is_near_center() {
            self.move_to_entrance()
        } else if self.is_near_spawn() {
            self.move_near_center()
        }
    }

    fn is_near_center(&self) -> bool {
        let coordinates = self.components.transform.translation;
        let center = self.get_spawn(Pinky).coordinates;

        match self.get_spawn(Pinky).spawn_direction {
            Up | Down => coordinates.x == center.x,
            Left | Right => coordinates.y == center.y,
        }
    }

    fn move_to_entrance(&mut self) {
        *self.components.direction = self.get_spawn(Pinky).spawn_direction;
        let entrance_coordinates = self.get_spawn(Blinky).coordinates;
        self.components.target.set(entrance_coordinates);
    }

    fn is_near_spawn(&self) -> bool {
        let coordinates = self.components.transform.translation;
        let respawn = match *self.components.ghost {
            Blinky => self.get_spawn(Pinky).coordinates,
            _ => self.get_spawn(*self.components.ghost).coordinates,
        };

        match self.get_spawn(Pinky).spawn_direction {
            Up | Down => coordinates.x == respawn.x,
            Left | Right => coordinates.y == respawn.y
        }
    }

    fn move_near_center(&mut self) {
        let coordinates = self.components.transform.translation;
        let center = self.get_spawn(Pinky).coordinates;
        let respawn = self.get_spawn(*self.components.ghost).coordinates;

        *self.components.direction = match self.get_spawn(Pinky).spawn_direction {
            Up | Down => match respawn.x < center.x {
                true => Right,
                false => Left
            },
            Left | Right => match respawn.y < center.y {
                true => Up,
                false => Down
            }
        };

        match self.get_spawn(Pinky).spawn_direction {
            Up | Down => self.components.target.set(Vec3::new(center.x, coordinates.y, 0.0)),
            Left | Right => self.components.target.set(Vec3::new(coordinates.x, center.y, 0.0)),
        }
    }

    /// A ghost in the ghost house does not walk a full field in the ghost house (because he would clip into the wall).
    /// When bouncing around in the ghost house, he only moves slightly in one direction.
    fn coordinates_slightly_in_direction(&self, v: Vec3, d: Direction) -> Vec3 {
        let distance = FIELD_DIMENSION / 2.0;
        match d {
            Up => Vec3::new(v.x, v.y + distance, v.z),
            Down => Vec3::new(v.x, v.y - distance, v.z),
            Left => Vec3::new(v.x - distance, v.y, v.z),
            Right => Vec3::new(v.x + distance, v.y, v.z),
        }
    }
}