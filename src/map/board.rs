use std::fs::File;

use bevy::prelude::*;

use crate::common::{Direction::*, Position};
use crate::common;
use crate::map::{FieldType, PositionTypeMap};
use crate::map::FieldType::*;
use crate::map::pacmap::PacMap;

pub(in crate::map) type Fields<'a> = Vec<Field<'a>>;

pub(in crate::map) struct Field<'a> {
    pub position: &'a Position,
    pub field_type: &'a FieldType,
}

/// The Board is a resource that provides methods to easily manipulate
/// entities on the map (pacman, ghosts, points, etc).
pub struct Board {
    fields: PositionTypeMap,
    width: usize,
    height: usize,
    pub(in crate::map) field_dimension: Vec2,
    board_root: Vec2,
}

impl Board {
    pub(in crate::map) fn new() -> Self {
        let pacmap = PacMap::from_read(File::open("maps/default.pacmap").unwrap());
        let width = pacmap.width;
        let height = pacmap.height;
        let field_dimension = Vec2::new(15.0, 15.0);
        let board_root = Self::calculate_board_root(width, height, field_dimension);
        Board {
            width,
            height,
            fields: pacmap.into_position_type_map(),
            field_dimension,
            board_root,
        }
    }

    /// Calculate a board root where the board is always centered.
    fn calculate_board_root(width: usize, height: usize, field_dimension: Vec2) -> Vec2 {
        let x = - (width as f32 * field_dimension.x() / 2.0);
        let y = - (height as f32 * field_dimension.y() / 2.0);
        Vec2::new(x, y)
    }

    pub(in crate::map) fn fields(&self) -> Fields {
        self.fields.iter()
            .map(|(position, field_type)| Field { position, field_type })
            .collect()
    }

    pub fn coordinates_of_position(&self, position: &Position) -> Vec3 {
        let x = self.board_root.x() + (position.x() as f32) * self.field_dimension.x();
        let y = self.board_root.y() + (position.y() as f32) * self.field_dimension.y();
        Vec3::new(x, y, 0.0)
    }

    pub fn position_of_coordinates(&self, coordinates: &Vec3) -> Position {
        let x = (coordinates.x() - self.board_root.x() + self.field_dimension.x() / 2.0) / self.field_dimension.x();
        let y = (coordinates.y() - self.board_root.y() + self.field_dimension.y() / 2.0) / self.field_dimension.y();
        Position::new(x as usize, y as usize)
    }

    pub fn collides_with_obstacle(&self, position: &Position, direction: &common::Direction, coordinates: &Vec3, dimension: &Vec2) -> bool {
        match self.position_in_direction(position, direction) {
            Some(pos) if self.position_is_obstacle(&pos) => true,
            Some(pos) => !self.coordinates_in_field_center(coordinates, dimension, &pos, direction),
            None => true
        }
    }

    fn position_in_direction(&self, position: &Position, direction: &common::Direction) -> Option<Position> {
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

    fn position_is_obstacle(&self, position: &Position) -> bool {
        let field_type = self.fields.get(position).unwrap();
        match field_type {
            Wall => true,
            _ => false,
        }
    }

    fn coordinates_in_field_center(&self, coordinates: &Vec3, dimension: &Vec2, position: &Position, direction: &common::Direction) -> bool {
        let position_coordinates = self.coordinates_of_position(position);
        match direction {
            Left | Right => {
                let y_center_range = (self.field_dimension.y() - dimension.y()) / 2.0;
                let y_start = position_coordinates.y() - y_center_range;
                let y_end = position_coordinates.y() + y_center_range;
                coordinates.y() >= y_start && coordinates.y() <= y_end
            },
            Up | Down => {
                let x_center_range = (self.field_dimension.x() - dimension.x()) / 2.0;
                let x_start = position_coordinates.x() - x_center_range;
                let x_end = position_coordinates.x() + x_center_range;
                coordinates.x() >= x_start && coordinates.x() <= x_end
            }
        }
    }

    pub fn get_left_tunnel_position(&self) -> &Position {
        self.get_position_of_type(LeftTunnel)
    }

    pub fn get_right_tunnel_position(&self) -> &Position {
        self.get_position_of_type(RightTunnel)
    }

    pub fn get_pacman_spawn_position(&self) -> &Position {
        self.get_position_of_type(PacManSpawn)
    }

    /// Return the position of one specific field type. Of the FieldType
    /// should be exactly one on the map. If not, the program panics.
    fn get_position_of_type(&self, field_type: FieldType) -> &Position {
        let type_positions = self.fields.iter()
            .filter(|(_, t)| *t == &field_type)
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
        match type_positions.len() {
            1 => type_positions[0],
            _ => panic!("Expected exactly one field with type {:?}", field_type)
        }
    }

    pub fn get_point_positions(&self) -> Vec<&Position> {
        self.fields.iter()
            .filter(|(_, t)| *t == &Point)
            .map(|(pos, _)| pos)
            .collect()
    }
}