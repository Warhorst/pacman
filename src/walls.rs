use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::common::Position;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::map::board::Board;
use crate::map::{Element, Rotation, WallType};
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
    board: Res<Board>,
    asset_server: Res<AssetServer>,
) {
    let wall_positions = WallPositions::new(
        board.get_positions_matching(is!(Wall {..} | InvisibleWall)),
    );
    commands.insert_resource(wall_positions);

    for (position, element) in board.position_element_iter() {
        // TODO refactor
        match element {
            Wall { wall_type, is_corner, rotation} => match wall_type {
                WallType::Outer => {
                    let mut transform = Transform::from_translation(Vec3::from(position));

                    transform.rotation = match rotation {
                        Rotation::D0 => Quat::from_rotation_z(PI * 0.0),
                        Rotation::D90 => Quat::from_rotation_z(PI * 1.5),
                        Rotation::D180 => Quat::from_rotation_z(PI),
                        Rotation::D270 => Quat::from_rotation_z(PI * 0.5),
                    };

                    if *is_corner {
                        commands.spawn()
                            .insert_bundle(SpriteBundle {
                                texture: asset_server.load("textures/walls/outer_wall_corner.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                                    ..default()
                                },
                                transform,
                                ..Default::default()
                            });
                    } else {
                        commands.spawn()
                            .insert_bundle(SpriteBundle {
                                texture: asset_server.load("textures/walls/outer_wall.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                                    ..default()
                                },
                                transform,
                                ..Default::default()
                            });
                    }

                },
                _ => {
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
            },
            _ => {
                // spawn nothing
            }
        }
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