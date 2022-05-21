use bevy::prelude::*;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::map::board::Board;
use crate::map::Element;
use crate::map::Element::Wall;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

fn spawn_walls(
    mut commands: Commands,
    board: Res<Board>
) {
    for position in board.get_positions_matching(is!(Wall {..})) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(Board::coordinates_of_position(position)),
                ..Default::default()
            });
    }

    for position in board.get_positions_matching(is!(Element::GhostHouseEntrance {..})) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(Board::coordinates_of_position(position)),
                ..Default::default()
            });
    }
}