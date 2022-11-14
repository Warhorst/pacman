use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::animation::Animations;
use crate::common::{Direction, FromPositions};
use crate::common::position::Position;
use crate::constants::{DOT_Z, ENERGIZER_Z, FRUIT_Z, PACMAN_Z};
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::{Blinky, Clyde, Inky, Pinky};
use crate::is;
use crate::life_cycle::LifeCycle::{LevelTransition, Loading};
use crate::map::ghost_house::spawn_ghost_house;
use crate::map::labyrinth::spawn_labyrinth;
use crate::map::tunnel::{spawn_tunnel_hallways, spawn_tunnels, TunnelPlugin};
use crate::sprite_sheet::SpriteSheet;

pub mod labyrinth;
#[cfg(test)]
mod map_creator;
pub mod ghost_house;
pub mod tunnel;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TunnelPlugin)
            .add_plugin(JsonAssetPlugin::<RawMap>::new(&["map.json"]))
            .add_system_set(
                SystemSet::on_exit(Loading).with_system(spawn_map)
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

/// Component for the parent map entity
#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
}

/// Component to identify a wall
#[derive(Component)]
pub struct Wall;

#[derive(Component, Deref)]
pub struct PacmanSpawn(pub Vec3);

/// Parent component for all dot spawns (for organization only)
#[derive(Component)]
pub struct DotSpawns;

/// Coordinates where a dot can spawn
#[derive(Component, Deref)]
pub struct DotSpawn(pub Vec3);

/// Parent component for all energizer spawns (for organization only)
#[derive(Component)]
pub struct EnergizerSpawns;

#[derive(Component, Deref)]
pub struct EnergizerSpawn(pub Vec3);

#[derive(Component, Deref)]
pub struct FruitSpawn(pub Vec3);

#[derive(Component)]
pub struct GhostCorner {
    pub ghost: Ghost,
    pub position: Position,
}

fn spawn_map(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    fields_assets: Res<Assets<RawMap>>,
) {
    let fields = fields_assets.get(&loaded_assets.get_handle("maps/default.map.json")).expect("the map should be loaded at this point");
    let tile_map = TileMap::new(&fields);

    let map = commands.spawn((
        Name::new("Map"),
        Map {
            width: tile_map.get_width(),
            height: tile_map.get_height(),
        },
        SpatialBundle::default(),
    )).id();

    let children = [spawn_labyrinth(&mut commands, &tile_map, &loaded_assets, &sprite_sheets)]
        .into_iter()
        .chain([spawn_dot_spawns(&mut commands, &tile_map)])
        .chain([spawn_energizer_spawns(&mut commands, &tile_map)])
        .chain([spawn_pacman_spawn(&mut commands, &tile_map)])
        .chain([spawn_fruit_spawns(&mut commands, &tile_map)])
        .chain([spawn_ghost_house(&mut commands, &tile_map, &loaded_assets, &sprite_sheets)])
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
                        DotSpawn(pos.to_vec(DOT_Z))
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
                        EnergizerSpawn(pos.to_vec(ENERGIZER_Z))
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
        (spawn_corner)(Blinky, tile_map.blinky_corner),
        (spawn_corner)(Pinky, tile_map.pinky_corner),
        (spawn_corner)(Inky, tile_map.inky_corner),
        (spawn_corner)(Clyde, tile_map.clyde_corner),
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

#[derive(Clone, Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "a09992c9-9567-42d9-a0ac-c998756e4073"]
pub struct RawMap {
    pub blinky_corner: Position,
    pub pinky_corner: Position,
    pub inky_corner: Position,
    pub clyde_corner: Position,
    pub fields: Vec<Field>,
}

/// Resource that knows the spawn locations of every entity, based on an external map file.
///
/// The map should only be used to spawn or respawn entities into the world.
pub struct TileMap {
    pub blinky_corner: Position,
    pub pinky_corner: Position,
    pub inky_corner: Position,
    pub clyde_corner: Position,
    elements_map: HashMap<Position, Element>,
}

impl TileMap {
    fn new(raw_map: &RawMap) -> Self {
        TileMap {
            blinky_corner: raw_map.blinky_corner,
            pinky_corner: raw_map.pinky_corner,
            inky_corner: raw_map.inky_corner,
            clyde_corner: raw_map.clyde_corner,
            elements_map: raw_map.fields.clone().into_iter()
                .map(|f| (f.position, f.element))
                .collect(),
        }
    }

    pub(in crate::map) fn get_width(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.x)
            .collect::<HashSet<_>>()
            .len()
    }

    pub(in crate::map) fn get_height(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.y)
            .collect::<HashSet<_>>()
            .len()
    }

    /// Return an iterator over all positions matching the given element filter.
    pub fn get_positions_matching(&self, filter: impl Fn(&Element) -> bool) -> impl IntoIterator<Item=&Position> {
        self.elements_map.iter()
            .filter(move |(_, elem)| (filter)(elem))
            .map(|(pos, _)| pos)
    }

    /// Return an iterator over all positions and elements.
    pub fn position_element_iter(&self) -> impl IntoIterator<Item=(&Position, &Element)> {
        self.elements_map.iter()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    pub position: Position,
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
        opening_direction: Direction,
    },
    TunnelHallway,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum WallType {
    Outer,
    Inner,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Rotation {
    D0,
    D90,
    D180,
    D270,
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
            |e: &crate::map::Element| match e {
                $pattern => true,
                _ => false
            }
        }
    };
}
