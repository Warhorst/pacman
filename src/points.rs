use bevy::prelude::*;

use crate::constants::POINT_DIMENSION;
use crate::map::board::Board;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_points.system());
    }
}

pub struct Point;

fn spawn_points(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let point_dimension = Vec2::new(POINT_DIMENSION, POINT_DIMENSION);
    for position in board.get_point_positions() {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                sprite: Sprite::new(point_dimension),
                ..Default::default()
            })
            .with(Point)
            .with(position.clone());
    }
}

