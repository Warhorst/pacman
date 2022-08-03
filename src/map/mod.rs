use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::position::Position;
use crate::constants::MAP_PATH;
use crate::map::board::Board;

pub mod board;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::load(MAP_PATH);
        app.insert_resource(Board::new(&map));
        app.insert_resource(map);
    }
}

/// Resource that knows the spawn locations of every entity, based on an external map file.
///
/// The map should only be used to spawn or respawn entities into the world.
pub struct Map {
    elements_map: HashMap<Position, Vec<Element>>,
}

impl Map {
    // If we are currently a wasm application, we cannot open files from the users disc without explicit permission.
    // Therefore, the map is saved here as string to open the map without a file.
    // TODO: Use a build script to write the current map here automatically (if I don't find a better alternative)
    const DEFAULT_MAP_STRING: &'static str = r#"[{"position":{"x":2,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}},"PinkyCorner"]},{"position":{"x":3,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":4,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":5,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":8,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":9,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":10,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":11,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":14,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":15,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":16,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":17,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":18,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":19,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":22,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":23,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":24,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":25,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":28,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":29,"y":30},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}},"BlinkyCorner"]},{"position":{"x":2,"y":29},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":29},"elements":["DotSpawn"]},{"position":{"x":4,"y":29},"elements":["DotSpawn"]},{"position":{"x":5,"y":29},"elements":["DotSpawn"]},{"position":{"x":6,"y":29},"elements":["DotSpawn"]},{"position":{"x":7,"y":29},"elements":["DotSpawn"]},{"position":{"x":8,"y":29},"elements":["DotSpawn"]},{"position":{"x":9,"y":29},"elements":["DotSpawn"]},{"position":{"x":10,"y":29},"elements":["DotSpawn"]},{"position":{"x":11,"y":29},"elements":["DotSpawn"]},{"position":{"x":12,"y":29},"elements":["DotSpawn"]},{"position":{"x":13,"y":29},"elements":["DotSpawn"]},{"position":{"x":14,"y":29},"elements":["DotSpawn"]},{"position":{"x":15,"y":29},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":16,"y":29},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":17,"y":29},"elements":["DotSpawn"]},{"position":{"x":18,"y":29},"elements":["DotSpawn"]},{"position":{"x":19,"y":29},"elements":["DotSpawn"]},{"position":{"x":20,"y":29},"elements":["DotSpawn"]},{"position":{"x":21,"y":29},"elements":["DotSpawn"]},{"position":{"x":22,"y":29},"elements":["DotSpawn"]},{"position":{"x":23,"y":29},"elements":["DotSpawn"]},{"position":{"x":24,"y":29},"elements":["DotSpawn"]},{"position":{"x":25,"y":29},"elements":["DotSpawn"]},{"position":{"x":26,"y":29},"elements":["DotSpawn"]},{"position":{"x":27,"y":29},"elements":["DotSpawn"]},{"position":{"x":28,"y":29},"elements":["DotSpawn"]},{"position":{"x":29,"y":29},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":28},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":28},"elements":["DotSpawn"]},{"position":{"x":4,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":5,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":8,"y":28},"elements":["DotSpawn"]},{"position":{"x":9,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":10,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":11,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":14,"y":28},"elements":["DotSpawn"]},{"position":{"x":15,"y":28},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":16,"y":28},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":17,"y":28},"elements":["DotSpawn"]},{"position":{"x":18,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":19,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":22,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":23,"y":28},"elements":["DotSpawn"]},{"position":{"x":24,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":25,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":28},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":28,"y":28},"elements":["DotSpawn"]},{"position":{"x":29,"y":28},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":27},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":27},"elements":["EnergizerSpawn"]},{"position":{"x":4,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":5,"y":27},"elements":[]},{"position":{"x":6,"y":27},"elements":[]},{"position":{"x":7,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":8,"y":27},"elements":["DotSpawn"]},{"position":{"x":9,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":27},"elements":[]},{"position":{"x":11,"y":27},"elements":[]},{"position":{"x":12,"y":27},"elements":[]},{"position":{"x":13,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":14,"y":27},"elements":["DotSpawn"]},{"position":{"x":15,"y":27},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":16,"y":27},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":17,"y":27},"elements":["DotSpawn"]},{"position":{"x":18,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":19,"y":27},"elements":[]},{"position":{"x":20,"y":27},"elements":[]},{"position":{"x":21,"y":27},"elements":[]},{"position":{"x":22,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":23,"y":27},"elements":["DotSpawn"]},{"position":{"x":24,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":27},"elements":[]},{"position":{"x":26,"y":27},"elements":[]},{"position":{"x":27,"y":27},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":28,"y":27},"elements":["EnergizerSpawn"]},{"position":{"x":29,"y":27},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":26},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":26},"elements":["DotSpawn"]},{"position":{"x":4,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":5,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":8,"y":26},"elements":["DotSpawn"]},{"position":{"x":9,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":10,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":11,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":12,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":13,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":14,"y":26},"elements":["DotSpawn"]},{"position":{"x":15,"y":26},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":16,"y":26},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":17,"y":26},"elements":["DotSpawn"]},{"position":{"x":18,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":19,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":20,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":21,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":22,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":23,"y":26},"elements":["DotSpawn"]},{"position":{"x":24,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":25,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":26},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":28,"y":26},"elements":["DotSpawn"]},{"position":{"x":29,"y":26},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":25},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":25},"elements":["DotSpawn"]},{"position":{"x":4,"y":25},"elements":["DotSpawn"]},{"position":{"x":5,"y":25},"elements":["DotSpawn"]},{"position":{"x":6,"y":25},"elements":["DotSpawn"]},{"position":{"x":7,"y":25},"elements":["DotSpawn"]},{"position":{"x":8,"y":25},"elements":["DotSpawn"]},{"position":{"x":9,"y":25},"elements":["DotSpawn"]},{"position":{"x":10,"y":25},"elements":["DotSpawn"]},{"position":{"x":11,"y":25},"elements":["DotSpawn"]},{"position":{"x":12,"y":25},"elements":["DotSpawn"]},{"position":{"x":13,"y":25},"elements":["DotSpawn"]},{"position":{"x":14,"y":25},"elements":["DotSpawn"]},{"position":{"x":15,"y":25},"elements":["DotSpawn"]},{"position":{"x":16,"y":25},"elements":["DotSpawn"]},{"position":{"x":17,"y":25},"elements":["DotSpawn"]},{"position":{"x":18,"y":25},"elements":["DotSpawn"]},{"position":{"x":19,"y":25},"elements":["DotSpawn"]},{"position":{"x":20,"y":25},"elements":["DotSpawn"]},{"position":{"x":21,"y":25},"elements":["DotSpawn"]},{"position":{"x":22,"y":25},"elements":["DotSpawn"]},{"position":{"x":23,"y":25},"elements":["DotSpawn"]},{"position":{"x":24,"y":25},"elements":["DotSpawn"]},{"position":{"x":25,"y":25},"elements":["DotSpawn"]},{"position":{"x":26,"y":25},"elements":["DotSpawn"]},{"position":{"x":27,"y":25},"elements":["DotSpawn"]},{"position":{"x":28,"y":25},"elements":["DotSpawn"]},{"position":{"x":29,"y":25},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":24},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":24},"elements":["DotSpawn"]},{"position":{"x":4,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":5,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":8,"y":24},"elements":["DotSpawn"]},{"position":{"x":9,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":10,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":11,"y":24},"elements":["DotSpawn"]},{"position":{"x":12,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":13,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":14,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":15,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":16,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":17,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":18,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":19,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":20,"y":24},"elements":["DotSpawn"]},{"position":{"x":21,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":22,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":23,"y":24},"elements":["DotSpawn"]},{"position":{"x":24,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":25,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":24},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":28,"y":24},"elements":["DotSpawn"]},{"position":{"x":29,"y":24},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":23},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":23},"elements":["DotSpawn"]},{"position":{"x":4,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":5,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":8,"y":23},"elements":["DotSpawn"]},{"position":{"x":9,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":23},"elements":["DotSpawn"]},{"position":{"x":12,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":13,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":14,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":15,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":16,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":17,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":18,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":19,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":20,"y":23},"elements":["DotSpawn"]},{"position":{"x":21,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":23},"elements":["DotSpawn"]},{"position":{"x":24,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":25,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":23},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":28,"y":23},"elements":["DotSpawn"]},{"position":{"x":29,"y":23},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":22},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":22},"elements":["DotSpawn"]},{"position":{"x":4,"y":22},"elements":["DotSpawn"]},{"position":{"x":5,"y":22},"elements":["DotSpawn"]},{"position":{"x":6,"y":22},"elements":["DotSpawn"]},{"position":{"x":7,"y":22},"elements":["DotSpawn"]},{"position":{"x":8,"y":22},"elements":["DotSpawn"]},{"position":{"x":9,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":22},"elements":["DotSpawn"]},{"position":{"x":12,"y":22},"elements":["DotSpawn"]},{"position":{"x":13,"y":22},"elements":["DotSpawn"]},{"position":{"x":14,"y":22},"elements":["DotSpawn"]},{"position":{"x":15,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":16,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":22},"elements":["DotSpawn"]},{"position":{"x":18,"y":22},"elements":["DotSpawn"]},{"position":{"x":19,"y":22},"elements":["DotSpawn"]},{"position":{"x":20,"y":22},"elements":["DotSpawn"]},{"position":{"x":21,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":22},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":22},"elements":["DotSpawn"]},{"position":{"x":24,"y":22},"elements":["DotSpawn"]},{"position":{"x":25,"y":22},"elements":["DotSpawn"]},{"position":{"x":26,"y":22},"elements":["DotSpawn"]},{"position":{"x":27,"y":22},"elements":["DotSpawn"]},{"position":{"x":28,"y":22},"elements":["DotSpawn"]},{"position":{"x":29,"y":22},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":3,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":4,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":5,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":8,"y":21},"elements":["DotSpawn"]},{"position":{"x":9,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":10,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":11,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":14,"y":21},"elements":[]},{"position":{"x":15,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":16,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":21},"elements":[]},{"position":{"x":18,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":19,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":22,"y":21},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":21},"elements":["DotSpawn"]},{"position":{"x":24,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":25,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":28,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":29,"y":21},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":2,"y":20},"elements":[]},{"position":{"x":3,"y":20},"elements":[]},{"position":{"x":4,"y":20},"elements":[]},{"position":{"x":5,"y":20},"elements":[]},{"position":{"x":6,"y":20},"elements":[]},{"position":{"x":7,"y":20},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":20},"elements":["DotSpawn"]},{"position":{"x":9,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":10,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":11,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":14,"y":20},"elements":[]},{"position":{"x":15,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":16,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":17,"y":20},"elements":[]},{"position":{"x":18,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":19,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":22,"y":20},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":20},"elements":["DotSpawn"]},{"position":{"x":24,"y":20},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":20},"elements":[]},{"position":{"x":26,"y":20},"elements":[]},{"position":{"x":27,"y":20},"elements":[]},{"position":{"x":28,"y":20},"elements":[]},{"position":{"x":29,"y":20},"elements":[]},{"position":{"x":2,"y":19},"elements":[]},{"position":{"x":3,"y":19},"elements":[]},{"position":{"x":4,"y":19},"elements":[]},{"position":{"x":5,"y":19},"elements":[]},{"position":{"x":6,"y":19},"elements":[]},{"position":{"x":7,"y":19},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":19},"elements":["DotSpawn"]},{"position":{"x":9,"y":19},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":19},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":19},"elements":[]},{"position":{"x":12,"y":19},"elements":[]},{"position":{"x":13,"y":19},"elements":[]},{"position":{"x":14,"y":19},"elements":[]},{"position":{"x":15,"y":19},"elements":["BlinkySpawn"]},{"position":{"x":16,"y":19},"elements":["BlinkySpawn"]},{"position":{"x":17,"y":19},"elements":[]},{"position":{"x":18,"y":19},"elements":[]},{"position":{"x":19,"y":19},"elements":[]},{"position":{"x":20,"y":19},"elements":[]},{"position":{"x":21,"y":19},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":19},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":19},"elements":["DotSpawn"]},{"position":{"x":24,"y":19},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":19},"elements":[]},{"position":{"x":26,"y":19},"elements":[]},{"position":{"x":27,"y":19},"elements":[]},{"position":{"x":28,"y":19},"elements":[]},{"position":{"x":29,"y":19},"elements":[]},{"position":{"x":2,"y":18},"elements":[]},{"position":{"x":3,"y":18},"elements":[]},{"position":{"x":4,"y":18},"elements":[]},{"position":{"x":5,"y":18},"elements":[]},{"position":{"x":6,"y":18},"elements":[]},{"position":{"x":7,"y":18},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":18},"elements":["DotSpawn"]},{"position":{"x":9,"y":18},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":18},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":18},"elements":[]},{"position":{"x":12,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D0","is_corner":true}}]},{"position":{"x":13,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D0","is_corner":false}}]},{"position":{"x":14,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D0","is_corner":false}}]},{"position":{"x":15,"y":18},"elements":[{"GhostHouseEntrance":{"rotation":"D0"}}]},{"position":{"x":16,"y":18},"elements":[{"GhostHouseEntrance":{"rotation":"D0"}}]},{"position":{"x":17,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D0","is_corner":false}}]},{"position":{"x":18,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D0","is_corner":false}}]},{"position":{"x":19,"y":18},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D90","is_corner":true}}]},{"position":{"x":20,"y":18},"elements":[]},{"position":{"x":21,"y":18},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":18},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":18},"elements":["DotSpawn"]},{"position":{"x":24,"y":18},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":18},"elements":[]},{"position":{"x":26,"y":18},"elements":[]},{"position":{"x":27,"y":18},"elements":[]},{"position":{"x":28,"y":18},"elements":[]},{"position":{"x":29,"y":18},"elements":[]},{"position":{"x":0,"y":17},"elements":["InvisibleWall"]},{"position":{"x":1,"y":17},"elements":["InvisibleWall"]},{"position":{"x":2,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":3,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":4,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":5,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":8,"y":17},"elements":["DotSpawn"]},{"position":{"x":9,"y":17},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":10,"y":17},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":11,"y":17},"elements":[]},{"position":{"x":12,"y":17},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D90","is_corner":false}}]},{"position":{"x":13,"y":17},"elements":["GhostHouse"]},{"position":{"x":14,"y":17},"elements":["GhostHouse"]},{"position":{"x":15,"y":17},"elements":["GhostHouse"]},{"position":{"x":16,"y":17},"elements":["GhostHouse"]},{"position":{"x":17,"y":17},"elements":["GhostHouse"]},{"position":{"x":18,"y":17},"elements":["GhostHouse"]},{"position":{"x":19,"y":17},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D90","is_corner":false}}]},{"position":{"x":20,"y":17},"elements":[]},{"position":{"x":21,"y":17},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":22,"y":17},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":23,"y":17},"elements":["DotSpawn"]},{"position":{"x":24,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":25,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":28,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":29,"y":17},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":30,"y":17},"elements":["InvisibleWall"]},{"position":{"x":31,"y":17},"elements":["InvisibleWall"]},{"position":{"x":0,"y":16},"elements":[{"Tunnel":{"index":0,"opening_direction":"Left"}}]},{"position":{"x":1,"y":16},"elements":["TunnelEntrance"]},{"position":{"x":2,"y":16},"elements":["TunnelHallway"]},{"position":{"x":3,"y":16},"elements":["TunnelHallway"]},{"position":{"x":4,"y":16},"elements":["TunnelHallway"]},{"position":{"x":5,"y":16},"elements":["TunnelHallway"]},{"position":{"x":6,"y":16},"elements":["TunnelHallway"]},{"position":{"x":7,"y":16},"elements":["TunnelHallway"]},{"position":{"x":8,"y":16},"elements":[]},{"position":{"x":9,"y":16},"elements":[]},{"position":{"x":10,"y":16},"elements":[]},{"position":{"x":11,"y":16},"elements":[]},{"position":{"x":12,"y":16},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D270","is_corner":false}}]},{"position":{"x":13,"y":16},"elements":["GhostHouse","InkySpawn"]},{"position":{"x":14,"y":16},"elements":["GhostHouse","InkySpawn"]},{"position":{"x":15,"y":16},"elements":["GhostHouse","PinkySpawn"]},{"position":{"x":16,"y":16},"elements":["GhostHouse","PinkySpawn"]},{"position":{"x":17,"y":16},"elements":["GhostHouse","ClydeSpawn"]},{"position":{"x":18,"y":16},"elements":["GhostHouse","ClydeSpawn"]},{"position":{"x":19,"y":16},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D90","is_corner":false}}]},{"position":{"x":20,"y":16},"elements":[]},{"position":{"x":21,"y":16},"elements":[]},{"position":{"x":22,"y":16},"elements":[]},{"position":{"x":23,"y":16},"elements":[]},{"position":{"x":24,"y":16},"elements":["TunnelHallway"]},{"position":{"x":25,"y":16},"elements":["TunnelHallway"]},{"position":{"x":26,"y":16},"elements":["TunnelHallway"]},{"position":{"x":27,"y":16},"elements":["TunnelHallway"]},{"position":{"x":28,"y":16},"elements":["TunnelHallway"]},{"position":{"x":29,"y":16},"elements":["TunnelHallway"]},{"position":{"x":30,"y":16},"elements":["TunnelEntrance"]},{"position":{"x":31,"y":16},"elements":[{"Tunnel":{"index":0,"opening_direction":"Right"}}]},{"position":{"x":0,"y":15},"elements":["InvisibleWall"]},{"position":{"x":1,"y":15},"elements":["InvisibleWall"]},{"position":{"x":2,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":3,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":4,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":5,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":8,"y":15},"elements":["DotSpawn"]},{"position":{"x":9,"y":15},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":10,"y":15},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":11,"y":15},"elements":[]},{"position":{"x":12,"y":15},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D270","is_corner":false}}]},{"position":{"x":13,"y":15},"elements":["GhostHouse"]},{"position":{"x":14,"y":15},"elements":["GhostHouse"]},{"position":{"x":15,"y":15},"elements":["GhostHouse"]},{"position":{"x":16,"y":15},"elements":["GhostHouse"]},{"position":{"x":17,"y":15},"elements":["GhostHouse"]},{"position":{"x":18,"y":15},"elements":["GhostHouse"]},{"position":{"x":19,"y":15},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D90","is_corner":false}}]},{"position":{"x":20,"y":15},"elements":[]},{"position":{"x":21,"y":15},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":22,"y":15},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":23,"y":15},"elements":["DotSpawn"]},{"position":{"x":24,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":25,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":28,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":29,"y":15},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":30,"y":15},"elements":["InvisibleWall"]},{"position":{"x":31,"y":15},"elements":["InvisibleWall"]},{"position":{"x":2,"y":14},"elements":[]},{"position":{"x":3,"y":14},"elements":[]},{"position":{"x":4,"y":14},"elements":[]},{"position":{"x":5,"y":14},"elements":[]},{"position":{"x":6,"y":14},"elements":[]},{"position":{"x":7,"y":14},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":14},"elements":["DotSpawn"]},{"position":{"x":9,"y":14},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":14},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":14},"elements":[]},{"position":{"x":12,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D270","is_corner":true}}]},{"position":{"x":13,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":14,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":15,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":16,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":17,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":18,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":false}}]},{"position":{"x":19,"y":14},"elements":[{"Wall":{"wall_type":"Ghost","rotation":"D180","is_corner":true}}]},{"position":{"x":20,"y":14},"elements":[]},{"position":{"x":21,"y":14},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":14},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":14},"elements":["DotSpawn"]},{"position":{"x":24,"y":14},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":14},"elements":[]},{"position":{"x":26,"y":14},"elements":[]},{"position":{"x":27,"y":14},"elements":[]},{"position":{"x":28,"y":14},"elements":[]},{"position":{"x":29,"y":14},"elements":[]},{"position":{"x":2,"y":13},"elements":[]},{"position":{"x":3,"y":13},"elements":[]},{"position":{"x":4,"y":13},"elements":[]},{"position":{"x":5,"y":13},"elements":[]},{"position":{"x":6,"y":13},"elements":[]},{"position":{"x":7,"y":13},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":13},"elements":["DotSpawn"]},{"position":{"x":9,"y":13},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":13},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":13},"elements":[]},{"position":{"x":12,"y":13},"elements":[]},{"position":{"x":13,"y":13},"elements":[]},{"position":{"x":14,"y":13},"elements":[]},{"position":{"x":15,"y":13},"elements":["FruitSpawn"]},{"position":{"x":16,"y":13},"elements":["FruitSpawn"]},{"position":{"x":17,"y":13},"elements":[]},{"position":{"x":18,"y":13},"elements":[]},{"position":{"x":19,"y":13},"elements":[]},{"position":{"x":20,"y":13},"elements":[]},{"position":{"x":21,"y":13},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":13},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":13},"elements":["DotSpawn"]},{"position":{"x":24,"y":13},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":13},"elements":[]},{"position":{"x":26,"y":13},"elements":[]},{"position":{"x":27,"y":13},"elements":[]},{"position":{"x":28,"y":13},"elements":[]},{"position":{"x":29,"y":13},"elements":[]},{"position":{"x":2,"y":12},"elements":[]},{"position":{"x":3,"y":12},"elements":[]},{"position":{"x":4,"y":12},"elements":[]},{"position":{"x":5,"y":12},"elements":[]},{"position":{"x":6,"y":12},"elements":[]},{"position":{"x":7,"y":12},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":8,"y":12},"elements":["DotSpawn"]},{"position":{"x":9,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":12},"elements":[]},{"position":{"x":12,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":13,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":14,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":15,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":16,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":17,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":18,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":19,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":20,"y":12},"elements":[]},{"position":{"x":21,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":12},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":12},"elements":["DotSpawn"]},{"position":{"x":24,"y":12},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":12},"elements":[]},{"position":{"x":26,"y":12},"elements":[]},{"position":{"x":27,"y":12},"elements":[]},{"position":{"x":28,"y":12},"elements":[]},{"position":{"x":29,"y":12},"elements":[]},{"position":{"x":2,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":3,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":4,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":5,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":8,"y":11},"elements":["DotSpawn"]},{"position":{"x":9,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":10,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":11,"y":11},"elements":[]},{"position":{"x":12,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":13,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":14,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":15,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":16,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":17,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":18,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":19,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":20,"y":11},"elements":[]},{"position":{"x":21,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":22,"y":11},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":23,"y":11},"elements":["DotSpawn"]},{"position":{"x":24,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":25,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":28,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":29,"y":11},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":2,"y":10},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":10},"elements":["DotSpawn"]},{"position":{"x":4,"y":10},"elements":["DotSpawn"]},{"position":{"x":5,"y":10},"elements":["DotSpawn"]},{"position":{"x":6,"y":10},"elements":["DotSpawn"]},{"position":{"x":7,"y":10},"elements":["DotSpawn"]},{"position":{"x":8,"y":10},"elements":["DotSpawn"]},{"position":{"x":9,"y":10},"elements":["DotSpawn"]},{"position":{"x":10,"y":10},"elements":["DotSpawn"]},{"position":{"x":11,"y":10},"elements":["DotSpawn"]},{"position":{"x":12,"y":10},"elements":["DotSpawn"]},{"position":{"x":13,"y":10},"elements":["DotSpawn"]},{"position":{"x":14,"y":10},"elements":["DotSpawn"]},{"position":{"x":15,"y":10},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":16,"y":10},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":10},"elements":["DotSpawn"]},{"position":{"x":18,"y":10},"elements":["DotSpawn"]},{"position":{"x":19,"y":10},"elements":["DotSpawn"]},{"position":{"x":20,"y":10},"elements":["DotSpawn"]},{"position":{"x":21,"y":10},"elements":["DotSpawn"]},{"position":{"x":22,"y":10},"elements":["DotSpawn"]},{"position":{"x":23,"y":10},"elements":["DotSpawn"]},{"position":{"x":24,"y":10},"elements":["DotSpawn"]},{"position":{"x":25,"y":10},"elements":["DotSpawn"]},{"position":{"x":26,"y":10},"elements":["DotSpawn"]},{"position":{"x":27,"y":10},"elements":["DotSpawn"]},{"position":{"x":28,"y":10},"elements":["DotSpawn"]},{"position":{"x":29,"y":10},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":9},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":9},"elements":["DotSpawn"]},{"position":{"x":4,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":5,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":8,"y":9},"elements":["DotSpawn"]},{"position":{"x":9,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":10,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":11,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":14,"y":9},"elements":["DotSpawn"]},{"position":{"x":15,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":16,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":9},"elements":["DotSpawn"]},{"position":{"x":18,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":19,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":22,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":23,"y":9},"elements":["DotSpawn"]},{"position":{"x":24,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":25,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":9},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":28,"y":9},"elements":["DotSpawn"]},{"position":{"x":29,"y":9},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":8},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":8},"elements":["DotSpawn"]},{"position":{"x":4,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":5,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":7,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":8,"y":8},"elements":["DotSpawn"]},{"position":{"x":9,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":10,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":11,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":12,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":13,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":14,"y":8},"elements":["DotSpawn"]},{"position":{"x":15,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":16,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":17,"y":8},"elements":["DotSpawn"]},{"position":{"x":18,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":19,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":20,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":21,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":22,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":23,"y":8},"elements":["DotSpawn"]},{"position":{"x":24,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":25,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":26,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":8},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":28,"y":8},"elements":["DotSpawn"]},{"position":{"x":29,"y":8},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":7},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":7},"elements":["EnergizerSpawn"]},{"position":{"x":4,"y":7},"elements":["DotSpawn"]},{"position":{"x":5,"y":7},"elements":["DotSpawn"]},{"position":{"x":6,"y":7},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":7,"y":7},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":8,"y":7},"elements":["DotSpawn"]},{"position":{"x":9,"y":7},"elements":["DotSpawn"]},{"position":{"x":10,"y":7},"elements":["DotSpawn"]},{"position":{"x":11,"y":7},"elements":["DotSpawn"]},{"position":{"x":12,"y":7},"elements":["DotSpawn"]},{"position":{"x":13,"y":7},"elements":["DotSpawn"]},{"position":{"x":14,"y":7},"elements":["DotSpawn"]},{"position":{"x":15,"y":7},"elements":["PacManSpawn"]},{"position":{"x":16,"y":7},"elements":["PacManSpawn"]},{"position":{"x":17,"y":7},"elements":["DotSpawn"]},{"position":{"x":18,"y":7},"elements":["DotSpawn"]},{"position":{"x":19,"y":7},"elements":["DotSpawn"]},{"position":{"x":20,"y":7},"elements":["DotSpawn"]},{"position":{"x":21,"y":7},"elements":["DotSpawn"]},{"position":{"x":22,"y":7},"elements":["DotSpawn"]},{"position":{"x":23,"y":7},"elements":["DotSpawn"]},{"position":{"x":24,"y":7},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":25,"y":7},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":26,"y":7},"elements":["DotSpawn"]},{"position":{"x":27,"y":7},"elements":["DotSpawn"]},{"position":{"x":28,"y":7},"elements":["EnergizerSpawn"]},{"position":{"x":29,"y":7},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":3,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":4,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":5,"y":6},"elements":["DotSpawn"]},{"position":{"x":6,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":7,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":8,"y":6},"elements":["DotSpawn"]},{"position":{"x":9,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":10,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":11,"y":6},"elements":["DotSpawn"]},{"position":{"x":12,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":13,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":14,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":15,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":16,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":17,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":18,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":19,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":20,"y":6},"elements":["DotSpawn"]},{"position":{"x":21,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":22,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":23,"y":6},"elements":["DotSpawn"]},{"position":{"x":24,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":25,"y":6},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":26,"y":6},"elements":["DotSpawn"]},{"position":{"x":27,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":28,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":false}}]},{"position":{"x":29,"y":6},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":2,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D0","is_corner":true}}]},{"position":{"x":3,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":4,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}}]},{"position":{"x":5,"y":5},"elements":["DotSpawn"]},{"position":{"x":6,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":7,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":8,"y":5},"elements":["DotSpawn"]},{"position":{"x":9,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":5},"elements":["DotSpawn"]},{"position":{"x":12,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":13,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":14,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":15,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":16,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":17,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":18,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":19,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":20,"y":5},"elements":["DotSpawn"]},{"position":{"x":21,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":5},"elements":["DotSpawn"]},{"position":{"x":24,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":25,"y":5},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":26,"y":5},"elements":["DotSpawn"]},{"position":{"x":27,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}}]},{"position":{"x":28,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":29,"y":5},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":true}}]},{"position":{"x":2,"y":4},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":4},"elements":["DotSpawn"]},{"position":{"x":4,"y":4},"elements":["DotSpawn"]},{"position":{"x":5,"y":4},"elements":["DotSpawn"]},{"position":{"x":6,"y":4},"elements":["DotSpawn"]},{"position":{"x":7,"y":4},"elements":["DotSpawn"]},{"position":{"x":8,"y":4},"elements":["DotSpawn"]},{"position":{"x":9,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":10,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":11,"y":4},"elements":["DotSpawn"]},{"position":{"x":12,"y":4},"elements":["DotSpawn"]},{"position":{"x":13,"y":4},"elements":["DotSpawn"]},{"position":{"x":14,"y":4},"elements":["DotSpawn"]},{"position":{"x":15,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":16,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":4},"elements":["DotSpawn"]},{"position":{"x":18,"y":4},"elements":["DotSpawn"]},{"position":{"x":19,"y":4},"elements":["DotSpawn"]},{"position":{"x":20,"y":4},"elements":["DotSpawn"]},{"position":{"x":21,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":22,"y":4},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":23,"y":4},"elements":["DotSpawn"]},{"position":{"x":24,"y":4},"elements":["DotSpawn"]},{"position":{"x":25,"y":4},"elements":["DotSpawn"]},{"position":{"x":26,"y":4},"elements":["DotSpawn"]},{"position":{"x":27,"y":4},"elements":["DotSpawn"]},{"position":{"x":28,"y":4},"elements":["DotSpawn"]},{"position":{"x":29,"y":4},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":3},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":3},"elements":["DotSpawn"]},{"position":{"x":4,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":5,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":6,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":7,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":8,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":9,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":10,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":11,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":12,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":13,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":14,"y":3},"elements":["DotSpawn"]},{"position":{"x":15,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":false}}]},{"position":{"x":16,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":false}}]},{"position":{"x":17,"y":3},"elements":["DotSpawn"]},{"position":{"x":18,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":true}}]},{"position":{"x":19,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":20,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":21,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":22,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":23,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":24,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":25,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":26,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D0","is_corner":false}}]},{"position":{"x":27,"y":3},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D90","is_corner":true}}]},{"position":{"x":28,"y":3},"elements":["DotSpawn"]},{"position":{"x":29,"y":3},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":2},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":2},"elements":["DotSpawn"]},{"position":{"x":4,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":5,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":8,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":9,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":10,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":11,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":12,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":13,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":14,"y":2},"elements":["DotSpawn"]},{"position":{"x":15,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":16,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":17,"y":2},"elements":["DotSpawn"]},{"position":{"x":18,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D270","is_corner":true}}]},{"position":{"x":19,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":20,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":21,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":22,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":23,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":24,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":25,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":2},"elements":[{"Wall":{"wall_type":"Inner","rotation":"D180","is_corner":true}}]},{"position":{"x":28,"y":2},"elements":["DotSpawn"]},{"position":{"x":29,"y":2},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":1},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":false}}]},{"position":{"x":3,"y":1},"elements":["DotSpawn"]},{"position":{"x":4,"y":1},"elements":["DotSpawn"]},{"position":{"x":5,"y":1},"elements":["DotSpawn"]},{"position":{"x":6,"y":1},"elements":["DotSpawn"]},{"position":{"x":7,"y":1},"elements":["DotSpawn"]},{"position":{"x":8,"y":1},"elements":["DotSpawn"]},{"position":{"x":9,"y":1},"elements":["DotSpawn"]},{"position":{"x":10,"y":1},"elements":["DotSpawn"]},{"position":{"x":11,"y":1},"elements":["DotSpawn"]},{"position":{"x":12,"y":1},"elements":["DotSpawn"]},{"position":{"x":13,"y":1},"elements":["DotSpawn"]},{"position":{"x":14,"y":1},"elements":["DotSpawn"]},{"position":{"x":15,"y":1},"elements":["DotSpawn"]},{"position":{"x":16,"y":1},"elements":["DotSpawn"]},{"position":{"x":17,"y":1},"elements":["DotSpawn"]},{"position":{"x":18,"y":1},"elements":["DotSpawn"]},{"position":{"x":19,"y":1},"elements":["DotSpawn"]},{"position":{"x":20,"y":1},"elements":["DotSpawn"]},{"position":{"x":21,"y":1},"elements":["DotSpawn"]},{"position":{"x":22,"y":1},"elements":["DotSpawn"]},{"position":{"x":23,"y":1},"elements":["DotSpawn"]},{"position":{"x":24,"y":1},"elements":["DotSpawn"]},{"position":{"x":25,"y":1},"elements":["DotSpawn"]},{"position":{"x":26,"y":1},"elements":["DotSpawn"]},{"position":{"x":27,"y":1},"elements":["DotSpawn"]},{"position":{"x":28,"y":1},"elements":["DotSpawn"]},{"position":{"x":29,"y":1},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D90","is_corner":false}}]},{"position":{"x":2,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D270","is_corner":true}},"ClydeCorner"]},{"position":{"x":3,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":4,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":5,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":6,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":7,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":8,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":9,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":10,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":11,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":12,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":13,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":14,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":15,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":16,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":17,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":18,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":19,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":20,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":21,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":22,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":23,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":24,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":25,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":26,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":27,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":28,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":false}}]},{"position":{"x":29,"y":0},"elements":[{"Wall":{"wall_type":"Outer","rotation":"D180","is_corner":true}},"InkyCorner"]}]"#;

