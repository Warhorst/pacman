use serde::{Serialize, Deserialize};
use crate::common::{MoveDirection, Position};

#[derive(Serialize, Deserialize)]
pub struct RawMap {
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
struct Field {
    position: Position,
    elements: Vec<Element>,
}

#[derive(Clone, Serialize, Deserialize)]
enum Element {
    Empty,
    Wall {
        wall_type: WallType,
        rotation: f32,
        is_corner: bool,
    },
    GhostHouseEntrance {
        rotation: f32
    },
    GhostHouse,
    PacManSpawn,
    GhostSpawn,
    DotSpawn,
    EnergizerSpawn,
    BlinkySpawn,
    PinkySpawn,
    InkySpawn,
    ClydeSpawn,
    BlinkyCorner,
    PinkyCorner,
    InkyCorner,
    ClydeCorner,
    Tunnel {
        index: usize,
        opening_direction: MoveDirection,
    },
    TunnelEntrance,
    TunnelHallway,
    InvisibleWall,
}

#[derive(Clone, Serialize, Deserialize)]
enum WallType {
    Outer,
    Inner,
    Ghost,
}

/// This bullshit is only used to generate the json map until I have a better way to do this
#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use crate::common::{MoveDirection, Position};
    use crate::map::new_map::{Element, Field, RawMap, WallType};
    use crate::map::new_map::Element::*;

    #[test]
    fn from_json() {
        serde_json::from_reader::<_, RawMap>(File::open("./maps/new_map.json").unwrap()).expect("Failed to deserialize map");
    }

