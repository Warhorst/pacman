use std::collections::HashMap;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_sprite_sheet::SpriteSheets;
use serde::{Deserialize, Serialize};

use crate::core::prelude::*;

#[cfg(test)]
mod map_creator;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                JsonAssetPlugin::<RawMap>::new(&["map.json"])
            )
            // .add_systems(OnExit(Setup(CreateSpriteSheets)), spawn_map)
            .add_systems(OnEnter(Game(LevelTransition)), set_animation_to_blinking)
            .add_systems(OnExit(Game(LevelTransition)), set_animation_to_idle)
        ;
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