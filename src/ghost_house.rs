use std::collections::HashSet;
use bevy::prelude::*;
use crate::common::Position;
use crate::{is, map};
use crate::map::board::Board;
use map::Element::*;

pub struct GhostHousePlugin;

impl Plugin for GhostHousePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ghost_house);
    }
}

/// Resource that knows where the ghost house and its entrances are.
/// The walls around a ghost house are not considered part of the ghost house.
pub struct GhostHousePositions {
    entrances: HashSet<Position>,
    interior: HashSet<Position>
}

impl GhostHousePositions {
    fn new<'a, E: IntoIterator<Item=&'a Position>, I: IntoIterator<Item=&'a Position>>(entrance_iter: E, interior_iter: I) -> Self {
        let entrances = entrance_iter.into_iter().map(|p| *p).collect();
        let interior = interior_iter.into_iter().map(|p| *p).collect();

        GhostHousePositions {
            entrances, interior
        }
    }

    pub fn position_is_entrance(&self, pos: &Position) -> bool {
        self.entrances.contains(pos)
    }

    pub fn position_is_interior(&self, pos: &Position) -> bool {
        self.interior.contains(pos)
    }
}

fn spawn_ghost_house(
    mut commands: Commands,
    board: Res<Board>
) {
    let ghost_house_positions = GhostHousePositions::new(
        board.get_positions_matching(is!(GhostHouseEntrance {..})),
        board.get_positions_matching(is!(GhostHouse))
    );
    commands.insert_resource(ghost_house_positions);
}