use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_sprite_sheet::SpriteSheets;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::constants::{DOT_Z, ENERGIZER_Z, FRUIT_Z, PACMAN_Z};
use crate::game::ghosts::Ghost;
use crate::game::ghosts::Ghost::{Blinky, Clyde, Inky, Pinky};
use crate::game::helper::FromPositions;
use crate::game::map::ghost_house::spawn_ghost_house;
use crate::game::map::labyrinth::spawn_labyrinth;
use crate::game::map::tunnel::{spawn_tunnel_hallways, spawn_tunnels, TunnelPlugin};
use crate::animation::Animations;
use crate::is;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::direction::Dir;
use crate::game::position::Pos;

pub mod labyrinth;
#[cfg(test)]
mod map_creator;
pub mod ghost_house;
pub mod tunnel;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Map>()
            .register_type::<Tiles>()
            .register_type::<Maze>()
            .register_type::<Wall>()
            .register_type::<WallStyle>()
            .register_type::<GhostHouseArea>()
            .register_type::<Tunnel>()
            .register_type::<TunnelHallway>()
            .register_type::<EnergizerSpawns>()
            .register_type::<EnergizerSpawn>()
            .register_type::<DotSpawns>()
            .register_type::<DotSpawn>()
            .register_type::<FruitSpawn>()
            .register_type::<PacmanSpawn>()
            .add_plugins((
                TunnelPlugin,
                JsonAssetPlugin::<RawMap>::new(&["map.json"])
            ))
            .add_systems(OnExit(CreateSpriteSheets), spawn_map)
            .add_systems(OnEnter(Game(LevelTransition)), set_animation_to_blinking)
            .add_systems(OnExit(Game(LevelTransition)), set_animation_to_idle)
        ;
    }
}

/// Component for the parent map entity
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum Tiles {
    Single { pos: Pos },
    Double { pos_a: Pos, pos_b: Pos },
}

impl Default for Tiles {
    fn default() -> Self {
        Tiles::Single {pos: Pos::default()}
    }
}

/// Parent of all walls in the maze. For organization only.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Maze;

/// Component to identify a wall
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Wall;

/// Describes how a wall looks
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct WallStyle {
    pub wall_type: WallType_,
    pub rotation: Rotation,
    pub is_corner: bool,
}

#[derive(Reflect, Default)]
pub enum WallType_ {
    #[default]
    Inner,
    Outer,
    Ghost,
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Rotation {
    #[default]
    D0,
    D90,
    D180,
    D270,
}

/// Marks a tile to spawn pacman here
#[derive(Component, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct PacmanSpawn(pub Vec3);

/// Parent component for all dot spawns (for organization only)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DotSpawns;

/// Coordinates where a dot can spawn
#[derive(Component, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct DotSpawn(pub Vec3);

/// Parent component for all energizer spawns (for organization only)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EnergizerSpawns;

/// Marks a tile to spawn an energizer here.
#[derive(Component, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct EnergizerSpawn(pub Vec3);

/// Marks a tile to spawn a fruit here
#[derive(Component, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct FruitSpawn(pub Vec3);

#[derive(Component)]
pub struct GhostCorner {
    pub ghost: Ghost,
    pub position: Pos,
}

/// A single tile of the ghost house
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GhostHouseArea {
    pub rotation: Rotation
}

/// Tile where pacman or a ghost can switch to another tunnel with the same index
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tunnel {
    pub index: usize,
    pub direction: Dir
}

/// Tile leading to a tunnel, which also slows down ghosts.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TunnelHallway;

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

    let children = [spawn_labyrinth(&mut commands, &tile_map, &sprite_sheets)]
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

impl Rotation {
    /// Return the Quat created from rotating around the z axes for the given degree.
    pub fn quat_z(&self) -> Quat {
        match self {
            D0 => Quat::from_rotation_z(PI * 0.0),
            D90 => Quat::from_rotation_z(PI * 1.5),
            D180 => Quat::from_rotation_z(PI),
            D270 => Quat::from_rotation_z(PI * 0.5),
        }
    }
}

/// Macro which quickly creates an element filter (closure Fn(&Element) -> bool) by passing a pattern.
///
/// The alternative would be a match/if let expression, which is much longer and harder to read.
#[macro_export]
macro_rules! is {
    ($pattern:pat) => {
        {
            |e: &crate::game::map::Element| match e {
                $pattern => true,
                _ => false
            }
        }
    };
}
