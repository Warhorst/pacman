use std::collections::HashMap;

use crate::common::Position;
use crate::constants::MAP_PATH;
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