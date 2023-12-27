use bevy::prelude::*;

use crate::game::direction::Dir::*;
use crate::game::position::Pos;

/// Provides a helper method to set the x and y value from this Vec3 to the x and y from another Vec3.
/// Using 'this = other' might overwrite the z value wrong, leading to graphic errors.
pub trait SetXY {
    fn set_xy(&mut self, target: &Vec3);
}

impl SetXY for Vec3 {
    fn set_xy(&mut self, target: &Vec3) {
        self.x = target.x;
        self.y = target.y;
    }
}

/// Provides a helper method to check if x and y values of two Vec3 are equal.
/// Using 'this == other' might return false incorrectly due to non equal z values.
pub trait XYEqual {
    fn xy_equal(&self, other: &Self) -> bool;
}

impl XYEqual for Vec3 {
    fn xy_equal(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Provides a helper method to create a Vec3 from an iterator of Position.
/// Despite taking an iterator, exactly two elements are expected. Using iterators just makes things easier.
pub trait FromPositions {
    fn from_positions<'a>(positions: impl IntoIterator<Item=&'a Pos>, z: f32) -> Self;
}

impl FromPositions for Vec3 {
    fn from_positions<'a>(positions: impl IntoIterator<Item=&'a Pos>, z: f32) -> Self {
        let positions = positions.into_iter().collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);

        let (pos0, pos1) = (positions[0], positions[1]);
        let neighbour_direction = pos0.get_direction_to_neighbour(&pos1);
        let (vec0, vec1) = (pos0.to_vec3(0.0), pos1.to_vec3(0.0));

        match neighbour_direction {
            Up | Down => {
                let x = vec0.x;
                let y = (vec0.y + vec1.y) / 2.0;
                Vec3::new(x, y, 0.0)
            }
            Left | Right => {
                let x = (vec0.x + vec1.x) / 2.0;
                let y = vec0.y;
                Vec3::new(x, y, z)
            },
        }
    }
}