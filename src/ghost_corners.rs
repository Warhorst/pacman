use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::common::position::Position;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::life_cycle::LifeCycle::Start;
use crate::map::TileMap;

pub struct GhostCornersPlugin;

impl Plugin for GhostCornersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(create_ghost_corners)
            )
        ;
    }
}

pub struct GhostCorners {
    corners: HashMap<Ghost, Position>,
}

impl GhostCorners {
    fn new(map: &TileMap) -> Self {
        GhostCorners {
            corners: [
                (Blinky, map.blinky_corner),
                (Pinky, map.pinky_corner),
                (Inky, map.inky_corner),
                (Clyde, map.clyde_corner),
            ].into_iter().collect()
        }
    }

    pub fn get_corner(&self, ghost: &Ghost) -> Position {
        *self.corners.get(ghost).expect("every ghost should have a corner")
    }
}

fn create_ghost_corners(
    mut commands: Commands,
    map: Res<TileMap>,
) {
    commands.insert_resource(GhostCorners::new(&map));
}