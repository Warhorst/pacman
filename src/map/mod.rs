use std::collections::HashMap;

use bevy::app::{AppBuilder, Plugin};
use bevy::prelude::*;
use bevy::property::serde::export::{Formatter, TryFrom};

use FieldType::*;

use crate::common;
use crate::common::Position;
use crate::constants::WALL_DIMENSION;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::{Blinky, Clyde, Inky, Pinky};
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
    for position in get_wall_positions(&board) {
        commands.spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            sprite: Sprite::new(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
            ..Default::default()
        });
    }

    for position in board.positions_of_type(GhostWall) {
        commands.spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            sprite: Sprite::new(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
            ..Default::default()
        });
    }
}

/// Return all positions of fields that are considered walls.
fn get_wall_positions(board: &Board) -> Vec<&Position> {
    let mut walls = board.positions_of_type(Wall);
    walls.push(board.position_of_type(GhostCorner(Blinky)));
    walls.push(board.position_of_type(GhostCorner(Pinky)));
    walls.push(board.position_of_type(GhostCorner(Inky)));
    walls.push(board.position_of_type(GhostCorner(Clyde)));
    walls
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FieldType {
    Free,
    Wall,
    PacManSpawn,
    Point,
    LeftTunnel,
    RightTunnel,
    GhostWall,
    GhostSpawn,
    GhostCorner(Ghost),
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
            'B' => Ok(GhostCorner(Blinky)),
            'P' => Ok(GhostCorner(Pinky)),
            'I' => Ok(GhostCorner(Inky)),
            'C' => Ok(GhostCorner(Clyde)),
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

/// Represents the neighbour of a specific field, with ist type and the direction
/// relative to the original position.
pub struct Neighbour {
    pub position: Position,
    pub field_type: FieldType,
    pub direction: common::Direction,
}

impl Neighbour {
    pub fn new(position: Position, field_type: FieldType, direction: common::Direction) -> Self {
        Neighbour { position, field_type, direction }
    }
}
