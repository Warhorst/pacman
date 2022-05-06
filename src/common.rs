use bevy::prelude::*;

use crate::common::MoveDirection::*;

/// A type alias for the typical components when processing movement.
/// A component bundle might be preferable, but the transform is created
/// with the SpriteComponents.
pub type MoveComponents<'a> = (&'a mut Transform, &'a mut Position, &'a mut MoveDirection);

#[derive(Copy, Clone, Component, Hash, Debug, Eq, PartialEq)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    /// Returns the distance between two positions.
    pub fn distance_to(&self, other: &Position) -> usize {
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

#[derive(Copy, Clone, Component, Debug, PartialOrd, PartialEq)]
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