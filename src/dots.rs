use bevy::prelude::*;

use crate::constants::POINT_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType;

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_dots.system());
    }
}

pub struct Dot;

fn spawn_dots(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let point_dimension = Vec2::new(POINT_DIMENSION, POINT_DIMENSION);
    for position in board.positions_of_type(FieldType::Point) {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                sprite: Sprite::new(point_dimension),
                ..Default::default()
            })
            .with(Dot)
            .with(position.clone());
    }
}

