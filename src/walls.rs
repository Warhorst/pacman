use bevy::prelude::*;
use crate::common::position::Position;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::map::{Element, Map, Rotation, WallType};
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
    spawn_ghost_house_entrance(&mut commands, &map, &asset_server);
}

fn spawn_labyrinth_walls(commands: &mut Commands, map: &Map, asset_server: &AssetServer) {
    for (position, element) in map.position_element_iter() {
        if let Wall { is_corner, rotation, wall_type } = element {
            let transform = create_transform(position, rotation);
            let texture = select_texture(asset_server, *is_corner, wall_type);
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

fn select_texture(asset_server: &AssetServer, is_corner: bool, wall_type: &WallType) -> Handle<Image> {
    match (wall_type, is_corner) {
        (WallType::Outer, true) => asset_server.load("textures/walls/outer_wall_corner.png"),
        (WallType::Outer, false) => asset_server.load("textures/walls/outer_wall.png"),
        (WallType::Inner, true) => asset_server.load("textures/walls/inner_wall_corner.png"),
        (WallType::Inner, false) => asset_server.load("textures/walls/inner_wall.png"),
        (WallType::Ghost, true) => asset_server.load("textures/walls/ghost_house_wall_corner.png"),
        (WallType::Ghost, false) => asset_server.load("textures/walls/ghost_house_wall.png"),
    }
}

fn spawn_ghost_house_entrance(commands: &mut Commands, map: &Map, asset_server: &AssetServer) {
    for position in map.get_positions_matching(is!(Element::GhostHouseEntrance {..})) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/walls/ghost_house_entrance.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::from(position)),
                ..Default::default()
            });
    }
}