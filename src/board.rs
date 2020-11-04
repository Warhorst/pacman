use std::collections::HashMap;

use bevy::ecs::Commands;
use bevy::prelude::*;

use FieldType::{Free, Wall};

use crate::common::{Direction::*, Position};
use crate::common;

pub type Fields<'a> = Vec<Field<'a>>;
type FieldTypeVec = Vec<Vec<FieldType>>;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Board::new())
            .add_startup_system(create_board.system());
    }
}

pub struct Board {
    fields: HashMap<Position, FieldType>,
    field_dimension: Vec2,
    board_root: Vec2,
    max_position: Position,
}

pub struct Field<'a> {
    position: &'a Position,
    field_type: &'a FieldType,
}

#[derive(Copy, Clone)]
enum FieldType {
    Free,
    Wall,
}

impl Board {
    fn new() -> Self {
        let board = Self::create_board();
        let max_position = Self::get_max_position(&board);
        let fields = Self::fields_from_field_type_vec(board);
        let board_root = Vec2::new(0.0, 0.0);
        let field_size = Vec2::new(30.0, 30.0);
        Board {
            fields,
            field_dimension: field_size,
            board_root,
            max_position,
        }
    }

    fn create_board() -> FieldTypeVec {
        vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Free, Free, Free, Free, Free, Free, Wall],
            vec![Wall, Free, Wall, Free, Wall, Wall, Free, Wall],
            vec![Wall, Free, Wall, Free, Wall, Wall, Free, Wall],
            vec![Wall, Free, Wall, Free, Wall, Wall, Free, Wall],
            vec![Wall, Free, Wall, Free, Wall, Wall, Free, Wall],
            vec![Wall, Free, Wall, Free, Wall, Wall, Free, Wall],
            vec![Wall, Free, Free, Free, Free, Free, Free, Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall]
        ]
    }

    fn get_max_position(board: &Vec<Vec<FieldType>>) -> Position {
        let x = board.len() - 1;
        let y = match board.get(0) {
            Some(vec) => vec.len(),
            None => 0
        };
        Position::new(x, y)
    }

    fn fields_from_field_type_vec(fields: FieldTypeVec) -> HashMap<Position, FieldType> {
        let mut result = HashMap::new();
        let width = fields.len();
        let height = match fields.get(0) {
            Some(vec) => vec.len(),
            None => 1
        };
        for i in 0..width {
            for j in 0..height {
                result.insert(Position::new(i, j), fields[i][j]);
            }
        }
        result
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
            _ => false,
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
            y if y == self.max_position.y() => None,
            y => Some(Position::new(position.x(), y + 1))
        }
    }

    fn position_down_of(&self, position: &Position) -> Option<Position> {
        match position.y() {
            0 => None,
            y => Some(Position::new(position.x(), y - 1))
        }
    }

    fn position_left_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            0 => None,
            x => Some(Position::new(x - 1, position.y()))
        }
    }

    fn position_right_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            x if x > self.max_position.x() => None,
            x => Some(Position::new(x + 1, position.y()))
        }
    }

    fn position_is_obstacle(&self, position: &Position) -> bool {
        let field_type = self.fields.get(position).unwrap();
        match field_type {
            Free => false,
            Wall => true
        }
    }

    fn coordinates_in_field_center(&self, coordinates: &Vec3, dimension: &Vec2, position: &Position, direction: &common::Direction) -> bool {
        let position_coordinates = self.window_coordinates(position);

        let x_start = position_coordinates.x() - ((self.field_dimension.x() - dimension.x()) / 2.0);
        let x_end = position_coordinates.x() + ((self.field_dimension.x() - dimension.x()) / 2.0);
        let y_start = position_coordinates.y() - ((self.field_dimension.y() - dimension.y()) / 2.0);
        let y_end = position_coordinates.y() + ((self.field_dimension.y() - dimension.y()) / 2.0);

        match direction {
            Left | Right => coordinates.y() >= y_start && coordinates.y() <= y_end,
            Up | Down => coordinates.x() >= x_start && coordinates.x() <= x_end
        }
    }
}

fn create_board(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for field in board.fields() {
        let color_material = match field.field_type {
            Free => Color::rgb(0.0, 0.0, 0.0).into(),
            Wall => Color::rgb(0.0, 0.0, 1.0).into()
        };

        commands.spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.window_coordinates(field.position)),
            sprite: Sprite::new(board.field_dimension),
            ..Default::default()
        });
    }
}