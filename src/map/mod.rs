use std::collections::HashMap;

use bevy::app::{AppBuilder, Plugin};
use bevy::prelude::*;
use bevy::property::serde::export::{Formatter, TryFrom};

use FieldType::*;

use crate::common::Position;
use crate::map::board::Board;

pub mod board;
mod pacmap;

type FieldTypeMatrix = Vec<Vec<FieldType>>;
type PositionTypeMap = HashMap<Position, FieldType>;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Board::new())
            .add_startup_system(spawn_map.system());
    }
}

/// Spawn the map (walkable fields, walls, tunnels).
pub fn spawn_map(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for field in board.fields() {
        let color_material = match field.field_type {
            Free | PacManSpawn => Color::rgb(0.0, 0.0, 0.0).into(),
            Wall => Color::rgb(0.0, 0.0, 1.0).into(),
            LeftTunnel | RightTunnel => Color::rgb(211.0, 211.0, 211.0).into()
        };

        commands.spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.coordinates_of_position(field.position)),
            sprite: Sprite::new(board.field_dimension),
            ..Default::default()
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum FieldType {
    Free,
    Wall,
    PacManSpawn,
    LeftTunnel,
    RightTunnel,
}

impl TryFrom<char> for FieldType {
    type Error = FieldTypeFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Free),
            'W' => Ok(Wall),
            'P' => Ok(PacManSpawn),
            'L' => Ok(LeftTunnel),
            'R' => Ok(RightTunnel),
            error_char => Err(FieldTypeFromCharError { error_char })
        }
    }
}

#[derive(Debug)]
struct FieldTypeFromCharError {
    pub error_char: char
}

impl std::error::Error for FieldTypeFromCharError {}

impl std::fmt::Display for FieldTypeFromCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown character: {}", self.error_char)
    }
}
