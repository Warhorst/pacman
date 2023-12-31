use std::collections::HashMap;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_sprite_sheet::SpriteSheets;
use serde::{Deserialize, Serialize};

use crate::core::prelude::*;

use crate::game::map::ghost_house::spawn_ghost_house;
use crate::game::map::tunnel::{spawn_tunnel_hallways, spawn_tunnels, TunnelPlugin};


#[cfg(test)]
mod map_creator;
pub mod ghost_house;
pub mod tunnel;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TunnelPlugin,
                JsonAssetPlugin::<RawMap>::new(&["map.json"])
            ))
            // .add_systems(OnExit(Setup(CreateSpriteSheets)), spawn_map)
            .add_systems(OnEnter(Game(LevelTransition)), set_animation_to_blinking)
            .add_systems(OnExit(Game(LevelTransition)), set_animation_to_idle)
        ;
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets: Res<SpriteSheets>,
    fields_assets: Res<Assets<RawMap>>,
) {
    let fields = fields_assets.get(&asset_server.load("maps/default.map.json")).expect("the map should be loaded at this point");
    let tile_map = TileMap::new(&fields);

    let map = commands.spawn((
        Name::new("Map"),
        Map {
            width: tile_map.get_width(),
            height: tile_map.get_height(),
        },
        SpatialBundle::default(),
    )).id();

    let children = []
        .into_iter()
        .chain([spawn_dot_spawns(&mut commands, &tile_map)])
        .chain([spawn_energizer_spawns(&mut commands, &tile_map)])
        .chain([spawn_pacman_spawn(&mut commands, &tile_map)])
        .chain([spawn_fruit_spawns(&mut commands, &tile_map)])
        .chain([spawn_ghost_house(&mut commands, &tile_map, &asset_server, &sprite_sheets)])
        .chain(spawn_tunnels(&mut commands, &tile_map))
        .chain(spawn_tunnel_hallways(&mut commands, &tile_map))
        .chain(spawn_ghost_corners(&mut commands, &tile_map))
        .collect::<Vec<_>>();
    commands.entity(map).push_children(&children);
}

fn spawn_pacman_spawn(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Entity {
    let coordinates = Vec3::from_positions(tile_map.get_positions_matching(is!(Element::PacManSpawn)), PACMAN_Z);
    commands.spawn((
        Name::new("PacmanSpawn"),
        PacmanSpawn(coordinates)
    )).id()
}

fn spawn_dot_spawns(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Entity {
    let dot_spawns = commands.spawn((
        Name::new("DotSpawns"),
        DotSpawns
    )).id();

    tile_map.get_positions_matching(is!(Element::DotSpawn))
        .into_iter()
        .for_each(|pos| {
            commands
                .entity(dot_spawns)
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("DotSpawn"),
                        DotSpawn(pos.to_vec3(DOT_Z))
                    ));
                });
        });

    dot_spawns
}

fn spawn_energizer_spawns(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Entity {
    let energizer_spawns = commands.spawn((
        Name::new("EnergizerSpawns"),
        EnergizerSpawns
    )).id();

    tile_map.get_positions_matching(is!(Element::EnergizerSpawn))
        .into_iter()
        .for_each(|pos| {
            commands.entity(energizer_spawns)
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("EnergizerSpawn"),
                        EnergizerSpawn(pos.to_vec3(ENERGIZER_Z))
                    ));
                });
        });

    energizer_spawns
}

fn spawn_fruit_spawns(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Entity {
    let coordinates = Vec3::from_positions(tile_map.get_positions_matching(is!(Element::FruitSpawn)), FRUIT_Z);

    commands.spawn((
        Name::new("FruitSpawn"),
        FruitSpawn(coordinates)
    )).id()
}

fn spawn_ghost_corners(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> [Entity; 4] {
    let mut spawn_corner = |ghost, position| commands.spawn((
        Name::new("GhostCorner"),
        GhostCorner { ghost, position }
    )).id();

    [
        spawn_corner(Blinky, tile_map.blinky_corner),
        spawn_corner(Pinky, tile_map.pinky_corner),
        spawn_corner(Inky, tile_map.inky_corner),
        spawn_corner(Clyde, tile_map.clyde_corner),
    ]
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

#[derive(Clone, Asset, Serialize, Deserialize, bevy::reflect::TypeUuid, TypePath)]
#[uuid = "a09992c9-9567-42d9-a0ac-c998756e4073"]
pub struct RawMap {
    pub blinky_corner: Pos,
    pub pinky_corner: Pos,
    pub inky_corner: Pos,
    pub clyde_corner: Pos,
    pub fields: Vec<Field>,
}

/// Resource that knows the spawn locations of every entity, based on an external map file.
///
/// The map should only be used to spawn or respawn entities into the world.
pub struct TileMap {
    pub blinky_corner: Pos,
    pub pinky_corner: Pos,
    pub inky_corner: Pos,
    pub clyde_corner: Pos,
    elements_map: HashMap<Pos, Element>,
}

impl TileMap {
    fn new(raw_map: &RawMap) -> Self {
        TileMap {
            blinky_corner: raw_map.blinky_corner,
            pinky_corner: raw_map.pinky_corner,
            inky_corner: raw_map.inky_corner,
            clyde_corner: raw_map.clyde_corner,
            elements_map: raw_map.fields
                .clone()
                .into_iter()
                .map(|f| (f.position, f.element))
                .collect(),
        }
    }

    pub(crate) fn get_width(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.x())
            .collect::<HashSet<_>>()
            .len()
    }

    pub(crate) fn get_height(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.y())
            .collect::<HashSet<_>>()
            .len()
    }

    /// Return an iterator over all positions matching the given element filter.
    pub fn get_positions_matching(&self, filter: impl Fn(&Element) -> bool) -> impl IntoIterator<Item=&Pos> {
        self.elements_map.iter()
            .filter(move |(_, elem)| (filter)(elem))
            .map(|(pos, _)| pos)
    }

    /// Return an iterator over all positions and elements.
    pub fn position_element_iter(&self) -> impl IntoIterator<Item=(&Pos, &Element)> {
        self.elements_map.iter()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    pub position: Pos,
    pub element: Element,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Wall {
        wall_type: WallType,
        rotation: Rotation,
        is_corner: bool,
    },
    GhostHouseEntrance {
        rotation: Rotation
    },
    GhostHouse {
        rotation: Rotation
    },
    PacManSpawn,
    DotSpawn,
    EnergizerSpawn,
    FruitSpawn,
    Tunnel {
        index: usize,
        opening_direction: Dir,
    },
    TunnelHallway,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum WallType {
    Outer,
    Inner,
}