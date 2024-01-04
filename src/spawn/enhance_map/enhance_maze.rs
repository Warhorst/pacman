use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_sprite_sheet::{SpriteSheet, SpriteSheets};
use crate::core::prelude::*;

pub(super) struct EnhanceMazePlugin;

impl Plugin for EnhanceMazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Spawn(EnhanceMap)),
                enhance_maze
            )
        ;
    }
}

type IsCorner = bool;

fn enhance_maze(
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheets>,
    mazes: Query<Entity, With<Maze>>,
    walls: Query<(Entity, &Tiles, &WallStyle), With<Wall>>,
) {
    let wall_animations_map = create_animations(&sprite_sheets);

    commands.entity(mazes.single()).insert(SpatialBundle::default());

    for (entity, tiles, style) in &walls {
        let transform = create_transform(tiles, &style.rotation);
        let animations = wall_animations_map.get(&(style.wall_type, style.is_corner)).unwrap().clone();

        commands
            .entity(entity)
            .insert((
                SpriteBundle {
                    texture: animations.current().texture(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                        ..default()
                    },
                    transform,
                    ..Default::default()
                },
                animations,
            ));
    }
}

fn create_animations(
    sprite_sheets: &SpriteSheets
) -> HashMap<(WallType, IsCorner), Animations> {
    [
        (Outer, true, "textures/walls/outer_wall_corner"),
        (Outer, false, "textures/walls/outer_wall"),
        (Inner, true, "textures/walls/inner_wall_corner"),
        (Inner, false, "textures/walls/inner_wall"),
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

fn create_transform(tiles: &Tiles, rotation: &Rotation) -> Transform {
    let mut transform = Transform::from_translation(tiles.to_vec3(0.0));
    transform.rotation = rotation.quat_z();
    transform
}