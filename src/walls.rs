use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::animation::{Animation, Animations};
use crate::common::position::Position;
use crate::constants::WALL_DIMENSION;
use crate::is;
use crate::life_cycle::LifeCycle::{LevelTransition, Start};
use crate::map::{Element, Map, Rotation, WallType};

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_walls)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(set_animation_to_blinking)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(set_animation_to_idle)
            )
        ;
    }
}

/// Component to identify a wall
#[derive(Component)]
pub struct Wall;

fn spawn_walls(
    mut commands: Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
) {
    spawn_labyrinth_walls(&mut commands, &map, &asset_server);
    spawn_ghost_house_entrance(&mut commands, &map, &asset_server);
}

fn spawn_labyrinth_walls(commands: &mut Commands, map: &Map, asset_server: &AssetServer) {
    let wall_animations_map = create_animations(asset_server);

    for (position, element) in map.position_element_iter() {
        if let Element::Wall { is_corner, rotation, wall_type } = element {
            let transform = create_transform(position, rotation);
            let animations = wall_animations_map.get(&(*wall_type, *is_corner)).unwrap().clone();
            let custom_size = Some(Vec2::new(WALL_DIMENSION, WALL_DIMENSION));

            commands.spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size,
                        ..default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(animations)
                .insert(Wall)
            ;
        }
    }
}

fn create_transform(position: &Position, rotation: &Rotation) -> Transform {
    let mut transform = Transform::from_translation(Vec3::from(position));
    transform.rotation = rotation.quat_z();
    transform
}

fn create_animations(asset_server: &AssetServer) -> HashMap<(WallType, bool), Animations> {
    [
        (WallType::Outer, true, "textures/walls/outer_wall_corner.png", "textures/walls/outer_wall_corner_blinking.png.sheet"),
        (WallType::Outer, false, "textures/walls/outer_wall.png", "textures/walls/outer_wall_blinking.png.sheet"),
        (WallType::Inner, true, "textures/walls/inner_wall_corner.png", "textures/walls/inner_wall_corner_blinking.png.sheet"),
        (WallType::Inner, false, "textures/walls/inner_wall.png", "textures/walls/inner_wall_blinking.png.sheet"),
        (WallType::Ghost, true, "textures/walls/ghost_house_wall_corner.png", "textures/walls/ghost_house_wall_corner_blinking.png.sheet"),
        (WallType::Ghost, false, "textures/walls/ghost_house_wall.png", "textures/walls/ghost_house_wall_blinking.png.sheet"),
    ]
        .into_iter()
        .map(|(tp, is_corner, idle_path, blinking_path)| ((tp, is_corner), Animations::new(
            [
                ("idle", Animation::from_texture(asset_server.load(idle_path))),
                ("blinking", Animation::from_sprite_sheet(0.5, true, 2, asset_server.load(blinking_path)))
            ]
            , "idle"
        )))
        .collect()
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

fn set_animation_to_blinking(
    mut query: Query<&mut Animations, With<Wall>>
) {
    for mut animations in &mut query {
        animations.change_animation_to("blinking")
    }
}

fn set_animation_to_idle(
    mut query: Query<&mut Animations, With<Wall>>
) {
    for mut animations in &mut query {
        animations.change_animation_to("idle")
    }
}