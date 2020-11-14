use std::collections::HashMap;

use bevy::app::{AppBuilder, Plugin};
use bevy::prelude::*;
use bevy::property::serde::export::{Formatter, TryFrom};

use FieldType::*;

use crate::common::Position;
use crate::constants::WALL_DIMENSION;
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
            .add_startup_system(spawn_walls.system());
    }
}

fn spawn_walls(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for position in board.get_wall_positions() {
        let color_material = Color::rgb(0.0, 0.0, 1.0).into();
        commands.spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            sprite: Sprite::new(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
            ..Default::default()
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum FieldType {
    Free,
    Wall,
    PacManSpawn,
    Point,
    LeftTunnel,
    RightTunnel,
    GhostWall,
    GhostSpawn
}

impl TryFrom<char> for FieldType {
    type Error = FieldTypeFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Free),
            'W' => Ok(Wall),
            'M' => Ok(PacManSpawn),
            '#' => Ok(Point),
            '[' => Ok(LeftTunnel),
            ']' => Ok(RightTunnel),
            '_' => Ok(GhostWall),
            'G' => Ok(GhostSpawn),
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
