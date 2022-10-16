use bevy::prelude::*;
use crate::ghosts::target::TargetSetter;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::XYEqual;

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
        let respawn = self.ghost_house.respawn_coordinates_of(self.components.ghost);
        let above_respawn = self.coordinates_slightly_in_direction(respawn, self.ghost_house.entrance_direction);
        let below_respawn = self.coordinates_slightly_in_direction(respawn, self.ghost_house.entrance_direction.opposite());

        if coordinates.xy_equal_to(&respawn) {
            match *self.components.direction {
                dir if dir == self.ghost_house.entrance_direction => self.components.target.set(above_respawn),
                _ => self.components.target.set(below_respawn)
            };
        } else if coordinates.xy_equal_to(&above_respawn) {
            self.components.target.set(below_respawn);
            *self.components.direction = self.ghost_house.entrance_direction.opposite();
        } else if coordinates.xy_equal_to(&below_respawn) {
            self.components.target.set(above_respawn);
            *self.components.direction = self.ghost_house.entrance_direction;
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
        let center = self.ghost_house.center_coordinates();

        match self.ghost_house.entrance_direction {
            Up | Down => coordinates.x == center.x,
            Left | Right => coordinates.y == center.y,
        }
    }

    fn move_to_entrance(&mut self) {
        *self.components.direction = self.ghost_house.entrance_direction;
        self.components.target.set(self.ghost_house.coordinates_in_front_of_entrance());
    }

    fn is_near_spawn(&self) -> bool {
        let coordinates = self.components.transform.translation;
        let respawn = self.ghost_house.respawn_coordinates_of(self.components.ghost);
        match self.ghost_house.entrance_direction {
            Up | Down => coordinates.x == respawn.x,
            Left | Right => coordinates.y == respawn.y
        }
    }

    fn move_near_center(&mut self) {
        let coordinates = self.components.transform.translation;
        let center = self.ghost_house.center_coordinates();
        let respawn = self.ghost_house.respawn_coordinates_of(self.components.ghost);

        *self.components.direction = match self.ghost_house.entrance_direction {
            Up | Down => match respawn.x < center.x {
                true => Right,
                false => Left
            },
            Left | Right => match respawn.y < center.y {
                true => Up,
                false => Down
            }
        };

        match self.ghost_house.entrance_direction {
            Up | Down => self.components.target.set(Vec3::new(center.x, coordinates.y, 0.0)),
            Left | Right => self.components.target.set(Vec3::new(coordinates.x, center.y, 0.0)),
        }
    }

    /// A ghost in the ghost house does not walk a full field in the ghost house (because he would clip into the wall).
    /// When bouncing around in the ghost house, he only moves slightly in one direction.
    fn coordinates_slightly_in_direction(&self, v: Vec3, d: Direction) -> Vec3 {
        let distance = self.dimensions.field() / 2.0;
        match d {
            Up => Vec3::new(v.x, v.y + distance, v.z),
            Down => Vec3::new(v.x, v.y - distance, v.z),
            Left => Vec3::new(v.x - distance, v.y, v.z),
            Right => Vec3::new(v.x + distance, v.y, v.z),
        }
    }
}