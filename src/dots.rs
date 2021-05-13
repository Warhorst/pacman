use bevy::prelude::*;

use crate::constants::POINT_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType;
use crate::common::Position;
use crate::pacman::Pacman;

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<DotEaten>()
            .add_startup_system(spawn_dots.system())
            .add_system(pacman_eat_dot.system());
    }
}

pub struct Dot;

/// Fired when pacman eats a dot.
pub struct DotEaten;

fn spawn_dots(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let point_dimension = Vec2::new(POINT_DIMENSION, POINT_DIMENSION);
    for position in board.positions_of_type(FieldType::Point) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                sprite: Sprite::new(point_dimension),
                ..Default::default()
            })
            .insert(Dot)
            .insert(position.clone());
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<DotEaten>,
    pacman_positions: Query<&Position, With<Pacman>>,
    dot_positions: Query<(Entity, &Position), With<Dot>>,
) {
    for pacman_pos in pacman_positions.iter() {
        for (entity, dot_pos) in dot_positions.iter() {
            if pacman_pos == dot_pos {
                commands.entity(entity).despawn();
                event_writer.send(DotEaten)
            }
        }
    }
}