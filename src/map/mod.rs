use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use Rotation::*;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::position::Position;
use crate::game_asset_handles::GameAssetHandles;
use crate::game_asset_handles::keys::MAP;
use crate::life_cycle::LifeCycle::Loading;
use crate::map::board::Board;

pub mod board;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(JsonAssetPlugin::<Fields>::new(&["map.json"]))
            .add_system_set(
                SystemSet::on_exit(Loading).with_system(create_board_and_map)
            )
        ;
    }
}

fn create_board_and_map(
    mut commands: Commands,
    game_asset_handles: Res<GameAssetHandles>,
    fields_assets: Res<Assets<Fields>>,
) {
    let fields = fields_assets.get(&game_asset_handles.get_handle(MAP)).expect("the map should be loaded at this point");
    let map = Map::new(&fields);
    let board = Board::new(&map);
    commands.insert_resource(map);
    commands.insert_resource(board);
}

#[derive(Clone, Deref, Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "a09992c9-9567-42d9-a0ac-c998756e4073"]
struct Fields(Vec<Field>);

/// Resource that knows the spawn locations of every entity, based on an external map file.
///
/// The map should only be used to spawn or respawn entities into the world.
pub struct Map {
    elements_map: HashMap<Position, Vec<Element>>,
}

impl Map {
    fn new(fields: &Fields) -> Self {
        Map {
            elements_map: fields.clone().0.into_iter()
                .map(|f| (f.position, f.elements))
                .collect(),
        }
    }

    pub(in crate::map) fn get_width(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.x)
            .collect::<HashSet<_>>()
            .len()
    }

    pub(in crate::map) fn get_height(&self) -> usize {
        self.elements_map.iter()
            .map(|(pos, _)| pos.y)
            .collect::<HashSet<_>>()
            .len()
    }

    /// Return an iterator over all positions matching the given element filter.
    pub fn get_positions_matching(&self, filter: impl Fn(&Element) -> bool) -> impl IntoIterator<Item=&Position> {
        self.elements_map.iter()
            .filter(move |(_, elems)| Self::elements_match_filter(elems.iter(), &filter))
            .map(|(pos, _)| pos)
    }

    fn elements_match_filter<'a>(elems: impl IntoIterator<Item=&'a Element>, filter: &impl Fn(&Element) -> bool) -> bool {
        elems.into_iter()
            .map(filter)
            .max()
            .unwrap_or(false)
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
        let positions_matching_filter = self.get_positions_matching(filter).into_iter().collect::<Vec<_>>();

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
            }
            Left | Right => {
                let x = (vec_0.x + vec_1.x) / 2.0;
                let y = vec_0.y;
                Vec3::new(x, y, 0.0)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
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

/// Macro which quickly creates an element filter (closure Fn(&Element) -> bool) by passing a pattern.
///
/// The alternative would be a match/if let expression, which is much longer and harder to read.
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

/// This bullshit is only used to generate the json map until I have a better way to do this
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::OpenOptions;
    use std::io::Write;

    use crate::common::position::Position;
    use crate::common::Direction;
    use crate::map::{Element, Field, Rotation, WallType};
    use crate::map::Element::*;
    use crate::map::Rotation::*;
    use QuickWall::*;

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

    #[test]
    fn to_json() {
        let fields = vec![
            create_field_line(2, 0, vec![
                ghost_corner(PinkyCorner, D0),
                wall(12, D0, O),
                corner(D90, O),
                corner(D0, O),
                wall(12, D0, O),
                ghost_corner(BlinkyCorner, D90),
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
                elem(6, GhostHouse),
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
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, InkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, PinkySpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
                elems(vec![GhostHouse, ClydeSpawn]),
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
                elem(6, GhostHouse),
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
            create_field_line(2, 30, vec![
                ghost_corner(ClydeCorner, D270),
                wall(26, D180, O),
                ghost_corner(InkyCorner, D180),
            ]),
        ];

        let mut flat_fields = fields.into_iter()
            .enumerate()
            .inspect(|(i, vec)| {
                println!("{i}");
                assert!(vec.len() == 28 || vec.len() == 32)
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

    fn ghost_corner(ghost_corner: Element, rotation: Rotation) -> Vec<Vec<Element>> {
        let mut res = corner(rotation, O);
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