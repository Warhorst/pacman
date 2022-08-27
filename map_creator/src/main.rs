use std::collections::HashSet;
use pacman::map::WallType;
use pacman::common::position::Position;
use pacman::common::Direction;
use pacman::map::Element;
use pacman::map::Element::*;
use pacman::map::Field;
use pacman::map::Rotation;
use pacman::map::Rotation::*;
use QuickWall::*;
use std::fs::OpenOptions;
use std::io::Write;

const TARGET_FILE: &'static str = "../assets/maps/default.map.json";

/// This bullshit is only used to generate the json map until I have a better way to do this (aka a level editor)
fn main() {
    let fields = vec![
        create_field_line(1, 0, vec![
            elem(1, PinkyCorner),
            corner(D0, O),
            wall(12, D0, O),
            corner(D90, O),
            corner(D0, O),
            wall(12, D0, O),
            corner(D90, O),
            elem(1, BlinkyCorner)
        ]),
        create_field_line(2, 1, vec![
            wall(1, D270, O),
            dot(12),
            wall(1, D90, O),
            wall(1, D270, O),
            dot(12),
            wall(1, D90, O),
        ]),
        create_field_line(2, 2, vec![
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
        create_field_line(2, 3, vec![
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
        create_field_line(2, 4, vec![
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
        create_field_line(2, 5, vec![
            wall(1, D270, O),
            dot(26),
            wall(1, D90, O),
        ]),
        create_field_line(2, 6, vec![
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
        create_field_line(2, 7, vec![
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
        create_field_line(2, 8, vec![
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
        create_field_line(2, 9, vec![
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
        create_field_line(2, 10, vec![
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
        create_field_line(2, 11, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(4),
            elem(2, BlinkySpawn),
            empty(4),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(2, 12, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(1),
            corner(D0, G),
            wall(2, D0, G),
            elem(2, GhostHouseEntrance { rotation: D0 }),
            wall(2, D0, G),
            corner(D90, G),
            empty(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(0, 13, vec![
            elem(2, InvisibleWall),
            wall(5, D180, O),
            corner(D180, O),
            dot(1),
            corner(D270, I),
            corner(D180, I),
            empty(1),
            wall(1, D90, G),
            empty(6),
            wall(1, D90, G),
            empty(1),
            corner(D270, I),
            corner(D180, I),
            dot(1),
            corner(D270, O),
            wall(5, D180, O),
            elem(2, InvisibleWall),
        ]),
        create_field_line(0, 14, vec![
            tunnel_left(),
            elem(1, TunnelEntrance),
            elem(6, TunnelHallway),
            empty(4),
            wall(1, D270, G),
            elems(vec![InkySpawn]),
            elems(vec![InkySpawn]),
            elems(vec![PinkySpawn]),
            elems(vec![PinkySpawn]),
            elems(vec![ClydeSpawn]),
            elems(vec![ClydeSpawn]),
            wall(1, D90, G),
            empty(4),
            elem(6, TunnelHallway),
            elem(1, TunnelEntrance),
            tunnel_right(),
        ]),
        create_field_line(0, 15, vec![
            elem(2, InvisibleWall),
            wall(5, D0, O),
            corner(D90, O),
            dot(1),
            corner(D0, I),
            corner(D90, I),
            empty(1),
            wall(1, D270, G),
            empty(6),
            wall(1, D90, G),
            empty(1),
            corner(D0, I),
            corner(D90, I),
            dot(1),
            corner(D0, O),
            wall(5, D0, O),
            elem(2, InvisibleWall),
        ]),
        create_field_line(2, 16, vec![
            empty(5),
            wall(1, D270, O),
            dot(1),
            wall(1, D270, I),
            wall(1, D90, I),
            empty(1),
            corner(D270, G),
            wall(6, D180, G),
            corner(D180, G),
            empty(1),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(1),
            wall(1, D90, O),
            empty(5),
        ]),
        create_field_line(2, 17, vec![
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
        create_field_line(2, 18, vec![
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
        create_field_line(2, 19, vec![
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
        create_field_line(2, 20, vec![
            wall(1, D270, O),
            dot(12),
            wall(1, D270, I),
            wall(1, D90, I),
            dot(12),
            wall(1, D90, O),
        ]),
        create_field_line(2, 21, vec![
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
        create_field_line(2, 22, vec![
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
        create_field_line(2, 23, vec![
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
        create_field_line(2, 24, vec![
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
        create_field_line(2, 25, vec![
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
        create_field_line(2, 26, vec![
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
        create_field_line(2, 27, vec![
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
        create_field_line(2, 28, vec![
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
        create_field_line(2, 29, vec![
            wall(1, D270, O),
            dot(26),
            wall(1, D90, O),
        ]),
        create_field_line(1, 30, vec![
            elem(1, ClydeCorner),
            corner(D270, O),
            wall(26, D180, O),
            corner(D180, O),
            elem(1, InkyCorner),
        ]),
    ];
    let mut flat_fields = fields.into_iter()
        .enumerate()
        .inspect(|(i, vec)| {
            println!("{i}");
            assert!(vec.len() == 28 || vec.len() == 30 || vec.len() == 32)
        })
        .flat_map(|(_, f)| f)
        .collect::<Vec<_>>();
    let height = flat_fields.iter()
        .map(|f| f.position.x)
        .collect::<HashSet<_>>()
        .len();
    flat_fields.iter_mut()
        .for_each(|f| {
            f.position.y = (height as isize) - 2 - f.position.y
        });
    let json = serde_json::to_string(&flat_fields).unwrap();
    let mut file = OpenOptions::new().truncate(true).write(true).open(TARGET_FILE).unwrap();
    file.write(json.as_bytes()).unwrap();
}

#[derive(Copy, Clone)]
enum QuickWall {
    I,
    O,
    G,
}

impl QuickWall {
    fn to_wall(self) -> WallType {
        match self {
            I => WallType::Inner,
            O => WallType::Outer,
            G => WallType::Ghost,
        }
    }
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

fn wall(amount: usize, rotation: Rotation, wall_type: QuickWall) -> Vec<Vec<Element>> {
    (0..amount).into_iter()
        .map(move |_| vec![Wall {
            wall_type: wall_type.to_wall(),
            rotation,
            is_corner: false,
        }])
        .collect()
}

fn corner(rotation: Rotation, wall_type: QuickWall) -> Vec<Vec<Element>> {
    vec![vec![Wall {
        wall_type: wall_type.to_wall(),
        is_corner: true,
        rotation,
    }]]
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
