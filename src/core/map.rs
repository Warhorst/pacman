use std::f32::consts::PI;
use bevy::prelude::*;
use crate::core::prelude::*;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Map>()
            .register_type::<Tiles>()
            .register_type::<Maze>()
            .register_type::<Wall>()
            .register_type::<WallStyle>()
            .register_type::<WallType>()
            .register_type::<Rotation>()
            .register_type::<GhostHouseArea>()
            .register_type::<Tunnel>()
            .register_type::<TunnelHallway>()
            .register_type::<EnergizerSpawns>()
            .register_type::<EnergizerSpawn>()
            .register_type::<DotSpawns>()
            .register_type::<DotSpawn>()
            .register_type::<FruitSpawn>()
            .register_type::<PacmanSpawn>()
            .register_type::<GhostHouse>()
            .register_type::<GhostSpawn>()
            .register_type::<GhostCorner>()
            .register_type::<OneWay>()
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

/// An entity with this component spans either one or more tiles on the map.
#[derive(Component, Reflect, Copy, Clone)]
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

impl Tiles {
    pub fn to_vec3(&self, z: f32) -> Vec3 {
        match self {
            Tiles::Single { pos } => pos.to_vec3(z),
            Tiles::Double { pos_a, pos_b } => Vec3::from_positions([pos_a, pos_b], z)
        }
    }

    pub fn to_pos(&self) -> Pos {
        match self {
            Tiles::Single { pos } => *pos,
            Tiles::Double { .. } => panic!("can only retrieve the position for single position tiles")
        }
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
    pub wall_type: WallType,
    pub rotation: Rotation,
    pub is_corner: bool,
}

#[derive(Reflect, Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum WallType {
    #[default]
    Inner,
    Outer,
}

#[derive(Reflect, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Rotation {
    #[default]
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

/// Marks a tile to spawn pacman here
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PacmanSpawn;

/// Parent component for all dot spawns (for organization only)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DotSpawns;

/// Coordinates where a dot can spawn
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DotSpawn;

/// Parent component for all energizer spawns (for organization only)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EnergizerSpawns;

/// Marks a tile to spawn an energizer here.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EnergizerSpawn;

/// Marks a tile to spawn a fruit here
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FruitSpawn;

#[derive(Component, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct GhostCorner(pub Ghost);

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

/// Parent component for everything related to the ghost house
#[derive(Component, Reflect)]
pub struct GhostHouse;

/// Spawn area of a ghost
#[derive(Component, Reflect, Copy, Clone)]
pub struct GhostSpawn {
    pub ghost: Ghost,
    pub coordinates: Vec3,
    pub spawn_direction: Dir,
    pub positions: [Pos; 2],
}

/// Marks a tile as one way. A one way is used to mark an intersection as a point where
/// a ghost can only move left or right.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct OneWay;