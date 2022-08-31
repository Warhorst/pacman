use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::animation::{Animation, Animations};
use crate::board_dimensions::BoardDimensions;
use crate::common::position::Position;
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::GHOST_HOUSE_ENTRANCE;
use crate::game_assets::keys::sprite_sheets::*;
use crate::is;
use crate::life_cycle::LifeCycle::{LevelTransition, Start};
use crate::map::{Element, Map, Rotation, WallType};
use crate::sprite_sheet::SpriteSheet;

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
    board_dimensions: Res<BoardDimensions>,
    game_asset_handles: Res<GameAssetHandles>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
) {
    spawn_labyrinth_walls(&mut commands, &map, &board_dimensions, &game_asset_handles, &sprite_sheets);
    spawn_ghost_house_entrance(&mut commands, &map, &board_dimensions, &game_asset_handles);
}

fn spawn_labyrinth_walls(
    commands: &mut Commands,
    map: &Map,
    dimensions: &BoardDimensions,
    game_assets: &GameAssetHandles,
    sprite_sheets: &Assets<SpriteSheet>,
) {
    let wall_animations_map = create_animations(game_assets, sprite_sheets);

    for (position, element) in map.position_element_iter() {
        if let Element::Wall { is_corner, rotation, wall_type } = element {
            let transform = create_transform(position, dimensions, rotation);
            let animations = wall_animations_map.get(&(*wall_type, *is_corner)).unwrap().clone();
            let custom_size = Some(Vec2::new(dimensions.wall(), dimensions.wall()));

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
            ;
        }
    }
}

fn create_transform(position: &Position, dimensions: &BoardDimensions, rotation: &Rotation) -> Transform {
    let mut transform = dimensions.pos_to_trans(position, 0.0);
    transform.rotation = rotation.quat_z();
    transform
}

fn create_animations(game_assets: &GameAssetHandles, sprite_sheets: &Assets<SpriteSheet>) -> HashMap<(WallType, bool), Animations> {
    [
        (WallType::Outer, true, game_assets.get_handle(OUTER_WALL_CORNER)),
        (WallType::Outer, false, game_assets.get_handle(OUTER_WALL)),
        (WallType::Inner, true, game_assets.get_handle(INNER_WALL_CORNER)),
        (WallType::Inner, false, game_assets.get_handle(INNER_WALL)),
        (WallType::Ghost, true, game_assets.get_handle(GHOST_WALL_CORNER)),
        (WallType::Ghost, false, game_assets.get_handle(GHOST_WALL)),
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

fn spawn_ghost_house_entrance(commands: &mut Commands, map: &Map, dimensions: &BoardDimensions, game_assets: &GameAssetHandles) {
    for position in map.get_positions_matching(is!(Element::GhostHouseEntrance {..})) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: game_assets.get_handle(GHOST_HOUSE_ENTRANCE),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(dimensions.field(), dimensions.field())),
                    ..default()
                },
                transform: dimensions.pos_to_trans(position, 0.0),
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