use std::fs::File;

use bevy::ecs::Commands;
use bevy::prelude::*;

use crate::common::{Direction::*, Position};
use crate::common;
use crate::map::{FieldType, PositionTypeMap};
use crate::map::FieldType::*;
use crate::map::pacmap::PacMap;

pub type Fields<'a> = Vec<Field<'a>>;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Board::new())
            .add_startup_system(create_board.system());
    }
}

pub struct Board {
    fields: PositionTypeMap,
    width: usize,
    height: usize,
    field_dimension: Vec2,
    board_root: Vec2,
}

pub struct Field<'a> {
    position: &'a Position,
    field_type: &'a FieldType,
}

impl Board {
    fn new() -> Self {
        let pacmap = PacMap::from_read(File::open("maps/default.pacmap").unwrap());
        let board_root = Vec2::new(0.0, 0.0);
        let field_size = Vec2::new(30.0, 30.0);
        Board {
            width: pacmap.width,
            height: pacmap.height,
            fields: pacmap.into_position_type_map(),
            field_dimension: field_size,
            board_root,
        }
    }

    pub fn fields(&self) -> Fields {
        self.fields.iter()
            .map(|(position, field_type)| Field { position, field_type })
            .collect()
    }

    pub fn window_coordinates(&self, position: &Position) -> Vec3 {
        let x = self.board_root.x() + (position.x() as f32) * self.field_dimension.x();
        let y = self.board_root.y() + (position.y() as f32) * self.field_dimension.y();
        Vec3::new(x, y, 0.0)
    }

    pub fn calculate_position(&self, coordinates: &Vec3) -> Position {
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
        let position_coordinates = self.window_coordinates(position);

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
        let left_tunnels: Vec<&Position> = self.fields.iter()
            .filter(|(_, field_type)| *field_type == &LeftTunnel)
            .map(|(position, _)| position)
            .collect();
        left_tunnels.get(0).expect("The board should contain one left tunnel")
    }

    pub fn get_right_tunnel_position(&self) -> &Position {
        let left_tunnels: Vec<&Position> = self.fields.iter()
            .filter(|(_, field_type)| *field_type == &RightTunnel)
            .map(|(position, _)| position)
            .collect();
        left_tunnels.get(0).expect("The board should contain one right tunnel")
    }
}

fn create_board(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for field in board.fields() {
        let color_material = match field.field_type {
            Free => Color::rgb(0.0, 0.0, 0.0).into(),
            Wall => Color::rgb(0.0, 0.0, 1.0).into(),
            LeftTunnel | RightTunnel => Color::rgb(211.0, 211.0, 211.0).into()
        };

        commands.spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.window_coordinates(field.position)),
            sprite: Sprite::new(board.field_dimension),
            ..Default::default()
        });
    }
}