use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::prelude::{Dir, Ghost, Pos};

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Map>()
            .register_type::<Tiles>()
            .register_type::<Maze>()
            .register_type::<Wall>()
            .register_type::<WallStyle>()
            .register_type::<WallType_>()
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

/// Parent component for everything related to the ghost house
#[derive(Component, Reflect)]
pub struct GhostHouse;

#[derive(Component, Reflect, Copy, Clone)]
pub struct GhostSpawn {
    pub ghost: Ghost,
    pub coordinates: Vec3,
    pub spawn_direction: Dir,
    pub positions: [Pos; 2],
}