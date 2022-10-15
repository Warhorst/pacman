use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::common::position::Position;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::is;
use crate::life_cycle::LifeCycle::Start;
use crate::map::{Element, Map};
use crate::map::Element::{BlinkyCorner, ClydeCorner, InkyCorner, PinkyCorner};

pub struct GhostCornersPlugin;

impl Plugin for GhostCornersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_ghost_corners)
            )
        ;
    }
}

pub struct GhostCorners {
    corners: HashMap<Ghost, Position>,
}

impl GhostCorners {
    fn new(map: &Map) -> Self {
        GhostCorners {
            corners: [
                (Blinky, Self::get_corner_position(map, is!(BlinkyCorner))),
                (Pinky, Self::get_corner_position(map, is!(PinkyCorner))),
                (Inky, Self::get_corner_position(map, is!(InkyCorner))),
                (Clyde, Self::get_corner_position(map, is!(ClydeCorner))),
            ].into_iter().collect()
        }
    }

    fn get_corner_position(map: &Map, filter: impl Fn(&Element) -> bool) -> Position {
        *map.get_positions_matching(filter).into_iter().next().expect("every ghost should have a corner")
    }

    pub fn get_corner(&self, ghost: &Ghost) -> Position {
        *self.corners.get(ghost).expect("every ghost should have a corner")
    }
}

fn spawn_ghost_corners(
    mut commands: Commands,
    map: Res<Map>,
) {
    commands.insert_resource(GhostCorners::new(&map));
}