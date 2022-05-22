use serde::{Serialize, Deserialize};
use bevy::prelude::*;

use crate::common::MoveDirection::*;
use crate::constants::FIELD_DIMENSION;

#[derive(Copy, Clone, Component, Deserialize, Hash, Debug, Eq, PartialEq, Serialize)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position {x, y}
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    /// Returns the distance between two positions.
    pub fn distance_to(&self, other: &Position) -> isize {
        let x_diff = match self.x() < other.x() {
            true => other.x() - self.x(),
            false => self.x() - other.x()
        };
        let y_diff = match self.y() < other.y() {
            true => other.y() - self.y(),
            false => self.y() - other.y()
        };
        x_diff.pow(2) + y_diff.pow(2)
    }
}

impl From<&Vec3> for Position {
    fn from(vec: &Vec3) -> Self {
        let x = (vec.x + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        let y = (vec.y + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        Position::new(x as isize, y as isize)
    }
}

impl From<&mut Vec3> for Position {
    fn from(vec: &mut Vec3) -> Self {
        let x = (vec.x + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        let y = (vec.y + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        Position::new(x as isize, y as isize)
    }
}

impl From<&Position> for Vec3 {
    fn from(pos: &Position) -> Self {
        let x = (pos.x as f32) * FIELD_DIMENSION;
        let y = (pos.y as f32) * FIELD_DIMENSION;
        Vec3::new(x, y, 0.0)
    }
}

impl From<&mut Position> for Vec3 {
    fn from(pos: &mut Position) -> Self {
        let x = (pos.x as f32) * FIELD_DIMENSION;
        let y = (pos.y as f32) * FIELD_DIMENSION;
        Vec3::new(x, y, 0.0)
    }
}

#[derive(Copy, Clone, Component, Deserialize, Debug, Eq, PartialEq, PartialOrd, Serialize)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    pub fn opposite(&self) -> MoveDirection {
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