    /// Load a map from a given file. The file must be a JSON containing an array of crate::map::Field.
    ///
    /// The call fails if the file could not be read.
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let fields: Vec<Field> = match File::open(path) {
            Ok(file) => serde_json::from_reader(file),
            Err(_) => serde_json::from_reader(Self::DEFAULT_MAP_STRING.as_bytes())
        }.expect("could not parse map from json");

        Map {
            elements_map: fields.clone().into_iter()
                .map(|f| (f.position, f.elements))
                .collect(),
        }
    }

    pub (in crate::map) fn get_width(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.x)
            .collect::<HashSet<_>>()
            .len()
    }

    pub (in crate::map) fn get_height(&self) -> usize {
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
            },
            Left | Right => {
                let x = (vec_0.x + vec_1.x) / 2.0;
                let y = vec_0.y;
                Vec3::new(x, y, 0.0)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Field {
    position: Position,
    elements: Vec<Element>,
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

/// This bullshit is only used to generate the json map until I have a better way to do this
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::OpenOptions;
    use std::io::Write;

    use crate::common::position::Position;
    use crate::common::Direction;
    use crate::map::{Element, Field, Rotation, WallType};
    use crate::map::Element::*;
    use crate::map::Rotation::*;
    use QuickWall::*;

    // #[test]
    // fn from_json() {
    //     serde_json::from_reader::<_, Map>(File::open("./maps/new_map.json").unwrap()).expect("Failed to deserialize map");
    // }

    #[derive(Copy, Clone)]
    enum QuickWall {
        I,
        O,
        G
    }

    impl QuickWall {
        fn to_wall(self) -> WallType {
            match self {
                I => WallType::Inner,
                O => WallType::Outer,
                G => WallType::Ghost,
            }
        }
    }

    #[test]
    fn to_json() {
        let fields = vec![
            create_field_line(2, 0, vec![
                ghost_corner(PinkyCorner, D0),
                wall(12, D0, O),
                corner(D90, O),
                corner(D0, O),
                wall(12, D0, O),
                ghost_corner(BlinkyCorner, D90),
            ]),
            create_field_line(2, 1, vec![
                wall(1, D270, O),
                dot(12),
                wall(1, D90, O),
                wall(1, D270, O),
                dot(12),
                wall(1, D90, O),
            ]),
            create_field_line(2, 2, vec![
                wall(1, D270, O),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(3, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D90, O),
                wall(1, D270, O),
                dot(1),
                corner(D0, I),
                wall(3, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 3, vec![
                wall(1, D270, O),
                energizer(),
                wall(1, D270, I),
                empty(2),
                wall(1, D90, I),
                dot(1),
                wall(1, D270, I),
                empty(3),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                wall(1, D270, O),
                dot(1),
                wall(1, D90, I),
                empty(3),
                wall(1, D270, I),
                dot(1),
                wall(1, D90, I),
                empty(2),
                wall(1, D270, I),
                energizer(),
                wall(1, D90, O),
            ]),
            create_field_line(2, 4, vec![
                wall(1, D270, O),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                wall(3, D180, I),
                corner(D180, I),
                dot(1),
                corner(D270, O),
                corner(D180, O),
                dot(1),
                corner(D270, I),
                wall(3, D180, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 5, vec![
                wall(1, D270, O),
                dot(26),
                wall(1, D90, O),
            ]),
            create_field_line(2, 6, vec![
                wall(1, D270, O),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(6, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 7, vec![
                wall(1, D270, O),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D90, I),
                corner(D0, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 8, vec![
                wall(1, D270, O),
                dot(6),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(6),
                wall(1, D90, O),
            ]),
            create_field_line(2, 9, vec![
                corner(D270, O),
                wall(4, D180, O),
                corner(D90, O),
                dot(1),
                wall(1, D90, I),
                corner(D270, I),
                wall(2, D0, I),
                corner(D90, I),
                empty(1),
                wall(2, D90, I),
                empty(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D180, I),
                wall(1, D90, I),
                dot(1),
                corner(D0, O),
                wall(4, D180, O),
                corner(D180, O),
            ]),
            create_field_line(2, 10, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D90, I),
                corner(D0, I),
                wall(2, D0, I),
                corner(D180, I),
                empty(1),
                corner(D270, I),
                corner(D180, I),
                empty(1),
                corner(D270, I),
                wall(2, D0, I),
                corner(D90, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(2, 11, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                empty(4),
                elem(2, BlinkySpawn),
                empty(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(2, 12, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                empty(1),
                corner(D0, G),
                wall(2, D0, G),
                elem(2, GhostHouseEntrance { rotation: D0 }),
                wall(2, D0, G),
                corner(D90, G),
                empty(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(0, 13, vec![
                elem(2, InvisibleWall),
                wall(5, D180, O),
                corner(D180, O),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                empty(1),
                wall(1, D90, G),
                elem(6, GhostHouse),
                wall(1, D90, G),
                empty(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                corner(D270, O),
                wall(5, D180, O),
                elem(2, InvisibleWall),
            ]),
            create_field_line(0, 14, vec![
                tunnel_left(),
                elem(1, TunnelEntrance),
                elem(6, TunnelHallway),
                empty(4),
                wall(1, D270, G),
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
                wall(1, D90, G),
                empty(4),
                elem(6, TunnelHallway),
                elem(1, TunnelEntrance),
                tunnel_right(),
            ]),
            create_field_line(0, 15, vec![
                elem(2, InvisibleWall),
                wall(5, D0, O),
                corner(D90, O),
                dot(1),
                corner(D0, I),
                corner(D90, I),
                empty(1),
                wall(1, D270, G),
                elem(6, GhostHouse),
                wall(1, D90, G),
                empty(1),
                corner(D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, O),
                wall(5, D0, O),
                elem(2, InvisibleWall),
            ]),
            create_field_line(2, 16, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                empty(1),
                corner(D270, G),
                wall(6, D180, G),
                corner(D180, G),
                empty(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(2, 17, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                empty(4),
                elem(2, FruitSpawn),
                empty(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(2, 18, vec![
                empty(5),
                wall(1, D270, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                empty(1),
                corner(D0, I),
                wall(6, D0, I),
                corner(D90, I),
                empty(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                wall(1, D90, O),
                empty(5),
            ]),
            create_field_line(2, 19, vec![
                corner(D0, O),
                wall(4, D180, O),
                corner(D180, O),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                empty(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D90, I),
                corner(D0, I),
                wall(2, D180, I),
                corner(D180, I),
                empty(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                corner(D270, O),
                wall(4, D180, O),
                corner(D90, O),
            ]),
            create_field_line(2, 20, vec![
                wall(1, D270, O),
                dot(12),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(12),
                wall(1, D90, O),
            ]),
            create_field_line(2, 21, vec![
                wall(1, D270, O),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(3, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D0, I),
                wall(3, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 22, vec![
                wall(1, D270, O),
                dot(1),
                corner(D270, I),
                wall(1, D0, I),
                corner(D90, I),
                wall(1, D90, I),
                dot(1),
                corner(D270, I),
                wall(3, D180, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                wall(3, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D90, I),
                corner(D0, I),
                wall(1, D0, I),
                corner(D180, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 23, vec![
                wall(1, D270, O),
                energizer(),
                dot(2),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(7),
                elem(2, PacManSpawn),
                dot(7),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(2),
                energizer(),
                wall(1, D90, O),
            ]),
            create_field_line(2, 24, vec![
                corner(D270, O),
                wall(1, D0, O),
                corner(D90, O),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                wall(6, D0, I),
                corner(D90, I),
                dot(1),
                corner(D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D0, O),
                wall(1, D0, O),
                corner(D180, O),
            ]),
            create_field_line(2, 25, vec![
                corner(D0, O),
                wall(1, D180, O),
                corner(D180, O),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D270, I),
                wall(2, D180, I),
                corner(D90, I),
                corner(D0, I),
                wall(2, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                corner(D270, O),
                wall(1, D180, O),
                corner(D90, O),
            ]),
            create_field_line(2, 26, vec![
                wall(1, D270, O),
                dot(6),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(4),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(6),
                wall(1, D90, O),
            ]),
            create_field_line(2, 27, vec![
                wall(1, D270, O),
                dot(1),
                corner(D0, I),
                wall(4, D0, I),
                corner(D180, I),
                corner(D270, I),
                wall(2, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D270, I),
                wall(1, D90, I),
                dot(1),
                corner(D0, I),
                wall(2, D0, I),
                corner(D180, I),
                corner(D270, I),
                wall(4, D0, I),
                corner(D90, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 28, vec![
                wall(1, D270, O),
                dot(1),
                corner(D270, I),
                wall(8, D180, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                corner(D180, I),
                dot(1),
                corner(D270, I),
                wall(8, D180, I),
                corner(D180, I),
                dot(1),
                wall(1, D90, O),
            ]),
            create_field_line(2, 29, vec![
                wall(1, D270, O),
                dot(26),
                wall(1, D90, O),
            ]),
            create_field_line(2, 30, vec![
                ghost_corner(ClydeCorner, D270),
                wall(26, D180, O),
                ghost_corner(InkyCorner, D180),
            ]),
        ];

        let mut flat_fields = fields.into_iter()
            .enumerate()
            .inspect(|(i, vec)| {
                println!("{i}");
                assert!(vec.len() == 28 || vec.len() == 32)
            })
            .flat_map(|(_, f)| f)
            .collect::<Vec<_>>();

        let height = flat_fields.iter()
            .map(|f| f.position.x)
            .collect::<HashSet<_>>()
            .len();

        flat_fields.iter_mut()
            .for_each(|f| {
                f.position.y = (height as isize) - 2 - f.position.y
            });

        let json = serde_json::to_string(&flat_fields).unwrap();
        let mut file = OpenOptions::new().truncate(true).write(true).open("./maps/new_map.json").unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    fn create_field_line(start_x: isize, y: isize, elements: Vec<Vec<Vec<Element>>>) -> Vec<Field> {
        elements.into_iter()
            .flat_map(|i| i)
            .enumerate()
            .map(|(i, elems)| Field {
                position: Position::new(start_x + (i as isize), y),
                elements: elems,
            })
            .collect()
    }

    fn wall(amount: usize, rotation: Rotation, wall_type: QuickWall) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(move |_| vec![Wall {
                wall_type: wall_type.to_wall(),
                rotation,
                is_corner: false,
            }])
            .collect()
    }

    fn corner(rotation: Rotation, wall_type: QuickWall) -> Vec<Vec<Element>> {
        vec![vec![Wall {
            wall_type: wall_type.to_wall(),
            is_corner: true,
            rotation
        }]]
    }

    fn ghost_corner(ghost_corner: Element, rotation: Rotation) -> Vec<Vec<Element>> {
        let mut res = corner(rotation, O);
        res[0].push(ghost_corner);
        res
    }

    fn dot(amount: usize) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![DotSpawn])
            .collect()
    }

    fn empty(amount: usize) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![])
            .collect()
    }

    fn energizer() -> Vec<Vec<Element>> {
        vec![vec![EnergizerSpawn]]
    }

    fn elem(amount: usize, elem: Element) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![elem.clone()])
            .collect()
    }

    fn elems(on_field: Vec<Element>) -> Vec<Vec<Element>> {
        vec![on_field]
    }

    fn tunnel_right() -> Vec<Vec<Element>> {
        vec![vec![Tunnel {
            index: 0,
            opening_direction: Direction::Right,
        }]]
    }

    fn tunnel_left() -> Vec<Vec<Element>> {
        vec![vec![Tunnel {
            index: 0,
            opening_direction: Direction::Left,
        }]]
    }
}