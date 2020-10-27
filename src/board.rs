use std::collections::HashMap;

use bevy::ecs::Commands;
use bevy::prelude::*;

use FieldType::{Free, Wall};

type FieldTypeVec = Vec<Vec<FieldType>>;
type Fields<'a> = Vec<Field<'a>>;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_board.system());
    }
}

struct Board {
    fields: HashMap<Position, FieldType>,
    pub width: usize,
    pub height: usize,
    pub field_size: Vec2,
}

struct Field<'a> {
    position: &'a Position,
    field_type: &'a FieldType,
}

#[derive(Hash, Eq, PartialEq)]
struct Position(usize, usize);

#[derive(Copy, Clone)]
enum FieldType {
    Free,
    Wall,
}

impl Board {
    fn new() -> Self {
        let field_type_vec = Self::create_board();
        let width = field_type_vec.len();
        let height = match field_type_vec.get(0) {
            Some(vec) => vec.len(),
            None => 1
        };
        let fields = Self::fields_from_field_type_vec(field_type_vec, width, height);
        let field_size = Vec2::new(30.0, 30.0);
        Board {
            fields,
            width,
            height,
            field_size,
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

    fn fields_from_field_type_vec(fields: FieldTypeVec, width: usize, height: usize) -> HashMap<Position, FieldType> {
        let mut result = HashMap::new();
        for i in 0..width {
            for j in 0..height {
                result.insert(Position(i, j), fields[i][j]);
            }
        }
        result
    }

    fn fields(&self) -> Fields {
        self.fields.iter()
            .map(|(position, field_type)| Field { position, field_type })
            .collect()
    }
}

fn create_board(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let board = Board::new();
    for field in board.fields() {
        let color_material = match field.field_type {
            Free => Color::rgb(0.0, 0.0, 0.0).into(),
            Wall => Color::rgb(0.0, 0.0, 1.0).into()
        };
        let x = field.position.0 as f32;
        let y = field.position.1 as f32;
        let field_width = board.field_size.x();
        let field_height = board.field_size.y();
        let translation = Vec3::new(x * field_width, y * field_height, 0.0);

        commands.spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(translation),
            sprite: Sprite::new(board.field_size),
            ..Default::default()
        });
    }
}