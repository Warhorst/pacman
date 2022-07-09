use bevy::prelude::*;
use crate::common::position::Position;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::map::{Element, Map, Rotation};
use crate::map::Element::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

fn spawn_walls(
    mut commands: Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
) {
    spawn_labyrinth_walls(&mut commands, &map, &asset_server);
    spawn_ghost_house_entrance(&mut commands, &map);
}

fn spawn_labyrinth_walls(commands: &mut Commands, map: &Map, asset_server: &AssetServer) {
    for (position, element) in map.position_element_iter() {
        if let Wall { is_corner, rotation, .. } = element {
            let transform = create_transform(position, rotation);
            let texture = select_texture(asset_server, *is_corner);
            let custom_size = Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION));

            commands.spawn()
                .insert_bundle(SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size,
                        ..default()
                    },
                    transform,
                    ..Default::default()
                });
        }
    }
}

fn create_transform(position: &Position, rotation: &Rotation) -> Transform {
    let mut transform = Transform::from_translation(Vec3::from(position));
    transform.rotation = rotation.quat_z();
    transform
}

fn select_texture(asset_server: &AssetServer, is_corner: bool) -> Handle<Image> {
    if is_corner {
        asset_server.load("textures/walls/outer_wall_corner.png")
    } else {
        asset_server.load("textures/walls/outer_wall.png")
    }
}

fn spawn_ghost_house_entrance(commands: &mut Commands, map: &Map) {
    for position in map.get_positions_matching(is!(Element::GhostHouseEntrance {..})) {
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