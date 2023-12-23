use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_sprite_sheet::{SpriteSheet, SpriteSheets};
use crate::game_assets::animation::{Animation, Animations};
use crate::game::position::Position;
use crate::constants::WALL_DIMENSION;
use crate::game::map::{Element, TileMap, Rotation, WallType, Wall};

#[derive(Component)]
pub struct Labyrinth;

type IsCorner = bool;

pub fn spawn_labyrinth(
    commands: &mut Commands,
    map: &TileMap,
    sprite_sheets: &SpriteSheets,
) -> Entity {
    let wall_animations_map = create_animations(sprite_sheets);
    let walls = &map.position_element_iter()
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
        .collect::<Vec<_>>();

    commands.spawn((
        Name::new("Labyrinth"),
        Labyrinth,
        SpatialBundle::default(),
    )).push_children(walls).id()
}

fn spawn_labyrinth_wall(
    commands: &mut Commands,
    transform: Transform,
    animations: Animations,
    custom_size: Option<Vec2>,
) -> Entity {
    commands.spawn((
        SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size,
                ..default()
            },
            transform,
            ..Default::default()
        },
        animations,
        Name::new("Wall"),
        Wall
    )).id()
}

fn create_transform(position: &Position, rotation: &Rotation) -> Transform {
    let mut transform = Transform::from_translation(position.to_vec(0.0));
    transform.rotation = rotation.quat_z();
    transform
}

fn create_animations(
    sprite_sheets: &SpriteSheets
) -> HashMap<(WallType, IsCorner), Animations> {
    [
        (WallType::Outer, true, "textures/walls/outer_wall_corner"),
        (WallType::Outer, false, "textures/walls/outer_wall"),
        (WallType::Inner, true, "textures/walls/inner_wall_corner"),
        (WallType::Inner, false, "textures/walls/inner_wall"),
    ]
        .into_iter()
        .map(|(tp, is_corner, sheet_path)| ((tp, is_corner), create_wall_animations(sprite_sheets.get_sheet(sheet_path))))
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