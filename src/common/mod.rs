use std::fmt::Formatter;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::Direction::*;
use crate::map::Rotation;
use crate::map::Rotation::*;

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

    pub fn rotate_right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }

    pub fn rotate_left(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
        }
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            D0 => *self,
            D90 => self.rotate_right(),
            D180 => self.opposite(),
            D270 => self.rotate_left(),
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