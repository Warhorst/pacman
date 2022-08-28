use std::any::TypeId;
use std::collections::HashMap;
use bevy::prelude::*;
use crate::common::position::Position;
use crate::{is, map};
use map::Element;
use crate::board_dimensions::BoardDimensions;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};
use crate::common::Direction;
use crate::life_cycle::LifeCycle::Start;
use crate::map::{Map, Rotation, WallType};
use crate::map::Rotation::*;

pub struct GhostHousePlugin;

impl Plugin for GhostHousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(create_ghost_house)
            )
        ;
    }
}

fn create_ghost_house(
    mut commands: Commands,
    map: Res<Map>,
    dimensions: Res<BoardDimensions>
) {
    commands.insert_resource(GhostHouse::new(&map, &dimensions));
}

/// Resource that describes the ghost house, the place where ghosts start and respawn.
///
/// The ghost house is a fixed structure on the map bounded to a set of game rules.
/// It is a 6 x 3 field surrounded by walls, with a 2 x 1 opening centered on the top.
/// Inky, Pinky and Clyde spawn (in this order) inside the house, Blinky spawns in front of
/// the opening. Every ghost respawns at his start position, except Blinky who shares
/// this spot with Pinky.
///
/// It should look like this:
///    BB
/// WWWEEWWW
/// WHHHHHHW
/// WIIPPCCW
/// WHHHHHHW
/// WWWWWWWW
///
/// B = BlinkySpawn
/// I = InkySpawn
/// P = PinkySpawn
/// C = ClydeSpawn
/// E = Entrance
/// H = House
/// W = Wall
///
// TODO: it might not be possible to destructure the ghost house anymore. But it might be possible in the
//  future to rotate it. Therefore, everyone accessing the house acts relative to the house (like respecting the entrance direction).
pub struct GhostHouse {
    pub entrance_direction: Direction,
    spawns: HashMap<TypeId, Spawn>,
}

impl GhostHouse {
    pub fn new(map: &Map, dimensions: &BoardDimensions) -> Self {
        let bottom_left = Self::get_bottom_left(map);
        let rotation = Self::get_rotation(map);
        let spawns = Self::create_spawns(rotation, bottom_left, dimensions);

        GhostHouse {
            entrance_direction: Direction::Up.rotate(rotation),
            spawns,
        }
    }

    fn get_bottom_left(map: &Map) -> Position {
        map
            .get_positions_matching(is!(Element::Wall {wall_type: WallType::Ghost, ..}))
            .into_iter()
            .fold(
                Position::new(isize::MAX, isize::MAX),
                |acc, pos| Position::new(isize::min(acc.x, pos.x), isize::min(acc.y, pos.y)),
            )
    }

    fn get_rotation(map: &Map) -> Rotation {
        map
            .position_element_iter()
            .into_iter()
            .filter_map(|(_, elem)| match elem {
                Element::GhostHouseEntrance {rotation} => Some(*rotation),
                _ => None
            })
            .next()
            .expect("the map should at least contain one ghost house entrance")
    }

    fn create_spawns(rotation: Rotation, bottom_left: Position, dimensions: &BoardDimensions) -> HashMap<TypeId, Spawn> {
        [
            (TypeId::of::<Blinky>(), Self::create_blinky_spawn(rotation, bottom_left, dimensions)),
            (TypeId::of::<Pinky>(), Self::create_pinky_spawn(rotation, bottom_left, dimensions)),
            (TypeId::of::<Inky>(), Self::create_inky_spawn(rotation, bottom_left, dimensions)),
            (TypeId::of::<Clyde>(), Self::create_clyde_spawn(rotation, bottom_left, dimensions)),
        ]
            .into_iter()
            .collect()
    }

    fn create_blinky_spawn(rotation: Rotation, bottom_left: Position, dimensions: &BoardDimensions) -> Spawn {
        match rotation {
            D0 => Self::create_spawn_with_offsets(bottom_left, (3, 5), (4, 5), dimensions),
            D90 => Self::create_spawn_with_offsets(bottom_left, (5, 3), (5, 4), dimensions),
            D180 => Self::create_spawn_with_offsets(bottom_left, (3, -1), (4, -1), dimensions),
            D270 => Self::create_spawn_with_offsets(bottom_left, (-1, 3), (-1, 4), dimensions),
        }
    }

