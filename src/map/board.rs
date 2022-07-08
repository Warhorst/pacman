use std::collections::HashMap;
use bevy::utils::HashSet;

use crate::common::position::Position;
use crate::constants::MAP_PATH;
use crate::map::{Element, Map};
use crate::map::Element::*;
use crate::Vec3;
use crate::common::Direction::*;
use crate::is;

static EMPTY: Vec<Element> = vec![];

#[derive(Debug)]
pub struct Board {
    elements_map: HashMap<Position, Vec<Element>>,
    wall_positions: HashSet<Position>,
    ghost_house_entrance_positions: HashSet<Position>,
    tunnel_positions: HashSet<Position>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new() -> Self {
        let map = Map::load(MAP_PATH);
        let width = map.get_width();
        let height = map.get_height();

        let wall_positions = Self::positions_matching_filter(&map, is!(Wall {..} | InvisibleWall));
        let ghost_house_entrance_positions = Self::positions_matching_filter(&map, is!(GhostHouseEntrance {..}));
        let tunnel_positions = Self::positions_matching_filter(&map, is!(Tunnel {..} | TunnelEntrance | TunnelHallway));

        let elements_map = map.fields.into_iter()
            .map(|f| (f.position, f.elements))
            .collect();

        Board {
            elements_map,
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

    pub fn position_is_tunnel(&self, pos: &Position) -> bool {
        self.tunnel_positions.contains(pos)
    }

    pub fn get_positions_matching(&self, filter: impl Fn(&Element) -> bool) -> Vec<&Position> {
        self.elements_map.iter()
            .filter(|(_, elems)| Self::elements_match_filter(elems, &filter))
            .map(|(pos, _)| pos)
            .collect()
    }

    /// Check if the given position matches the given element filter
    pub fn position_matches_filter(&self, position: &Position, filter: impl Fn(&Element) -> bool) -> bool {
        Self::elements_match_filter(self.elements_on_position(position), &filter)
    }

    fn elements_match_filter(elems: &Vec<Element>, filter: &impl Fn(&Element) -> bool) -> bool {
        elems.into_iter()
            .map(filter)
            .max()
            .unwrap_or(false)
    }

    /// Return the elements on the given position.
    ///
    /// If the position does not exists in the map, return a reference to an empty
    /// vector.
    pub fn elements_on_position(&self, position: &Position) -> &Vec<Element> {
        self.elements_map.get(position).unwrap_or(&EMPTY)
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
        let positions_matching_filter = self.get_positions_matching(filter);

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

#[cfg(test)]
mod tests {
    use crate::map::board::Board;

    #[test]
    fn creation_works() {
        let board = Board::new();
        print!("{:?}", board);
    }
}