use bevy::prelude::*;
use pad::Position;
use pad::Direction::*;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

/// The x and y coordinates of some entity.
#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Pos(Position);

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos(Position::new(x, y))
    }

    pub fn from_vec3(vec: Vec3) -> Pos {
        Pos(Position::from_vec3(vec, FIELD_DIMENSION))
    }

    pub fn x(&self) -> isize {
        self.0.x
    }

    pub fn y(&self) -> isize {
        self.0.y
    }

    /// Get the neighboured position in the given direction.
    pub fn neighbour_in_direction(&self, dir: Dir) -> Pos {
        Pos(self.0.neighbour_in_direction(dir.to_direction()))
    }

    pub fn position_in_direction(&self, dir: Dir, distance: usize) -> Pos {
        Pos(self.0.position_in_direction(dir.to_direction(), distance))
    }

    pub fn neighbours_with_directions(&self) -> impl IntoIterator<Item=(Pos, Dir)> {
        self.0.cardinal_neighbours_with_directions()
            .into_iter()
            .map(|(pos, dir)| (Pos(pos), Dir::from_direction(dir)))
    }

    pub fn get_direction_to_neighbour(&self, other: &Pos) -> Dir {
        match self.0.get_direction_to_neighbour(&other.0) {
            Some(d) => match d {
                XP => Right,
                XM => Left,
                YP => Up,
                YM => Down,
                _ => panic!("invalid direction")
            }
            None => panic!("positions are not neighboured")
        }
    }

    pub fn distance(&self, other: &Pos) -> f32 {
        self.0.euclidean_distance(&other.0)
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        self.0.to_vec3(FIELD_DIMENSION, z)
    }
}