use bevy::prelude::*;
use pad::Direction;
use pad::Direction::*;
use serde::{Deserialize, Serialize};
use crate::game::direction::Dir::*;

/// The direction some entity is currently moving to
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Dir {
    #[default]
    Right,
    Left,
    Up,
    Down
}

impl Dir {
    pub fn from_direction(dir: Direction) -> Self {
        match dir {
            XP => Right,
            XM => Left,
            YP => Up,
            YM => Down,
            _ => panic!("invalid direction")
        }
    }

    pub fn opposite(&self) -> Dir {
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up
        }
    }

    pub fn to_direction(&self) -> Direction {
        match self {
            Right => XP,
            Left => XM,
            Up => YP,
            Down => YM
        }
    }
}