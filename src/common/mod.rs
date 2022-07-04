use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::Direction::*;

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

    pub fn reverse(&mut self) {
        *self = self.opposite()
    }
}