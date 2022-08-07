use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::position::Position;
use crate::game_asset_handles::GameAssetHandles;
use crate::game_asset_handles::keys::MAP;
use crate::life_cycle::LifeCycle::Loading;
use crate::map::board::Board;

pub mod board;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(JsonAssetPlugin::<Fields>::new(&["map.json"]))
            .add_system_set(
                SystemSet::on_exit(Loading).with_system(create_board_and_map)
            )
        ;
    }
}

fn create_board_and_map(
    mut commands: Commands,
    game_asset_handles: Res<GameAssetHandles>,
    fields_assets: Res<Assets<Fields>>,
) {
    let fields = fields_assets.get(&game_asset_handles.get_handle(MAP)).expect("the map should be loaded at this point");
    let map = Map::new(&fields);
    let board = Board::new(&map);
    commands.insert_resource(map);
    commands.insert_resource(board);
}

#[derive(Clone, Deref, Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "a09992c9-9567-42d9-a0ac-c998756e4073"]
struct Fields(Vec<Field>);

/// Resource that knows the spawn locations of every entity, based on an external map file.
///
/// The map should only be used to spawn or respawn entities into the world.
pub struct Map {
    elements_map: HashMap<Position, Vec<Element>>,
}

impl Map {
    fn new(fields: &Fields) -> Self {
        Map {
            elements_map: fields.clone().0.into_iter()
                .map(|f| (f.position, f.elements))
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
            .filter(move |(_, elems)| Self::elements_match_filter(elems.iter(), &filter))
            .map(|(pos, _)| pos)
    }

    fn elements_match_filter<'a>(elems: impl IntoIterator<Item=&'a Element>, filter: &impl Fn(&Element) -> bool) -> bool {
        elems.into_iter()
            .map(filter)
            .max()
            .unwrap_or(false)
    }

    /// Return an iterator over all positions and elements.
    pub fn position_element_iter(&self) -> impl IntoIterator<Item=(&Position, &Element)> {
        self.elements_map
            .iter()
            .flat_map(|(pos, elements)| elements.into_iter().map(move |elem| (pos, elem)))
    }

    /// Return the coordinates between two positions matching the given filter.
    ///
    /// There must be exactly two positions matching this filter and these positions must be neighbored.
    /// This should only fail with invalid map design.
    pub fn coordinates_between_positions_matching(&self, filter: impl Fn(&Element) -> bool) -> Vec3 {
        let positions_matching_filter = self.get_positions_matching(filter).into_iter().collect::<Vec<_>>();

        if positions_matching_filter.len() != 2 {
            panic!("There must be exactly two positions matching the given filter!")
        }

        let (pos_0, pos_1) = (positions_matching_filter[0], positions_matching_filter[1]);
        let neighbour_direction = pos_0.get_neighbour_direction(&pos_1).expect("The two positions must be neighbored!");
        let (vec_0, vec_1) = (Vec3::from(pos_0), Vec3::from(pos_1));

        match neighbour_direction {
            Up | Down => {
                let x = vec_0.x;
                let y = (vec_0.y + vec_1.y) / 2.0;
                Vec3::new(x, y, 0.0)
            }
            Left | Right => {
                let x = (vec_0.x + vec_1.x) / 2.0;
                let y = vec_0.y;
                Vec3::new(x, y, 0.0)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    pub position: Position,
    pub elements: Vec<Element>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Wall {
        wall_type: WallType,
        rotation: Rotation,
        is_corner: bool,
    },
    GhostHouseEntrance {
        rotation: Rotation
    },
    GhostHouse,
    PacManSpawn,
    DotSpawn,
    EnergizerSpawn,
    BlinkySpawn,
    PinkySpawn,
    InkySpawn,
    ClydeSpawn,
    FruitSpawn,
    BlinkyCorner,
    PinkyCorner,
    InkyCorner,
    ClydeCorner,
    Tunnel {
        index: usize,
        opening_direction: Direction,
    },
    TunnelEntrance,
    TunnelHallway,
    InvisibleWall,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum WallType {
    Outer,
    Inner,
    Ghost,
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