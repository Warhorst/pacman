use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Formatter;

use bevy::app::Plugin;
use bevy::prelude::*;

use FieldType::*;

use crate::common;
use crate::common::Position;
use crate::constants::WALL_DIMENSION;
use crate::map::board::Board;

pub mod board;
mod pacmap;
mod new_map;

type FieldTypeMatrix = Vec<Vec<FieldType>>;
type PositionTypeMap = HashMap<Position, FieldType>;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Board::new())
            .add_startup_system(spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands, board: Res<Board>) {
    for position in board.positions_of_type(Wall) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                ..Default::default()
            });
    }

    for position in board.positions_of_type(GhostWall) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                ..Default::default()
            });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FieldType {
    Free,
    Wall,
    PacManSpawn,
    Point,
    GhostWall,
    GhostSpawn,
    BlinkyCorner,
    PinkyCorner,
    InkyCorner,
    ClydeCorner,
    TunnelEntrance(usize),
    TunnelOpening,
    TunnelHallway,
    InvisibleWall,
    Energizer,
}

impl TryFrom<char> for FieldType {
    type Error = FieldTypeFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let field_type = match value {
            ' ' => Free,
            'W' => Wall,
            'M' => PacManSpawn,
            '#' => Point,
            '_' => GhostWall,
            'G' => GhostSpawn,
            'B' => BlinkyCorner,
            'P' => PinkyCorner,
            'I' => InkyCorner,
            'C' => ClydeCorner,
            'T' => TunnelOpening,
            'H' => TunnelHallway,
            'V' => InvisibleWall,
            'E' => Energizer,
            c if c.is_numeric() => TunnelEntrance(c.to_digit(10).unwrap() as usize),
            other => return Err(FieldTypeFromCharError { error_char: other })
        };
        Ok(field_type)
    }
}

#[derive(Debug)]
pub struct FieldTypeFromCharError {
    pub error_char: char,
}

impl std::error::Error for FieldTypeFromCharError {}

impl std::fmt::Display for FieldTypeFromCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown character: {}", self.error_char)
    }
}

/// Represents the neighbour of a specific field, with ist type and the direction
/// relative to the original position.
#[derive(Copy, Clone)]
pub struct Neighbour {
    pub position: Position,
    pub field_type: FieldType,
    pub direction: common::MoveDirection,
}

impl Neighbour {
    pub fn new(position: Position, field_type: FieldType, direction: common::MoveDirection) -> Self {
        Neighbour { position, field_type, direction }
    }
}
