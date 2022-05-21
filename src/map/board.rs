use std::collections::HashMap;

use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::common::MoveDirection::*;
use crate::constants::{FIELD_DIMENSION, MAP_PATH};
use crate::map::Neighbour;
use crate::map::{Element, Map};
use crate::map::Element::*;

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

    // TODO: Maybe move to Position
    pub fn coordinates_of_position(position: &Position) -> Vec3 {
        let x = (position.x() as f32) * FIELD_DIMENSION;
        let y = (position.y() as f32) * FIELD_DIMENSION;
        Vec3::new(x, y, 0.0)
    }

    // TODO: Maybe move to Position
    pub fn position_of_coordinates(coordinates: &Vec3) -> Position {
        let x = (coordinates.x + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        let y = (coordinates.y + FIELD_DIMENSION / 2.0) / FIELD_DIMENSION;
        Position::new(x as usize, y as usize)
    }

    // TODO: Maybe move to Position
    pub fn position_in_direction(&self, position: &Position, direction: &MoveDirection) -> Option<Position> {
        match direction {
            Up => self.position_up_of(position),
            Down => self.position_down_of(position),
            Left => self.position_left_of(position),
            Right => self.position_right_of(position),
        }
    }

    fn position_up_of(&self, position: &Position) -> Option<Position> {
        match position.y() {
            y if y < self.height - 1 => Some(Position::new(position.x(), y + 1)),
            _ => None
        }
    }

    fn position_down_of(&self, position: &Position) -> Option<Position> {
        match position.y() {
            y if y > 0 => Some(Position::new(position.x(), y - 1)),
            _ => None
        }
    }

    fn position_left_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            x if x > 0 => Some(Position::new(x - 1, position.y())),
            _ => None
        }
    }

    fn position_right_of(&self, position: &Position) -> Option<Position> {
        match position.x() {
            x if x < self.width - 1 => Some(Position::new(x + 1, position.y())),
            _ => None
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
            (Up, self.position_up_of(position)),
            (Down, self.position_down_of(position)),
            (Left, self.position_left_of(position)),
            (Right, self.position_right_of(position)),
        ];
        neighbour_position_options.into_iter()
            .filter(|(_, option)| option.is_some())
            .map(|(dir, option)| match option {
                Some(pos) => Neighbour::new(pos, self.elements_on_position(&pos), dir),
                None => panic!()
            })
            .collect()
    }

    pub fn neighbour_behind(&self, position: &Position, direction: &MoveDirection) -> Neighbour {
        let position = (match direction.opposite() {
            Up => self.position_up_of(position),
            Down => self.position_down_of(position),
            Left => self.position_left_of(position),
            Right => self.position_right_of(position)
        }).unwrap();

        Neighbour::new(position, self.elements_on_position(&position), direction.opposite())
    }

    pub fn position_is_tunnel(&self, position: &Position) -> bool {
        self.elements_on_position(position).into_iter()
            .map(|e| match e {
                Tunnel {..} | TunnelEntrance | TunnelHallway => true,
                _ => false
            })
            .min()
            .unwrap_or(false)
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