    #[test]
    fn to_json() {
        let fields = vec![
            create_field_line(2, 0, vec![
                wall_corner(PinkySpawn),
                wall(26),
                wall_corner(BlinkyCorner),
            ]),
            create_field_line(2, 1, vec![
                wall(1),
                dot(12),
                wall(2),
                dot(12),
                wall(1),
            ]),
            create_field_line(2, 2, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 3, vec![
                wall(1),
                energizer(),
                wall(4),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(4),
                energizer(),
                wall(1),
            ]),
            create_field_line(2, 4, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 5, vec![
                wall(1),
                dot(26),
                wall(1),
            ]),
            create_field_line(2, 6, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(2),
                dot(1),
                wall(8),
                dot(1),
                wall(2),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 7, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(2),
                dot(1),
                wall(8),
                dot(1),
                wall(2),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 8, vec![
                wall(1),
                dot(6),
                wall(2),
                dot(4),
                wall(2),
                dot(4),
                wall(2),
                dot(6),
                wall(1),
            ]),
            create_field_line(2, 9, vec![
                wall(6),
                dot(1),
                wall(5),
                empty(1),
                wall(2),
                empty(1),
                wall(5),
                dot(1),
                wall(6),
            ]),
            create_field_line(2, 10, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(1),
                empty(5),
            ]),
            create_field_line(2, 11, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(2),
                empty(5),
                elem(1, BlinkySpawn),
                empty(4),
                wall(2),
                dot(1),
                wall(1),
                empty(5),
            ]),
            create_field_line(2, 12, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(2),
                empty(1),
                wall(3),
                elem(2, GhostHouseEntrance { rotation: 0.0 }),
                wall(3),
                empty(1),
                wall(2),
                dot(1),
                wall(1),
                empty(5),
            ]),
            create_field_line(0, 13, vec![
                elem(2, InvisibleWall),
                wall(6),
                dot(1),
                wall(2),
                empty(1),
                wall(1),
                elem(6, GhostHouse),
                wall(1),
                empty(1),
                wall(2),
                dot(1),
                wall(6),
                elem(2, InvisibleWall),
            ]),
            create_field_line(0, 14, vec![
                tunnel_left(),
                elem(1, TunnelEntrance),
                elem(6, TunnelHallway),
                empty(4),
                wall(1),
                elem(1, GhostHouse),
                elems(vec![GhostHouse, InkySpawn]),
                elem(1, GhostHouse),
                elems(vec![GhostHouse, PinkySpawn]),
                elem(1, GhostHouse),
                elems(vec![GhostHouse, ClydeSpawn]),
                wall(1),
                empty(4),
                elem(6, TunnelHallway),
                elem(1, TunnelEntrance),
                tunnel_right(),
            ]),
            create_field_line(0, 15, vec![
                elem(2, InvisibleWall),
                wall(6),
                dot(1),
                wall(2),
                empty(1),
                wall(1),
                elem(6, GhostHouse),
                wall(1),
                empty(1),
                wall(2),
                dot(1),
                wall(6),
                elem(2, InvisibleWall),
            ]),
            create_field_line(2, 16, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(2),
                empty(1),
                wall(8),
                empty(1),
                wall(2),
                dot(1),
                wall(1),
                wall(5),
            ]),
            create_field_line(2, 17, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(2),
                empty(10),
                wall(2),
                dot(1),
                wall(1),
                wall(5),
            ]),
            create_field_line(2, 18, vec![
                empty(5),
                wall(1),
                dot(1),
                wall(2),
                empty(1),
                wall(8),
                empty(1),
                wall(2),
                dot(1),
                wall(1),
                wall(5),
            ]),
            create_field_line(2, 19, vec![
                wall(6),
                dot(1),
                wall(2),
                empty(1),
                wall(8),
                empty(1),
                wall(2),
                dot(1),
                wall(6),
            ]),
            create_field_line(2, 20, vec![
                wall(1),
                dot(12),
                wall(2),
                dot(12),
                wall(1),
            ]),
            create_field_line(2, 21, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 22, vec![
                wall(1),
                dot(1),
                wall(4),
                dot(1),
                wall(5),
                dot(1),
                wall(2),
                dot(1),
                wall(5),
                dot(1),
                wall(4),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 23, vec![
                wall(1),
                energizer(),
                dot(2),
                wall(2),
                dot(7),
                empty(1),
                elem(1, PacManSpawn),
                dot(7),
                wall(2),
                dot(2),
                energizer(),
                wall(1),
            ]),
            create_field_line(2, 24, vec![
                wall(3),
                dot(1),
                wall(2),
                dot(1),
                wall(2),
                dot(1),
                wall(8),
                dot(1),
                wall(2),
                dot(1),
                wall(2),
                dot(1),
                wall(3),
            ]),
            create_field_line(2, 25, vec![
                wall(3),
                dot(1),
                wall(2),
                dot(1),
                wall(2),
                dot(1),
                wall(8),
                dot(1),
                wall(2),
                dot(1),
                wall(2),
                dot(1),
                wall(3),
            ]),
            create_field_line(2, 26, vec![
                wall(1),
                dot(6),
                wall(2),
                dot(4),
                wall(2),
                dot(4),
                wall(2),
                dot(6),
                wall(1),
            ]),
            create_field_line(2, 27, vec![
                wall(1),
                dot(1),
                wall(10),
                dot(1),
                wall(2),
                dot(1),
                wall(10),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 28, vec![
                wall(1),
                dot(1),
                wall(10),
                dot(1),
                wall(2),
                dot(1),
                wall(10),
                dot(1),
                wall(1),
            ]),
            create_field_line(2, 29, vec![
                wall(1),
                dot(26),
                wall(1),
            ]),
            create_field_line(2, 30, vec![
                wall_corner(ClydeCorner),
                wall(26),
                wall_corner(InkyCorner),
            ]),
        ];

        let map = RawMap {
            fields: fields.into_iter()
                .inspect(|vec| assert!(vec.len() == 28 || vec.len() == 32))
                .flat_map(|f| f)
                .collect()
        };

        let json = serde_json::to_string(&map).unwrap();
        let mut file = OpenOptions::new().write(true).open("./maps/new_map.json").unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    fn create_field_line(start_x: usize, y: usize, elements: Vec<Vec<Vec<Element>>>) -> Vec<Field> {
        elements.into_iter()
            .flat_map(|i| i)
            .enumerate()
            .map(|(i, elems)| Field {
                position: Position::new(start_x + i, y),
                elements: elems,
            })
            .collect()
    }

    fn wall(amount: usize) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![Wall {
                wall_type: WallType::Outer,
                rotation: 0.0,
                is_corner: false,
            }])
            .collect()
    }

    fn wall_corner(corner: Element) -> Vec<Vec<Element>> {
        let mut res = wall(1);
        res[0].push(corner);
        res
    }

    fn dot(amount: usize) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![DotSpawn])
            .collect()
    }

    fn empty(amount: usize) -> Vec<Vec<Element>> {
        (0..amount).into_iter()
            .map(|_| vec![Empty])
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
            opening_direction: MoveDirection::Right,
        }]]
    }

    fn tunnel_left() -> Vec<Vec<Element>> {
        vec![vec![Tunnel {
            index: 0,
            opening_direction: MoveDirection::Left,
        }]]
    }
}