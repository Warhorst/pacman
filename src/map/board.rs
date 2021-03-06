use std::fs::File;

use bevy::prelude::*;

use crate::common::{Direction::*, Position};
use crate::common;
use crate::constants::{FIELD_DIMENSION, USED_PACMAP_PATH, WALL_DIMENSION};
use crate::map::{FieldType, Neighbour, PositionTypeMap};
use crate::map::pacmap::PacMap;

/// The Board is a resource that provides methods to easily manipulate
/// entities on the map (pacman, ghosts, points, etc).
pub struct Board {
    fields: PositionTypeMap,
    width: usize,
    height: usize,
    board_root: Vec2,
}

impl Board {
    pub(in crate::map) fn new() -> Self {
        let pacmap = PacMap::from_read(File::open(USED_PACMAP_PATH).unwrap());
        let width = pacmap.width;
        let height = pacmap.height;
        let board_root = Self::calculate_board_root(width, height);
        Board {
            width,
            height,
            fields: pacmap.into_position_type_map(),
            board_root,
        }
    }

    /// Calculate a board root where the board is always centered.
    fn calculate_board_root(width: usize, height: usize) -> Vec2 {
        let x = -(width as f32 * FIELD_DIMENSION / 2.0);
        let y = -(height as f32 * FIELD_DIMENSION / 2.0);
        Vec2::new(x, y)
    }

    pub fn coordinates_of_position(&self, position: &Position) -> Vec3 {
        let x = self.board_root.x + (position.x() as f32) * FIELD_DIMENSION;
        let y = self.board_root.y + (position.y() as f32) * FIELD_DIMENSION;
        Vec3::new(x, y, 0.0)
    }

    pub fn position_of_coordinates(&self, coordinates: &Vec3) -> Position {
        let x = (coordinates.x - self.board_root.x + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        let y = (coordinates.y - self.board_root.y + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        Position::new(x as usize, y as usize)
    }

    pub fn position_in_direction(&self, position: &Position, direction: &common::Direction) -> Option<Position> {
        match direction {
            Up => self.position_up_of(position),
            Down => self.position_down_of(position),
            Left => self.position_left_of(position),
            Right => self.position_right_of(position),
        }
    }

    fn position_up_of(&self, position: &Position) -> Option<Position> {
        match position.y() {
            y if y < self.height - 1 => Some(Position::new(position.x(), y + 1)),
            _ => None
        }
    }

    fn position_down_of(&self, position: &Position) -> Option<Position> {
        match position.y() {
            y if y > 0 => Some(Position::new(position.x(), y - 1)),
            _ => None
        }
    }

    fn position_left_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            x if x > 0 => Some(Position::new(x - 1, position.y())),
            _ => None
        }
    }

    fn position_right_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            x if x < self.width - 1 => Some(Position::new(x + 1, position.y())),
            _ => None
        }
    }

    /// Determines if pacmans current coordinates are in the center of his current position. The center of the position is
    /// its middle point with the width/height of the accumulated distance between pacman and the walls.
    /// Assumes pacman is larger than a wall.
    pub fn are_coordinates_in_field_center(&self, direction: &common::Direction, position: &Position, coordinates: &Vec3, entity_dimension: f32) -> bool {
        let position_coordinates = self.coordinates_of_position(position);
        let entity_wall_distance = match entity_dimension > WALL_DIMENSION {
            true => entity_dimension - WALL_DIMENSION,
            false => 0.0
        };
        match direction {
            Left | Right => {
                let y_start = position_coordinates.y - entity_wall_distance;
                let y_end = position_coordinates.y + entity_wall_distance;
                coordinates.y >= y_start && coordinates.y <= y_end
            }
            Up | Down => {
                let x_start = position_coordinates.x - entity_wall_distance;
                let x_end = position_coordinates.x + entity_wall_distance;
                coordinates.x >= x_start && coordinates.x <= x_end
            }
        }
    }

    /// Return the position of one specific field type. Of the FieldType
    /// should be exactly one on the map. If not, the program panics.
    pub fn position_of_type(&self, field_type: FieldType) -> &Position {
        let type_positions = self.fields.iter()
            .filter(|(_, t)| *t == &field_type)
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
        match type_positions.len() {
            1 => type_positions[0],
            _ => panic!("Expected exactly one field with type {:?}", field_type)
        }
    }

    pub fn positions_of_type(&self, field_type: FieldType) -> Vec<&Position> {
        self.positions_of_type_filter(|ft| ft == &field_type)
    }

    pub fn positions_of_type_filter(&self, filter: impl Fn(&FieldType) -> bool) -> Vec<&Position> {
        self.fields.iter()
            .filter(|(_, t)| filter(t))
            .map(|(pos, _)| pos)
            .collect()
    }

    pub fn type_of_position(&self, position: &Position) -> &FieldType {
        self.fields.get(position).expect("The given position should be on the map.")
    }

    pub fn neighbours_of(&self, position: &Position) -> Vec<Neighbour> {
        let neighbour_position_options = vec![
            (Up, self.position_up_of(position)),
            (Down, self.position_down_of(position)),
            (Left, self.position_left_of(position)),
            (Right, self.position_right_of(position)),
        ];
        neighbour_position_options.into_iter()
            .filter(|(_, option)| option.is_some())
            .map(|(dir, option)| match option {
                Some(pos) => Neighbour::new(pos, *self.type_of_position(&pos), dir),
                None => panic!()
            })
            .collect()
    }
}