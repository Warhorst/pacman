use bevy::prelude::*;
use bevy_sprite_sheet::{SpriteSheet, SpriteSheets};
use crate::game::map::{Element, TileMap};

use crate::prelude::*;

pub(crate) fn spawn_ghost_house(
    commands: &mut Commands,
    tile_map: &TileMap,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) -> Entity {
    let bottom_left = get_bottom_left(tile_map);
    let rotation = get_rotation(tile_map);
    let spawns = create_spawns(rotation, bottom_left);

    let ghost_house = commands.spawn((
        Name::new("GhostHouse"),
        GhostHouse,
        SpatialBundle::default()
    )).id();

    for spawn in spawns {
        commands.entity(ghost_house).with_children(|parent| {
            parent.spawn((
                Name::new("GhostSpawn"),
                spawn
            ));
        });
    }

    spawn_house_walls(commands, ghost_house, bottom_left, rotation, asset_server, sprite_sheets);

    ghost_house
}

fn get_bottom_left(tile_map: &TileMap) -> Pos {
    tile_map
        .get_positions_matching(is!(Element::GhostHouse {..}))
        .into_iter()
        .fold(
            Pos::new(isize::MAX, isize::MAX),
            |acc, pos| Pos::new(isize::min(acc.x(), pos.x()), isize::min(acc.y(), pos.y())),
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

/// TODO: wrong. Pinky spawns looking down, Inky and Clyde looking up
fn create_spawns(rotation: Rotation, bottom_left: Pos) -> [GhostSpawn; 4] {
    [
        create_blinky_spawn(rotation, bottom_left),
        create_pinky_spawn(rotation, bottom_left),
        create_inky_spawn(rotation, bottom_left),
        create_clyde_spawn(rotation, bottom_left),
    ]
}

fn create_blinky_spawn(rotation: Rotation, bottom_left: Pos) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (3, 5), (4, 5), BLINKY_Z, Blinky, Left),
        D90 => create_spawn_with_offsets(bottom_left, (5, 3), (5, 4), BLINKY_Z, Blinky, Up),
        D180 => create_spawn_with_offsets(bottom_left, (3, -1), (4, -1), BLINKY_Z, Blinky, Right),
        D270 => create_spawn_with_offsets(bottom_left, (-1, 3), (-1, 4), BLINKY_Z, Blinky, Down),
    }
}

fn create_pinky_spawn(rotation: Rotation, bottom_left: Pos) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), PINKY_Z, Pinky, Up),
        D90 => create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), PINKY_Z, Pinky, Right),
        D180 => create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), PINKY_Z, Pinky, Down),
        D270 => create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), PINKY_Z, Pinky, Left),
    }
}

fn create_inky_spawn(rotation: Rotation, bottom_left: Pos) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), INKY_Z, Inky, Down),
        D90 => create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), INKY_Z, Inky, Left),
        D180 => create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), INKY_Z, Inky, Up),
        D270 => create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), INKY_Z, Inky, Right),
    }
}

fn create_clyde_spawn(rotation: Rotation, bottom_left: Pos) -> GhostSpawn {
    match rotation {
        D0 => create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), CLYDE_Z, Clyde, Down),
        D90 => create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), CLYDE_Z, Clyde, Left),
        D180 => create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), CLYDE_Z, Clyde, Up),
        D270 => create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), CLYDE_Z, Clyde, Right),
    }
}

fn create_spawn_with_offsets(
    bottom_left: Pos,
    offsets_0: (isize, isize),
    offsets_1: (isize, isize),
    z: f32,
    ghost: Ghost,
    spawn_direction: Dir,
) -> GhostSpawn {
    let x = bottom_left.x();
    let y = bottom_left.y();
    let positions = [
        Pos::new(x + offsets_0.0, y + offsets_0.1),
        Pos::new(x + offsets_1.0, y + offsets_1.1),
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
    bottom_left: Pos,
    rotation: Rotation,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) {
    let top_right = match rotation {
        D0 | D180 => Pos::new(bottom_left.x() + 7, bottom_left.y() + 4),
        _ => Pos::new(bottom_left.x() + 4, bottom_left.y() + 7),
    };

    let corners = spawn_corners(
        commands,
        bottom_left,
        top_right,
        sprite_sheets,
    );
    let top = spawn_top(commands, rotation, bottom_left, top_right, asset_server, sprite_sheets);
    let bottom = spawn_bottom(commands, rotation, bottom_left, asset_server, sprite_sheets);
    let left = spawn_left(commands, rotation, bottom_left, asset_server, sprite_sheets);
    let right = spawn_right(commands, rotation, bottom_left, top_right, asset_server, sprite_sheets);
    commands.entity(ghost_house).push_children(&corners);
    commands.entity(ghost_house).push_children(&top);
    commands.entity(ghost_house).push_children(&bottom);
    commands.entity(ghost_house).push_children(&left);
    commands.entity(ghost_house).push_children(&right);
}

fn spawn_corners(
    commands: &mut Commands,
    bottom_left: Pos,
    top_right: Pos,
    sprite_sheets: &SpriteSheets,
) -> [Entity; 4] {
    let sheet = sprite_sheets.get_sheet("textures/walls/ghost_house_wall_corner");
    [
        spawn_wall(commands, &sheet, D0, Pos::new(bottom_left.x(), top_right.y())),
        spawn_wall(commands, &sheet, D90, top_right),
        spawn_wall(commands, &sheet, D180, Pos::new(top_right.x(), bottom_left.y())),
        spawn_wall(commands, &sheet, D270, bottom_left),
    ]
}

fn spawn_top(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Pos,
    top_right: Pos,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) -> Vec<Entity> {
    let x = bottom_left.x() + 1;
    let y = top_right.y();
    let sheet = sprite_sheets.get_sheet("textures/walls/ghost_house_wall");

    match rotation {
        D0 => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_entrance(commands, asset_server, rotation, Pos::new(x + 2, y)),
            spawn_entrance(commands, asset_server, rotation, Pos::new(x + 3, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 4, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 5, y)),
        ],
        D180 => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 2, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 3, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 4, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 5, y)),
        ],
        _ => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 2, y)),
        ],
    }
}

