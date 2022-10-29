use std::fmt::Formatter;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::Direction::*;
use crate::common::position::Position;

pub mod position;

#[derive(Copy, Clone, Component, Deserialize, Debug, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right
        }
    }

    pub fn reverse(&mut self) {
        *self = self.opposite()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Up => "up",
            Down => "down",
            Left => "left",
            Right => "right"
        }.to_string())
    }
}

pub trait XYEqual {
    fn xy_equal_to(&self, other: &Self) -> bool;
}

impl XYEqual for Vec3 {
    fn xy_equal_to(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub trait FromPositions {
    fn from_positions<'a>(positions: impl IntoIterator<Item=&'a Position>, z: f32) -> Self;
}

impl FromPositions for Vec3 {
    fn from_positions<'a>(positions: impl IntoIterator<Item=&'a Position>, z: f32) -> Self {
        let positions = positions.into_iter().collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);

        let (pos0, pos1) = (positions[0], positions[1]);
        let neighbour_direction = pos0.get_neighbour_direction(&pos1).expect("the two positions must be neighbored");
        let (vec0, vec1) = (pos0.to_vec(0.0), pos1.to_vec(0.0));

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
            }
        }
    }
}