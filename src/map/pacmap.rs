use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Read};

use crate::common::Position;
use crate::map::{FieldType, FieldTypeMatrix, PositionTypeMap};

/// Holds width, height and a matrix with all fields of the map.
#[derive(PartialOrd, PartialEq, Debug)]
pub(in crate::map) struct PacMap {
    pub field_types: FieldTypeMatrix,
    pub width: usize,
    pub height: usize
}

impl PacMap {
    pub fn from_read<R: Read>(read: R) -> Self {
        let reader = BufReader::new(read);
        let lines = reader.lines()
            .map(|line_result| line_result.expect("Error reading pacmap"))
            .collect::<Vec<String>>();
        let (width, height) = Self::read_width_and_height(&lines);
        let field_types = Self::lines_to_field_type_matrix(lines, width);

        PacMap {
            field_types,
            width,
            height
        }
    }

    fn read_width_and_height(lines: &Vec<String>) -> (usize, usize) {
        let width = lines.iter()
            .map(|line| line.len())
            .max()
            .expect("The map should at least contain one row");
        let height = lines.len();
        (width, height)
    }

    fn lines_to_field_type_matrix(lines: Vec<String>, width: usize) -> FieldTypeMatrix {
        lines.into_iter()
            .map(|line| Self::line_to_field_types(line, width))
            .collect::<Vec<_>>()
    }

    /// Converts a line into a Vec of Field types.
    /// If the line is shorter than the width, the remaining space is
    /// filled with FieldType::Free.
    fn line_to_field_types(line: String, width: usize) -> Vec<FieldType> {
        let mut field_types = line.chars()
            .map(|char| FieldType::try_from(char).expect("The map should contain only valid characters."))
            .collect::<Vec<_>>();
        while field_types.len() < width {
            field_types.push(FieldType::Free)
        }
        field_types
    }

    /// Transforms this pacmap into a map of a Position to a FieldType.
    pub fn into_position_type_map(self) -> PositionTypeMap {
        let mut position_type_map = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                position_type_map.insert(Position::new(x, self.height - 1 - y), self.field_types[y][x]);
            }
        }
        position_type_map
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Position;
    use crate::map::FieldType::*;
    use crate::map::pacmap::PacMap;

    #[test]
    fn map_from_read_successful() {
        let map_as_string = "WWWWW\nW W W\nL   R\nW   W\nWWWWW";
        let field_types = vec![
            vec![Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Free, Wall, Free, Wall],
            vec![LeftTunnel, Free, Free, Free, RightTunnel],
            vec![Wall, Free, Free, Free, Wall],
            vec![Wall, Wall, Wall, Wall, Wall],
        ];
        let width = 5;
        let height = 5;
        let expected = PacMap {
            field_types,
            width,
            height
        };
        assert_eq!(expected, PacMap::from_read(map_as_string.as_bytes()))
    }

    #[test]
    fn into_position_type_map_successful() {
        let pacmap = PacMap {
            field_types: vec![
                vec![Wall, Wall, RightTunnel],
                vec![Wall, Free, Wall],
                vec![LeftTunnel, Wall, Wall],
            ],
            width: 3,
            height: 3
        };
        let expected_mappings = vec![
            (Position::new(0, 0), LeftTunnel),
            (Position::new(1, 0), Wall),
            (Position::new(2, 0), Wall),
            (Position::new(0, 1), Wall),
            (Position::new(1, 1), Free),
            (Position::new(2, 1), Wall),
            (Position::new(0, 2), Wall),
            (Position::new(1, 2), Wall),
            (Position::new(2, 2), RightTunnel),
        ];
        let position_type_map = pacmap.into_position_type_map();
        expected_mappings.into_iter()
            .for_each(|(p, t)| assert_eq!(&t, position_type_map.get(&p).unwrap()))
    }
}