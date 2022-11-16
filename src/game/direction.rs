use std::fmt::Formatter;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::direction::Direction::*;

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