fn spawn_bottom(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Pos,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) -> Vec<Entity> {
    let x = bottom_left.x() + 1;
    let y = bottom_left.y();
    let sheet = sprite_sheets.get_sheet("textures/walls/ghost_house_wall");

    match rotation {
        D180 => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_entrance(commands, asset_server, rotation, Pos::new(x + 2, y)),
            spawn_entrance(commands, asset_server, rotation, Pos::new(x + 3, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 4, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 5, y)),
        ],
        D0 => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 2, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 3, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 4, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 5, y)),
        ],
        _ => vec![
            spawn_wall(commands, &sheet, rotation, Pos::new(x, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 1, y)),
            spawn_wall(commands, &sheet, rotation, Pos::new(x + 2, y)),
        ],
    }
}

fn spawn_left(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Pos,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) -> Vec<Entity> {
    let x = bottom_left.x();
    let y = bottom_left.y() + 1;
    let sheet = sprite_sheets.get_sheet("textures/walls/ghost_house_wall");

    match rotation {
        D270 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_entrance(commands, asset_server, D90, Pos::new(x, y + 2)),
            spawn_entrance(commands, asset_server, D90, Pos::new(x, y + 3)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 4)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 5)),
        ],
        D90 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 2)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 3)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 4)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 5)),
        ],
        D0 | D180 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 2)),
        ],
    }
}

fn spawn_right(
    commands: &mut Commands,
    rotation: Rotation,
    bottom_left: Pos,
    top_right: Pos,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
) -> Vec<Entity> {
    let x = top_right.x();
    let y = bottom_left.y() + 1;
    let sheet = sprite_sheets.get_sheet("textures/walls/ghost_house_wall");

    match rotation {
        D90 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_entrance(commands, asset_server, D90, Pos::new(x, y + 2)),
            spawn_entrance(commands, asset_server, D90, Pos::new(x, y + 3)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 4)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 5)),
        ],
        D270 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 2)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 3)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 4)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 5)),
        ],
        D0 | D180 => vec![
            spawn_wall(commands, sheet, D90, Pos::new(x, y)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 1)),
            spawn_wall(commands, sheet, D90, Pos::new(x, y + 2)),
        ],
    }
}

fn spawn_wall(
    commands: &mut Commands,
    sheet: &SpriteSheet,
    rotation: Rotation,
    position: Pos,
) -> Entity {
    let animations = Animations::new(
        [
            ("idle", Animation::from_texture(sheet.image_at(0))),
            ("blinking", Animation::from_textures(0.5, true, sheet.images_at([0, 1])))
        ]
        , "idle",
    );

    let mut transform = Transform::from_translation(position.to_vec3(0.0));
    transform.rotation = rotation.quat_z();

    commands.spawn((
        Name::new("Wall"),
        Wall,
        SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                ..default()
            },
            transform,
            ..default()
        },
        animations
    )).id()
}

fn spawn_entrance(
    commands: &mut Commands,
    asset_server: &AssetServer,
    rotation: Rotation,
    position: Pos,
) -> Entity {
    let mut transform = Transform::from_translation(position.to_vec3(0.0));
    transform.rotation = rotation.quat_z();

    commands.spawn((
        Name::new("Wall"),
        Wall,
        SpriteBundle {
            texture: asset_server.load("textures/walls/ghost_house_entrance.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(WALL_DIMENSION)),
                ..default()
            },
            transform,
            ..default()
        }
    )).id()
}

