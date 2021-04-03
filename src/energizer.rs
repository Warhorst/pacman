use bevy::prelude::*;

use crate::constants::ENERGIZER_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_energizer.system());
    }
}

/// An energizer that allows pacman to eat ghosts.
pub struct Energizer;

fn spawn_energizer(commands: &mut Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let point_dimension = Vec2::new(ENERGIZER_DIMENSION, ENERGIZER_DIMENSION);
    for position in board.positions_of_type(FieldType::Energizer) {
        commands
            .spawn(SpriteBundle {
                material: materials.add(Color::rgb(0.9, 0.0, 0.9).into()),
                transform: Transform::from_translation(board.coordinates_of_position(position)),
                sprite: Sprite::new(point_dimension),
                ..Default::default()
            })
            .with(Energizer)
            .with(position.clone());
    }
}