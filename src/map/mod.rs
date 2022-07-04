use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::common::Direction;
use crate::common::position::Position;
use crate::map::board::Board;

pub mod board;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new());
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

impl Map {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).expect("could not open map from given path");
        serde_json::from_reader(file).expect("could not parse map from json")
    }

    pub fn get_width(&self) -> usize {
        self.fields.iter()
            .map(|f| f.position.x)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn get_height(&self) -> usize {
        self.fields.iter()
            .map(|f| f.position.y)
            .collect::<HashSet<_>>()
            .len()
    }
}

/// This bullshit is only used to generate the json map until I have a better way to do this
#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::Write;

    use crate::common::position::Position;
    use crate::common::Direction;
    use crate::map::{Element, Field, Map, Rotation, WallType};
    use crate::map::Element::*;
    use crate::map::Rotation::*;

    #[test]
    fn from_json() {
        serde_json::from_reader::<_, Map>(File::open("./maps/new_map.json").unwrap()).expect("Failed to deserialize map");
    }

    #[test]
    fn to_json() {
        let fields = vec![
            create_field_line(2, 0, vec![
                ghost_corner(PinkyCorner, D0),
                wall(12, D0),
                corner(D90),
                corner(D0),
                wall(12, D0),
                ghost_corner(BlinkyCorner, D90),
            ]),
            create_field_line(2, 1, vec![
                wall(1, D270),
                dot(12),
                wall(1, D90),
                wall(1, D270),
                dot(12),
                wall(1, D90),
            ]),
            create_field_line(2, 2, vec![
                wall(1, D270),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(3, D0),
                corner(D90),
                dot(1),
                wall(1, D90),
                wall(1, D270),
                dot(1),
                corner(D0),
                wall(3, D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 3, vec![
                wall(1, D270),
                energizer(),
                wall(1, D270),
                empty(2),
                wall(1, D90),
                dot(1),
                wall(1, D270),
                empty(3),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                wall(1, D270),
                dot(1),
                wall(1, D90),
                empty(3),
                wall(1, D270),
                dot(1),
                wall(1, D90),
                empty(2),
                wall(1, D270),
                energizer(),
                wall(1, D90),
            ]),
            create_field_line(2, 4, vec![
                wall(1, D270),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D180),
                dot(1),
                corner(D270),
                wall(3, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(3, D180),
                corner(D180),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D180),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 5, vec![
                wall(1, D270),
                dot(26),
                wall(1, D90),
            ]),
            create_field_line(2, 6, vec![
                wall(1, D270),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                corner(D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(6, D0),
                corner(D90),
                dot(1),
                corner(D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 7, vec![
                wall(1, D270),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D180),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D90),
                corner(D0),
                wall(2, D180),
                corner(D180),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D180),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 8, vec![
                wall(1, D270),
                dot(6),
                wall(1, D270),
                wall(1, D90),
                dot(4),
                wall(1, D270),
                wall(1, D90),
                dot(4),
                wall(1, D270),
                wall(1, D90),
                dot(6),
                wall(1, D90),
            ]),
            create_field_line(2, 9, vec![
                corner(D270),
                wall(4, D180),
                corner(D90),
                dot(1),
                wall(1, D90),
                corner(D270),
                wall(2, D0),
                corner(D90),
                empty(1),
                wall(2, D90),
                empty(1),
                corner(D0),
                wall(2, D0),
                corner(D180),
                wall(1, D90),
                dot(1),
                corner(D0),
                wall(4, D180),
                corner(D180),
            ]),
            create_field_line(2, 10, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D90),
                corner(D0),
                wall(2, D0),
                corner(D180),
                empty(1),
                corner(D270),
                corner(D180),
                empty(1),
                corner(D270),
                wall(2, D0),
                corner(D90),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(2, 11, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                empty(4),
                elem(2, BlinkySpawn),
                empty(4),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(2, 12, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                empty(1),
                corner(D0),
                wall(2, D0),
                elem(2, GhostHouseEntrance { rotation: D0 }),
                wall(2, D0),
                corner(D90),
                empty(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(0, 13, vec![
                elem(2, InvisibleWall),
                wall(5, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                empty(1),
                wall(1, D90),
                elem(6, GhostHouse),
                wall(1, D90),
                empty(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(5, D180),
                elem(2, InvisibleWall),
            ]),
            create_field_line(0, 14, vec![
                tunnel_left(),
                elem(1, TunnelEntrance),
                elem(6, TunnelHallway),
                empty(4),
                wall(1, D270),
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
                wall(1, D90),
                empty(4),
                elem(6, TunnelHallway),
                elem(1, TunnelEntrance),
                tunnel_right(),
            ]),
            create_field_line(0, 15, vec![
                elem(2, InvisibleWall),
                wall(5, D0),
                corner(D90),
                dot(1),
                corner(D0),
                corner(D90),
                empty(1),
                wall(1, D270),
                elem(6, GhostHouse),
                wall(1, D90),
                empty(1),
                corner(D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(5, D0),
                elem(2, InvisibleWall),
            ]),
            create_field_line(2, 16, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                empty(1),
                corner(D270),
                wall(6, D180),
                corner(D180),
                empty(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(2, 17, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                empty(4),
                elem(2, FruitSpawn),
                empty(4),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(2, 18, vec![
                empty(5),
                wall(1, D270),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                empty(1),
                corner(D0),
                wall(6, D0),
                corner(D90),
                empty(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                wall(1, D90),
                empty(5),
            ]),
            create_field_line(2, 19, vec![
                corner(D0),
                wall(4, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                empty(1),
                corner(D270),
                wall(2, D180),
                corner(D90),
                corner(D0),
                wall(2, D180),
                corner(D180),
                empty(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(4, D180),
                corner(D90),
            ]),
            create_field_line(2, 20, vec![
                wall(1, D270),
                dot(12),
                wall(1, D270),
                wall(1, D90),
                dot(12),
                wall(1, D90),
            ]),
            create_field_line(2, 21, vec![
                wall(1, D270),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(3, D0),
                corner(D90),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D0),
                wall(3, D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D90),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 22, vec![
                wall(1, D270),
                dot(1),
                corner(D270),
                wall(1, D0),
                corner(D90),
                wall(1, D90),
                dot(1),
                corner(D270),
                wall(3, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(3, D180),
                corner(D180),
                dot(1),
                wall(1, D90),
                corner(D0),
                wall(1, D0),
                corner(D180),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 23, vec![
                wall(1, D270),
                energizer(),
                dot(2),
                wall(1, D270),
                wall(1, D90),
                dot(7),
                elem(2, PacManSpawn),
                dot(7),
                wall(1, D270),
                wall(1, D90),
                dot(2),
                energizer(),
                wall(1, D90),
            ]),
            create_field_line(2, 24, vec![
                corner(D270),
                wall(1, D0),
                corner(D90),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D0),
                corner(D90),
                dot(1),
                corner(D0),
                wall(6, D0),
                corner(D90),
                dot(1),
                corner(D0),
                corner(D90),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D0),
                wall(1, D0),
                corner(D180),
            ]),
            create_field_line(2, 25, vec![
                corner(D0),
                wall(1, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D270),
                wall(2, D180),
                corner(D90),
                corner(D0),
                wall(2, D180),
                corner(D180),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(1, D180),
                corner(D90),
            ]),
            create_field_line(2, 26, vec![
                wall(1, D270),
                dot(6),
                wall(1, D270),
                wall(1, D90),
                dot(4),
                wall(1, D270),
                wall(1, D90),
                dot(4),
                wall(1, D270),
                wall(1, D90),
                dot(6),
                wall(1, D90),
            ]),
            create_field_line(2, 27, vec![
                wall(1, D270),
                dot(1),
                corner(D0),
                wall(4, D0),
                corner(D180),
                corner(D270),
                wall(2, D0),
                corner(D90),
                dot(1),
                wall(1, D270),
                wall(1, D90),
                dot(1),
                corner(D0),
                wall(2, D0),
                corner(D180),
                corner(D270),
                wall(4, D0),
                corner(D90),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 28, vec![
                wall(1, D270),
                dot(1),
                corner(D270),
                wall(8, D180),
                corner(D180),
                dot(1),
                corner(D270),
                corner(D180),
                dot(1),
                corner(D270),
                wall(8, D180),
                corner(D180),
                dot(1),
                wall(1, D90),
            ]),
            create_field_line(2, 29, vec![
                wall(1, D270),
                dot(26),
                wall(1, D90),
            ]),
            create_field_line(2, 30, vec![
                ghost_corner(ClydeCorner, D270),
                wall(26, D180),
                ghost_corner(InkyCorner, D180),
            ]),
        ];

        let mut map = Map {
            fields: fields.into_iter()
                .enumerate()
                .inspect(|(i, vec)| {
                    println!("{i}");
                    assert!(vec.len() == 28 || vec.len() == 32)
                })
                .flat_map(|(_, f)| f)
                .collect()
        };

        let height = map.get_height();

        map.fields.iter_mut()
            .for_each(|f| {
                f.position.y = (height as isize) - 1 - f.position.y
            });

        let json = serde_json::to_string(&map).unwrap();
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

    fn wall(amount: usize, rotation: Rotation) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![Wall {
                wall_type: WallType::Outer,
                rotation,
                is_corner: false,
            }])
            .collect()
    }

    fn corner(rotation: Rotation) -> Vec<Vec<Element>> {
        vec![vec![Wall {
            wall_type: WallType::Outer,
            is_corner: true,
            rotation
        }]]
    }

    fn ghost_corner(ghost_corner: Element, rotation: Rotation) -> Vec<Vec<Element>> {
        let mut res = corner(rotation);
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