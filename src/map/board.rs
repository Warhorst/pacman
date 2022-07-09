use bevy::utils::HashSet;

use crate::common::position::Position;
use crate::map::{Element, Map};
use crate::map::Element::*;
use crate::is;

/// Resource that knows where specific board elements are located which never change position.
/// It also knows the dimension of our board.
///
/// It should only be used for collision ("am I hitting a wall?") and area ("am I in a tunnel right now?") checks.
#[derive(Debug)]
pub struct Board {
    wall_positions: HashSet<Position>,
    ghost_house_entrance_positions: HashSet<Position>,
    tunnel_positions: HashSet<Position>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(map: &Map) -> Self {
        let width = map.get_width();
        let height = map.get_height();
        let wall_positions = Self::positions_matching_filter(&map, is!(Wall {..} | InvisibleWall));
        let ghost_house_entrance_positions = Self::positions_matching_filter(&map, is!(GhostHouseEntrance {..}));
        let tunnel_positions = Self::positions_matching_filter(&map, is!(Tunnel {..} | TunnelEntrance | TunnelHallway));

        Board {
            wall_positions,
            ghost_house_entrance_positions,
            tunnel_positions,
            width,
            height,
        }
    }

    fn positions_matching_filter(map: &Map, filter: impl Fn(&Element) -> bool) -> HashSet<Position> {
        map.get_positions_matching(filter)
            .into_iter()
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn position_is_wall(&self, pos: &Position) -> bool {
        self.wall_positions.contains(pos)
    }

    pub fn position_is_ghost_house_entrance(&self, pos: &Position) -> bool {
        self.ghost_house_entrance_positions.contains(pos)
    }

    /// Returns true if a position is a wall or a ghost house entrance, else false.
    ///
    /// These checks are often used in combination, so this method checks them both in one call.
    pub fn position_is_wall_or_entrance(&self, pos: &Position) -> bool {
        self.position_is_wall(pos) || self.position_is_ghost_house_entrance(pos)
    }

    pub fn position_is_tunnel(&self, pos: &Position) -> bool {
        self.tunnel_positions.contains(pos)
    }
}