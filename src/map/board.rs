use std::collections::HashMap;

use crate::common::{MoveDirection, Position};
use crate::common::MoveDirection::*;
use crate::constants::MAP_PATH;
use crate::map::Neighbour;
use crate::map::{Element, Map};

static EMPTY: Vec<Element> = vec![];

#[derive(Debug)]
pub struct Board {
    elements_map: HashMap<Position, Vec<Element>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new() -> Self {
        let map = Map::load(MAP_PATH);
        let width = map.get_width();
        let height = map.get_height();

        let elements_map = map.fields.into_iter()
            .map(|f| (f.position, f.elements))
            .collect();

        Board {
            elements_map,
            width,
            height,
        }
    }

    /// Return the position of one specific field type. Of the FieldType
    /// should be exactly one on the map. If not, the program panics.
    pub fn get_position_matching(&self, filter: impl Fn(&Element) -> bool) -> &Position {
        let positions = self.elements_map.iter()
            .filter(|(_, elems)| Self::elements_match_filter(elems, &filter))
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
        match positions.len() {
            1 => positions[0],
            _ => panic!("Expected exactly one field")
        }
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

    /// Returns the first element on the given position matching the given filter.
    ///
    /// Returns None if
    /// - the position is not on the board
    /// - no element matches the filter
    pub fn element_on_position_matching(&self, position: &Position, filter: impl Fn(&Element) -> bool) -> Option<&Element> {
        self.elements_map.get(position)?
            .into_iter()
            .filter(|e| (filter)(e))
            .next()
    }

    /// Return the elements on the given position.
    ///
    /// If the position does not exists in the map, return a reference to an empty
    /// vector.
    pub fn elements_on_position(&self, position: &Position) -> &Vec<Element> {
        self.elements_map.get(position).unwrap_or(&EMPTY)
    }

    pub fn neighbours_of(&self, position: &Position) -> Vec<Neighbour> {
        let neighbour_position_options = vec![
            (Up, position.neighbour_position(&Up)),
            (Down, position.neighbour_position(&Down)),
            (Left, position.neighbour_position(&Left)),
            (Right, position.neighbour_position(&Right)),
        ];
        neighbour_position_options.into_iter()
            .map(|(dir, pos)| Neighbour::new(pos, self.elements_on_position(&pos), dir))
            .collect()
    }

    pub fn neighbour_behind(&self, position: &Position, direction: &MoveDirection) -> Neighbour {
        let pos = position.neighbour_position(&direction.opposite());
        Neighbour::new(pos, self.elements_on_position(&pos), direction.opposite())
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