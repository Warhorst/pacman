use bevy::prelude::Transform;

use Direction::*;

/// A type alias for the typical components when processing movement.
/// A component bundle might be preferable, but the transform is created
/// with the SpriteComponents.
pub type MoveComponents<'a> = (&'a mut Transform, &'a mut Position, &'a mut Movement);

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
pub enum Movement {
    Idle,
    Moving(Direction),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
}