    fn create_pinky_spawn(rotation: Rotation, bottom_left: Position, dimensions: &BoardDimensions) -> Spawn {
        match rotation {
            D0 => Self::create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), dimensions),
            D90 => Self::create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), dimensions),
            D180 => Self::create_spawn_with_offsets(bottom_left, (3, 2), (4, 2), dimensions),
            D270 => Self::create_spawn_with_offsets(bottom_left, (2, 3), (2, 4), dimensions),
        }
    }

    fn create_inky_spawn(rotation: Rotation, bottom_left: Position, dimensions: &BoardDimensions) -> Spawn {
        match rotation {
            D0 => Self::create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), dimensions),
            D90 => Self::create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), dimensions),
            D180 => Self::create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), dimensions),
            D270 => Self::create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), dimensions),
        }
    }

    fn create_clyde_spawn(rotation: Rotation, bottom_left: Position, dimensions: &BoardDimensions) -> Spawn {
        match rotation {
            D0 => Self::create_spawn_with_offsets(bottom_left, (5, 2), (6, 2), dimensions),
            D90 => Self::create_spawn_with_offsets(bottom_left, (2, 1), (2, 2), dimensions),
            D180 => Self::create_spawn_with_offsets(bottom_left, (1, 2), (2, 2), dimensions),
            D270 => Self::create_spawn_with_offsets(bottom_left, (2, 5), (2, 6), dimensions),
        }
    }

    fn create_spawn_with_offsets(
        bottom_left: Position,
        offsets_0: (isize, isize),
        offsets_1: (isize, isize),
        dimensions: &BoardDimensions
    ) -> Spawn {
        let x = bottom_left.x;
        let y = bottom_left.y;
        let positions = [
            Position::new(x + offsets_0.0, y + offsets_0.1),
            Position::new(x + offsets_1.0, y + offsets_1.1),
        ];
        let coordinates = dimensions.positions_to_vec(positions.iter(), 0.0);
        Spawn { positions, coordinates }
    }

    pub fn spawn_coordinates_of<G: GhostType + 'static>(&self) -> Vec3 {
        self.spawn_of::<G>().coordinates
    }

    pub fn respawn_coordinates_of<G: GhostType + 'static>(&self) -> Vec3 {
        if TypeId::of::<G>() == TypeId::of::<Blinky>() {
            self.spawn_coordinates_of::<Pinky>()
        } else {
            self.spawn_coordinates_of::<G>()
        }
    }

    pub fn spawn_direction_of<G: GhostType + 'static>(&self) -> Direction {
        match TypeId::of::<G>() {
            t_id if t_id == TypeId::of::<Blinky>() => self.entrance_direction.rotate_left(),
            t_id if t_id == TypeId::of::<Pinky>() => self.entrance_direction,
            _ => self.entrance_direction.opposite()
        }
    }

    /// Blinky always spawns in front of the ghost house.
    pub fn positions_in_front_of_entrance(&self) -> impl IntoIterator<Item=&Position> {
        self.spawn_of::<Blinky>().positions.iter()
    }

    /// Blinky always spawns in front of the ghost house.
    pub fn coordinates_in_front_of_entrance(&self) -> Vec3 {
        self.spawn_of::<Blinky>().coordinates
    }

    /// Return the coordinates of the ghost house center. Every ghost moves to the center when leaving the house or entering for respawn.
    pub fn center_coordinates(&self) -> Vec3 {
        self.spawn_of::<Pinky>().coordinates
    }

    fn spawn_of<G: GhostType + 'static>(&self) -> &Spawn {
        self.spawns.get(&TypeId::of::<G>()).expect("every ghost should have a registered spawn")
    }
}

struct Spawn {
    pub coordinates: Vec3,
    pub positions: [Position; 2],
}