use std::collections::HashMap;

use bevy::property::serde::export::{Formatter, TryFrom};

use FieldType::*;

use crate::common::Position;

pub mod pacmap;
pub mod board;

pub type FieldTypeMatrix = Vec<Vec<FieldType>>;
pub type PositionTypeMap = HashMap<Position, FieldType>;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FieldType {
    Free,
    Wall,
    LeftTunnel,
    RightTunnel,
}

impl TryFrom<char> for FieldType {
    type Error = FieldTypeFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Free),
            'W' => Ok(Wall),
            'L' => Ok(LeftTunnel),
            'R' => Ok(RightTunnel),
            error_char => Err(FieldTypeFromCharError { error_char })
        }
    }
}

#[derive(Debug)]
pub struct FieldTypeFromCharError {
    pub error_char: char
}

impl std::error::Error for FieldTypeFromCharError {}

impl std::fmt::Display for FieldTypeFromCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown character: {}", self.error_char)
    }
}
