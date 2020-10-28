use std::cmp::Ordering;
use std::collections::HashMap;

use bevy::ecs::Commands;
use bevy::prelude::*;

use FieldType::{Free, Wall};

use crate::common::Position;

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
    board_root: Vec2
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
        let fields = Self::fields_from_field_type_vec(Self::create_board());
        let board_root = Vec2::new(-100.0, -50.0);
        let field_size = Vec2::new(30.0, 30.0);
        Board {
            fields,
            field_dimension: field_size,
            board_root
        }
    }

    fn create_board() -> FieldTypeVec {
        vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Free, Free, Free, Free, Free, Free, Wall],
            vec![Wall, Free, Free, Free, Free, Free, Free, Wall],
            vec![Wall, Free, Free, Free, Free, Free, Free, Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall]
        ]
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

    pub fn window_coordinates(&self, position: &Position, dimension: &Vec2) -> Vec3 {
        let padding_in_field = self.calc_padding_in_field(dimension);
        let x = self.board_root.x() + (position.x() as f32) * self.field_dimension.x() + padding_in_field.x();
        let y = self.board_root.y() + (position.y() as f32) * self.field_dimension.y() + padding_in_field.y();
        Vec3::new(x, y, 0.0)
    }

    fn calc_padding_in_field(&self, dimension: &Vec2) -> Vec2 {
        Vec2::new(Self::calc_padding(dimension.x(), self.field_dimension.x()), Self::calc_padding(dimension.y(), self.field_dimension.y()))
    }

    fn calc_padding(size_other: f32, size_field: f32) -> f32 {
        match size_other < size_field {
            true => (size_field - size_other) / 2.0,
            false => 0.0
        }
    }

    pub fn field_size(self) -> Vec2 {
        self.field_dimension
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
            transform: Transform::from_translation(board.window_coordinates(field.position, &board.field_dimension)),
            sprite: Sprite::new(board.field_dimension),
            ..Default::default()
        });
    }
}