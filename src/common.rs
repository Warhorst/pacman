use std::fmt::Formatter;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;

use crate::common::Direction::*;
use crate::constants::FIELD_DIMENSION;

#[derive(Copy, Clone, Deserialize, Hash, Debug, Eq, PartialEq, Serialize)]
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
    
    pub fn neighbour_position(&self, direction: &Direction) -> Position {
        match direction {
            Up => Position::new(self.x, self.y + 1),
            Down => Position::new(self.x, self.y - 1),
            Left => Position::new(self.x - 1, self.y),
            Right => Position::new(self.x + 1, self.y),
        }
    }

    pub fn get_neighbour_in_direction(&self, direction: &Direction) -> Neighbour {
        match direction {
            Up => Neighbour::new(Position::new(self.x, self.y + 1), *direction),
            Down => Neighbour::new(Position::new(self.x, self.y - 1), *direction),
            Left => Neighbour::new(Position::new(self.x - 1, self.y), *direction),
            Right => Neighbour::new(Position::new(self.x + 1, self.y), *direction),
        }
    }

    pub fn neighbour_behind(&self, direction: &Direction) -> Neighbour {
        self.get_neighbour_in_direction(&direction.opposite())
    }

    /// Return the direction where to find the other position when neighbored.
    /// If not neighbored, return None.
    pub fn get_neighbour_direction(&self, other: &Position) -> Option<Direction> {
        self.get_neighbours()
            .into_iter()
            .filter(|n| &n.position == other)
            .map(|n| n.direction)
            .next()
    }

    pub fn get_neighbours(&self) -> [Neighbour; 4] {
        [
            self.get_neighbour_in_direction(&Up),
            self.get_neighbour_in_direction(&Down),
            self.get_neighbour_in_direction(&Left),
            self.get_neighbour_in_direction(&Right),
        ]
    }

    pub fn get_nearest_from<'a, I: IntoIterator<Item=&'a Position>>(&self, iter: I) -> &'a Position {
        iter.into_iter()
            .min_by(|pos_0, pos_1| self.distance_to(pos_0).cmp(&self.distance_to(pos_1)))
            .expect("The given iterator of positions should not be empty!")
    }

    pub fn get_nearest_from_owned<'a, I: IntoIterator<Item=Position>>(&self, iter: I) -> Position {
        iter.into_iter()
            .min_by(|pos_0, pos_1| self.distance_to(pos_0).cmp(&self.distance_to(pos_1)))
            .expect("The given iterator of positions should not be empty!")
    }

    pub fn get_position_in_direction_with_offset(&self, direction: &Direction, offset: usize) -> Self {
        match direction {
            Up => Position::new(self.x, self.y + (offset as isize)),
            Down => Position::new(self.x, self.y - (offset as isize)),
            Left => Position::new(self.x - (offset as isize), self.y),
            Right => Position::new(self.x + (offset as isize), self.y)
        }
    }
}

impl From<Vec3> for Position {
    fn from(vec: Vec3) -> Self {
        let x = (vec.x + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        let y = (vec.y + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        Position::new(x as isize, y as isize)
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

impl From<Position> for Vec3 {
    fn from(pos: Position) -> Self {
        let x = (pos.x as f32) * FIELD_DIMENSION;
        let y = (pos.y as f32) * FIELD_DIMENSION;
        Vec3::new(x, y, 0.0)
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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
pub struct Neighbour {
    pub position: Position,
    pub coordinates: Vec3,
    pub direction: Direction
}

impl Neighbour {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self { position, direction, coordinates: position.into()  }
    }
}

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

    pub fn reverse(&mut self) {
        *self = self.opposite()
    }
}

/// Provides helper methods for working with coordinates (Vec3).
///
/// The games logic widely uses positions to perform specific checks (like collisions and distance calculations).
/// These methods aim to make this easier.
pub trait Vec3Helper {
    fn pos(&self) -> Position;

    fn pos_center(&self) -> Vec3;

    fn set_xy(&mut self, target: &Vec3);

    fn get_nearest_from(&self, iter: impl IntoIterator<Item = Vec3>) -> Vec3;

    fn get_neighbours(&self) -> [Neighbour; 4];
}

impl Vec3Helper for Vec3 {
    /// Vec3 to Position. Shorter that calling from all the time.
    fn pos(&self) -> Position {
        Position::from(self)
    }

    /// The center coordinates of the position the coordinates belongs to.
    fn pos_center(&self) -> Vec3 {
        Vec3::from(Position::from(self))
    }

    /// Set this coordinates x and y to the ones of the other transform.
    /// The z value defines what is rendered before or after another sprite,
    /// so this value should not be changed.
    fn set_xy(&mut self, target: &Vec3) {
        self.x = target.x;
        self.y = target.y;
    }

    fn get_nearest_from(&self, iter: impl IntoIterator<Item=Vec3>) -> Vec3 {
        iter.into_iter()
            .min_by(|t_0, t_1| self.distance(*t_0).partial_cmp(&self.distance(*t_1)).expect("the distance calculation should not create NaN"))
            .expect("The given iterator of positions should not be empty!")
    }

    fn get_neighbours(&self) -> [Neighbour; 4] {
        self.pos().get_neighbours()
    }
}