use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::common::Position;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::map::board::Board;
use crate::map::Element;
use crate::map::Element::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

/// Resource that knows the positions of fields that are considered walls.
#[derive(Deref, DerefMut)]
pub struct WallPositions(HashSet<Position>);

impl WallPositions {
    fn new<'a, W: IntoIterator<Item=&'a Position>>(wall_iter: W) -> Self {
        WallPositions(wall_iter.into_iter().map(|p| *p).collect())
    }

    pub fn position_is_wall(&self, pos: &Position) -> bool {
        self.0.contains(pos)
    }
}

fn spawn_walls(
    mut commands: Commands,
    board: Res<Board>
) {
    let wall_positions = WallPositions::new(
        board.get_positions_matching(is!(Wall {..} | InvisibleWall)),
    );
    commands.insert_resource(wall_positions);

    for position in board.get_positions_matching(is!(Wall {..})) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::from(position)),
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
                transform: Transform::from_translation(Vec3::from(position)),
                ..Default::default()
            });
    }
}