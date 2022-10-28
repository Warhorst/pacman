use bevy::prelude::*;
use crate::animation::{Animation, Animations};
use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::FromPositions;
use crate::common::position::Position;
use crate::constants::{BLINKY_Z, CLYDE_Z, INKY_Z, PINKY_Z, WALL_DIMENSION};
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::is;
use crate::map::{Element, Rotation, TileMap};
use crate::map::Rotation::{D0, D180, D270, D90};
use crate::map::walls::Wall;
use crate::sprite_sheet::SpriteSheet;

/// Parent component for everything related to the ghost house
#[derive(Component)]
pub struct GhostHouse;

#[derive(Component)]
pub struct GhostSpawn {
    pub ghost: Ghost,
    pub coordinates: Vec3,
    pub spawn_direction: Direction,
    pub positions: [Position; 2],
}

pub(in crate::map) fn spawn_ghost_house(
    commands: &mut Commands,
    tile_map: &TileMap,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Entity {
    let bottom_left = get_bottom_left(tile_map);
    let rotation = get_rotation(tile_map);
    let spawns = create_spawns(rotation, bottom_left);

    let ghost_house = commands.spawn()
        .insert(Name::new("GhostHouse"))
        .insert(GhostHouse)
        .insert_bundle(SpatialBundle::default())
        .id();

    for spawn in spawns {
        commands.entity(ghost_house).with_children(|parent| {
            parent.spawn()
                .insert(Name::new("GhostSpawn"))
                .insert(spawn);
        });
    }

    spawn_house_walls(commands, ghost_house, bottom_left, rotation, loaded_assets, sprite_sheets);

    ghost_house
}

fn get_bottom_left(tile_map: &TileMap) -> Position {
    tile_map
        .get_positions_matching(is!(Element::GhostHouse {..}))
        .into_iter()
        .fold(
            Position::new(isize::MAX, isize::MAX),
            |acc, pos| Position::new(isize::min(acc.x, pos.x), isize::min(acc.y, pos.y)),
        )
}

fn get_rotation(tile_map: &TileMap) -> Rotation {
    tile_map
        .position_element_iter()
        .into_iter()
        .filter_map(|(_, elem)| match elem {
            Element::GhostHouse { rotation } => Some(*rotation),
            _ => None
        })
        .next()
        .expect("the map should at least contain one ghost house entrance")
}

fn create_spawns(rotation: Rotation, bottom_left: Position) -> [GhostSpawn; 4] {
    [
        create_blinky_spawn(rotation, bottom_left),
        create_pinky_spawn(rotation, bottom_left),
        create_inky_spawn(rotation, bottom_left),
        create_clyde_spawn(rotation, bottom_left),
    ]
}

fn create_blinky_spawn(rotation: Rotation, bottom_left: Position) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (3, 5), (4, 5), BLINKY_Z, Blinky, Left),
        D90 => create_spawn_with_offsets(bottom_left, (5, 3), (5, 4), BLINKY_Z, Blinky, Up),
        D180 => create_spawn_with_offsets(bottom_left, (3, -1), (4, -1), BLINKY_Z, Blinky, Right),
        D270 => create_spawn_with_offsets(bottom_left, (-1, 3), (-1, 4), BLINKY_Z, Blinky, Down),
    }
}

fn create_pinky_spawn(rotation: Rotation, bottom_left: Position) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), PINKY_Z, Pinky, Up),
        D90 => create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), PINKY_Z, Pinky, Right),
        D180 => create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), PINKY_Z, Pinky, Down),
        D270 => create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), PINKY_Z, Pinky, Left),
    }
}

fn create_inky_spawn(rotation: Rotation, bottom_left: Position) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), INKY_Z, Inky, Down),
        D90 => create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), INKY_Z, Inky, Left),
        D180 => create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), INKY_Z, Inky, Up),
        D270 => create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), INKY_Z, Inky, Right),
    }
}

fn create_clyde_spawn(rotation: Rotation, bottom_left: Position) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), CLYDE_Z, Clyde, Down),
        D90 => create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), CLYDE_Z, Clyde, Left),
        D180 => create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), CLYDE_Z, Clyde, Up),
        D270 => create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), CLYDE_Z, Clyde, Right),
    }
}

fn create_spawn_with_offsets(
    bottom_left: Position,
    offsets_0: (isize, isize),
    offsets_1: (isize, isize),
    z: f32,
    ghost: Ghost,
    spawn_direction: Direction,
) -> GhostSpawn {
    let x = bottom_left.x;
    let y = bottom_left.y;
    let positions = [
        Position::new(x + offsets_0.0, y + offsets_0.1),
        Position::new(x + offsets_1.0, y + offsets_1.1),
    ];
    let coordinates = Vec3::from_positions(positions.iter(), z);
    GhostSpawn {
        ghost,
        spawn_direction,
        positions,
        coordinates,
    }
}

fn spawn_house_walls(
    commands: &mut Commands,
    ghost_house: Entity,
    bottom_left: Position,
    rotation: Rotation,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) {
    let top_right = match rotation {
        D0 | D180 => Position::new(bottom_left.x + 7, bottom_left.y + 4),
        _ => Position::new(bottom_left.x + 4, bottom_left.y + 7),
    };

    let corners = spawn_corners(
        commands,
        bottom_left,
        top_right,
        loaded_assets,
        sprite_sheets
    );
    let top = spawn_top(commands, rotation, bottom_left, top_right, loaded_assets, sprite_sheets);
    let bottom = spawn_bottom(commands, rotation, bottom_left, loaded_assets, sprite_sheets);
    let left = spawn_left(commands, rotation, bottom_left, loaded_assets, sprite_sheets);
    let right = spawn_right(commands, rotation, bottom_left, top_right, loaded_assets, sprite_sheets);
    commands.entity(ghost_house).push_children(&corners);
    commands.entity(ghost_house).push_children(&top);
    commands.entity(ghost_house).push_children(&bottom);
    commands.entity(ghost_house).push_children(&left);
    commands.entity(ghost_house).push_children(&right);
}

