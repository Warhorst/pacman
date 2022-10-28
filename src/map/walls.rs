use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::animation::{Animation, Animations};
use crate::common::position::Position;
use crate::constants::WALL_DIMENSION;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::is;
use crate::life_cycle::LifeCycle::LevelTransition;
use crate::map::{Element, TileMap, Rotation, WallType};
use crate::sprite_sheet::SpriteSheet;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
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

pub fn spawn_walls(
    commands: &mut Commands,
    map: &TileMap,
    game_asset_handles: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    spawn_labyrinth_walls(commands, &map, &game_asset_handles, &sprite_sheets)
        .into_iter()
        .chain(spawn_ghost_house_entrance(commands, &map, &game_asset_handles).into_iter())
        .collect()
}

fn spawn_labyrinth_walls(
    commands: &mut Commands,
    map: &TileMap,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    let wall_animations_map = create_animations(loaded_assets, sprite_sheets);
    map.position_element_iter()
        .into_iter()
        .filter_map(|(position, element)| match element {
            Element::Wall { is_corner, rotation, wall_type } => Some((position, is_corner, rotation, wall_type)),
            _ => None
        })
        .map(|(position, is_corner, rotation, wall_type)| {
            let transform = create_transform(position, rotation);
            let animations = wall_animations_map.get(&(*wall_type, *is_corner)).unwrap().clone();
            let custom_size = Some(Vec2::splat(WALL_DIMENSION));
            spawn_labyrinth_wall(commands, transform, animations, custom_size)
        })
        .collect()
}

fn spawn_labyrinth_wall(
    commands: &mut Commands,
    transform: Transform,
    animations: Animations,
    custom_size: Option<Vec2>,
) -> Entity {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size,
                ..default()
            },
            transform,
            ..Default::default()
        })
        .insert(animations)
        .insert(Wall)
        .insert(Name::new("Wall"))
        .id()
}

fn create_transform(position: &Position, rotation: &Rotation) -> Transform {
    let mut transform = Transform::from_translation(position.to_vec(0.0));
    transform.rotation = rotation.quat_z();
    transform
}

fn create_animations(loaded_assets: &LoadedAssets, sprite_sheets: &Assets<SpriteSheet>) -> HashMap<(WallType, bool), Animations> {
    [
        (WallType::Outer, true, loaded_assets.get_handle("textures/walls/outer_wall_corner")),
        (WallType::Outer, false, loaded_assets.get_handle("textures/walls/outer_wall")),
        (WallType::Inner, true, loaded_assets.get_handle("textures/walls/inner_wall_corner")),
        (WallType::Inner, false, loaded_assets.get_handle("textures/walls/inner_wall")),
    ]
        .into_iter()
        .map(|(tp, is_corner, sheet_handle)| ((tp, is_corner), create_wall_animations(sprite_sheets.get(&sheet_handle).expect("sheet should be present"))))
        .collect()
}

fn create_wall_animations(sheet: &SpriteSheet) -> Animations {
    Animations::new(
        [
            ("idle", Animation::from_texture(sheet.image_at(0))),
            ("blinking", Animation::from_textures(0.5, true, sheet.images_at([0, 1])))
        ]
        , "idle",
    )
}

fn spawn_ghost_house_entrance(
    commands: &mut Commands,
    map: &TileMap,
    loaded_assets: &LoadedAssets,
) -> Vec<Entity> {
    map.get_positions_matching(is!(Element::GhostHouseEntrance {..}))
        .into_iter()
        .map(|position| commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: loaded_assets.get_handle("textures/walls/ghost_house_entrance.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(position.to_vec(0.0)),
                ..Default::default()
            })
            .insert(Name::new("Wall"))
            .id())
        .collect()
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