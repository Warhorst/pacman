use crate::game::map::{RawMap, WallType};
use crate::game::position::Position;
use crate::game::direction::Direction;
use crate::game::map::Element;
use crate::game::map::Element::*;
use crate::game::map::Field;
use crate::game::map::Rotation;
use crate::game::map::Rotation::*;
use QuickWall::*;
use std::fs::OpenOptions;
use std::io::Write;

const TARGET_FILE: &'static str = "./assets/maps/default.map.json";

/// This bullshit is only used to generate the json map until I have a better way to do this (aka a level editor)
#[test]
fn run() {
    let fields = vec![
        create_field_line(0, vec![
            corner(D0, O),
            wall(12, D0, O),
            corner(D90, O),
            corner(D0, O),
            wall(12, D0, O),
            corner(D90, O),
        ]),
        create_field_line(1, vec![
            wall(1, D270, O),
            dot(12),
            wall(1, D90, O),
            wall(1, D270, O),
            dot(12),
            wall(1, D90, O),
        ]),
        create_field_line(2, vec![
            wall(1, D270, O),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(3, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D90, O),
            wall(1, D270, O),
            dot(1),
            corner(D0, I),
            wall(3, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(3, vec![
            wall(1, D270, O),
            energizer(),
            wall(1, D270, I),
            empty(2),
            wall(1, D90, I),
            dot(1),
            wall(1, D270, I),
            empty(3),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            wall(1, D270, O),
            dot(1),
            wall(1, D90, I),
            empty(3),
            wall(1, D270, I),
            dot(1),
            wall(1, D90, I),
            empty(2),
            wall(1, D270, I),
            energizer(),
            wall(1, D90, O),
        ]),
        create_field_line(4, vec![
            wall(1, D270, O),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            wall(3, D180, I),
            corner(D180, I),
            dot(1),
            corner(D270, O),
            corner(D180, O),
            dot(1),
            corner(D270, I),
            wall(3, D180, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(5, vec![
            wall(1, D270, O),
            dot(26),
            wall(1, D90, O),
        ]),
        create_field_line(6, vec![
            wall(1, D270, O),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(6, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(7, vec![
            wall(1, D270, O),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D90, I),
            corner(D0, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(8, vec![
            wall(1, D270, O),
            dot(6),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(6),
            wall(1, D90, O),
        ]),
        create_field_line(9, vec![
            corner(D270, O),
            wall(4, D180, O),
            corner(D90, O),
            dot(1),
            wall(1, D90, I),
            corner(D270, I),
            wall(2, D0, I),
            corner(D90, I),
            empty(1),
            wall(2, D90, I),
            empty(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D180, I),
            wall(1, D90, I),
            dot(1),
            corner(D0, O),
            wall(4, D180, O),
            corner(D180, O),
        ]),
        create_field_line(10, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D90, I),
            corner(D0, I),
            wall(2, D0, I),
            corner(D180, I),
            empty(1),
            corner(D270, I),
            corner(D180, I),
            empty(1),
            corner(D270, I),
            wall(2, D0, I),
            corner(D90, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(11, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(10),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(12, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(1),
            elem(8, GhostHouse {rotation: D0}),
            empty(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(13, vec![
            wall(5, D180, O),
            corner(D180, O),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            empty(1),
            elem(8, GhostHouse {rotation: D0}),
            empty(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, O),
            wall(5, D180, O),
        ]),
        create_field_line(14, vec![
            tunnel_left(),
            elem(5, TunnelHallway),
            empty(4),
            elem(8, GhostHouse {rotation: D0}),
            empty(4),
            elem(5, TunnelHallway),
            tunnel_right(),
        ]),
        create_field_line(15, vec![
            wall(5, D0, O),
            corner(D90, O),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            empty(1),
            elem(8, GhostHouse {rotation: D0}),
            empty(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, O),
            wall(5, D0, O),
        ]),
        create_field_line(16, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(1),
            elem(8, GhostHouse {rotation: D0}),
            empty(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(17, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(4),
            elem(2, FruitSpawn),
            empty(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(18, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(1),
            corner(D0, I),
            wall(6, D0, I),
            corner(D90, I),
            empty(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(19, vec![
            corner(D0, O),
            wall(4, D180, O),
            corner(D180, O),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            empty(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D90, I),
            corner(D0, I),
            wall(2, D180, I),
            corner(D180, I),
            empty(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, O),
            wall(4, D180, O),
            corner(D90, O),
        ]),
        create_field_line(20, vec![
            wall(1, D270, O),
            dot(12),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(12),
            wall(1, D90, O),
        ]),
        create_field_line(21, vec![
            wall(1, D270, O),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(3, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D0, I),
            wall(3, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(22, vec![
            wall(1, D270, O),
            dot(1),
            corner(D270, I),
            wall(1, D0, I),
            corner(D90, I),
            wall(1, D90, I),
            dot(1),
            corner(D270, I),
            wall(3, D180, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            wall(3, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D90, I),
            corner(D0, I),
            wall(1, D0, I),
            corner(D180, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(23, vec![
            wall(1, D270, O),
            energizer(),
            dot(2),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(7),
            elem(2, PacManSpawn),
            dot(7),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(2),
            energizer(),
            wall(1, D90, O),
        ]),
        create_field_line(24, vec![
            corner(D270, O),
            wall(1, D0, O),
            corner(D90, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            wall(6, D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D0, O),
            wall(1, D0, O),
            corner(D180, O),
        ]),
        create_field_line(25, vec![
            corner(D0, O),
            wall(1, D180, O),
            corner(D180, O),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D270, I),
            wall(2, D180, I),
            corner(D90, I),
            corner(D0, I),
            wall(2, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, O),
            wall(1, D180, O),
            corner(D90, O),
        ]),
        create_field_line(26, vec![
            wall(1, D270, O),
            dot(6),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(6),
            wall(1, D90, O),
        ]),
        create_field_line(27, vec![
            wall(1, D270, O),
            dot(1),
            corner(D0, I),
            wall(4, D0, I),
            corner(D180, I),
            corner(D270, I),
            wall(2, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            corner(D0, I),
            wall(2, D0, I),
            corner(D180, I),
            corner(D270, I),
            wall(4, D0, I),
            corner(D90, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(28, vec![
            wall(1, D270, O),
            dot(1),
            corner(D270, I),
            wall(8, D180, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, I),
            wall(8, D180, I),
            corner(D180, I),
            dot(1),
            wall(1, D90, O),
        ]),
        create_field_line(29, vec![
            wall(1, D270, O),
            dot(26),
            wall(1, D90, O),
        ]),
        create_field_line(30, vec![
            corner(D270, O),
            wall(26, D180, O),
            corner(D180, O),
        ]),
    ];
    let height = fields.len();
    let mut flat_fields = fields.into_iter()
        .enumerate()
        .flat_map(|(_, f)| f)
        .collect::<Vec<_>>();
    flat_fields.iter_mut()
        .for_each(|f| {
            f.position.y = (height as isize) - (f.position.y + 1)
        });

    let raw_map = RawMap {
        blinky_corner: Position::new(28, 30),
        pinky_corner: Position::new(0, 30),
        inky_corner: Position::new(28, 0),
        clyde_corner: Position::new(0, 0),
        fields: flat_fields
    };

    let json = serde_json::to_string(&raw_map).unwrap();
    let mut file = OpenOptions::new().truncate(true).write(true).open(TARGET_FILE).unwrap();
    file.write(json.as_bytes()).unwrap();
}

#[derive(Copy, Clone)]
enum QuickWall {
    I,
    O,
}

impl QuickWall {
    fn to_wall(self) -> WallType {
        match self {
            I => WallType::Inner,
            O => WallType::Outer,
        }
    }
}

fn create_field_line(y: isize, elements: Vec<Vec<Element>>) -> Vec<Field> {
    elements.into_iter()
        .flat_map(|i| i)
        .enumerate()
        .map(|(i, elem)| Field {
            position: Position::new(i as isize, y),
            element: elem,
        })
        .filter(|f| match f.element {
            Tunnel {index: 42, ..} => false,
            _ => true
        })
        .collect()
}

fn wall(amount: usize, rotation: Rotation, wall_type: QuickWall) -> Vec<Element> {
    (0..amount).into_iter()
        .map(move |_| Wall {
            wall_type: wall_type.to_wall(),
            rotation,
            is_corner: false,
        })
        .collect()
}

fn corner(rotation: Rotation, wall_type: QuickWall) -> Vec<Element> {
    vec![Wall {
        wall_type: wall_type.to_wall(),
        is_corner: true,
        rotation,
    }]
}

fn dot(amount: usize) -> Vec<Element> {
    (0..amount).into_iter()
        .map(|_| DotSpawn)
        .collect()
}

/// Bad workaround. I dont want to add an 'Empty' field, as it will be
/// never used in the real app. So I just create a tunnel with index 42 and filter such elements later
fn empty(amount: usize) -> Vec<Element> {
    (0..amount).into_iter()
        .map(|_| Tunnel {
            index: 42,
            opening_direction: Direction::Left,
        })
        .collect()
}

fn energizer() -> Vec<Element> {
    vec![EnergizerSpawn]
}

fn elem(amount: usize, elem: Element) -> Vec<Element> {
    (0..amount).into_iter()
        .map(|_| elem)
        .collect()
}

fn tunnel_right() -> Vec<Element> {
    vec![Tunnel {
        index: 0,
        opening_direction: Direction::Right,
    }]
}

fn tunnel_left() -> Vec<Element> {
    vec![Tunnel {
        index: 0,
        opening_direction: Direction::Left,
    }]
}