fn spawn_corners(
    commands: &mut Commands,
    bottom_left: Position,
    top_right: Position,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> [Entity; 4] {
    let sheet = loaded_assets.get_handle("textures/walls/ghost_house_wall_corner");
    [
        spawn_wall(commands, &sheet, D0, Position::new(bottom_left.x, top_right.y), sprite_sheets),
        spawn_wall(commands, &sheet, D90, top_right, sprite_sheets),
        spawn_wall(commands, &sheet, D180, Position::new(top_right.x, bottom_left.y), sprite_sheets),
        spawn_wall(commands, &sheet, D270, bottom_left, sprite_sheets),
    ]
}

fn spawn_top(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Position,
    top_right: Position,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    let x = bottom_left.x + 1;
    let y = top_right.y;
    let sheet = loaded_assets.get_handle("textures/walls/ghost_house_wall");

    match rotation {
       D0 => vec![
           spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
           spawn_entrance(commands, loaded_assets, rotation, Position::new(x + 2, y)),
           spawn_entrance(commands, loaded_assets, rotation, Position::new(x + 3, y)),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 4, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 5, y), sprite_sheets),
       ],
       D180 => vec![
           spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 2, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 3, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 4, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 5, y), sprite_sheets),
       ],
       _ => vec![
           spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
           spawn_wall(commands, &sheet, rotation, Position::new(x + 2, y), sprite_sheets),
       ],
    }
}

fn spawn_bottom(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Position,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    let x = bottom_left.x + 1;
    let y = bottom_left.y;
    let sheet = loaded_assets.get_handle("textures/walls/ghost_house_wall");

    match rotation {
        D180 => vec![
            spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
            spawn_entrance(commands, loaded_assets, rotation, Position::new(x + 2, y)),
            spawn_entrance(commands, loaded_assets, rotation, Position::new(x + 3, y)),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 4, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 5, y), sprite_sheets),
        ],
        D0 => vec![
            spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 2, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 3, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 4, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 5, y), sprite_sheets),
        ],
        _ => vec![
            spawn_wall(commands, &sheet, rotation, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 1, y), sprite_sheets),
            spawn_wall(commands, &sheet, rotation, Position::new(x + 2, y), sprite_sheets),
        ],
    }
}

fn spawn_left(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Position,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    let x = bottom_left.x;
    let y = bottom_left.y + 1;
    let sheet = loaded_assets.get_handle("textures/walls/ghost_house_wall");

    match rotation {
        D270 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_entrance(commands, loaded_assets, D90, Position::new(x, y + 2)),
            spawn_entrance(commands, loaded_assets, D90, Position::new(x, y + 3)),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 4), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 5), sprite_sheets),
        ],
        D90 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 2), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 3), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 4), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 5), sprite_sheets),
        ],
        D0 | D180 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 2), sprite_sheets),
        ],
    }
}

fn spawn_right(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Position,
    top_right: Position,
    loaded_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Vec<Entity> {
    let x = top_right.x;
    let y = bottom_left.y + 1;
    let sheet = loaded_assets.get_handle("textures/walls/ghost_house_wall");

    match rotation {
        D90 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_entrance(commands, loaded_assets, D90, Position::new(x, y + 2)),
            spawn_entrance(commands, loaded_assets, D90, Position::new(x, y + 3)),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 4), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 5), sprite_sheets),
        ],
        D270 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 2), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 3), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 4), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 5), sprite_sheets),
        ],
        D0 | D180 => vec![
            spawn_wall(commands, &sheet, D90, Position::new(x, y), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 1), sprite_sheets),
            spawn_wall(commands, &sheet, D90, Position::new(x, y + 2), sprite_sheets),
        ],
    }
}

fn spawn_wall(
    commands: &mut Commands,
    sheet: &Handle<SpriteSheet>,
    rotation: Rotation,
    position: Position,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Entity {
    let sheet = sprite_sheets.get(sheet).expect("should be there");
    let animations = Animations::new(
        [
            ("idle", Animation::from_texture(sheet.image_at(0))),
            ("blinking", Animation::from_textures(0.5, true, sheet.images_at([0, 1])))
        ]
        , "idle",
    );

    let mut transform = Transform::from_translation(position.to_vec(0.0));
    transform.rotation = rotation.quat_z();

    commands.spawn()
        .insert(Name::new("Wall"))
        .insert(Wall)
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                ..default()
            },
            transform,
            ..default()
        })
        .insert(animations)
        .id()
}

fn spawn_entrance(
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
    rotation: Rotation,
    position: Position,
) -> Entity {
    let mut transform = Transform::from_translation(position.to_vec(0.0));
    transform.rotation = rotation.quat_z();

    commands.spawn()
        .insert(Name::new("Wall"))
        .insert(Wall)
        .insert_bundle(SpriteBundle {
            texture: loaded_assets.get_handle("textures/walls/ghost_house_entrance.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                ..default()
            },
            transform,
            ..default()
        })
        .id()
}

