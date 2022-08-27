use std::any::TypeId;
use std::collections::HashMap;
use bevy::prelude::*;
use crate::common::position::Position;
use crate::{is, map};
use map::Element;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};
use crate::common::Direction;
use crate::map::{Map, WallType};

pub struct GhostHousePlugin;

impl Plugin for GhostHousePlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Spawn the ghosthouse here
    }
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
    /// TODO: Basically, the ghost house is one big rectangle. I could retrieve this rectangle from
    ///  the map and calculate all positions from it (walls, spawns, entrance)
    pub fn new(map: &Map) -> Self {
        let top_right = map
            .get_positions_matching(is!(Element::Wall {wall_type: WallType::Ghost, ..}))
            .into_iter()
            .fold(Position::new(isize::MIN, isize::MIN), |acc, pos| Position::new(isize::max(acc.x, pos.x), isize::max(acc.y, pos.y)));

        let mut spawns = HashMap::with_capacity(4);
        spawns.insert(TypeId::of::<Blinky>(), Self::create_blinky_spawn(&top_right));
        spawns.insert(TypeId::of::<Pinky>(), Self::create_pinky_spawn(&top_right));
        spawns.insert(TypeId::of::<Inky>(), Self::create_inky_spawn(&top_right));
        spawns.insert(TypeId::of::<Clyde>(), Self::create_clyde_spawn(&top_right));

        GhostHouse {
            entrance_direction: Direction::Up,
            spawns
        }
    }

    fn create_blinky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 4, top_right.y + 1),
            Position::new(top_right.x - 3, top_right.y + 1)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_pinky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 4, top_right.y - 2),
            Position::new(top_right.x - 3, top_right.y - 2)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_inky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 6, top_right.y - 2),
            Position::new(top_right.x - 5, top_right.y - 2)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_clyde_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 2, top_right.y - 2),
            Position::new(top_right.x - 1, top_right.y - 2)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn centered_position_for(positions: &[Position; 2]) -> Vec3 {
        let vec_0 = Vec3::from(&positions[0]);
        let vec_1 = Vec3::from(&positions[1]);

        Vec3::new(
            (vec_0.x + vec_1.x) / 2.0,
            vec_0.y,
            0.0
        )
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
    pub positions: [Position; 2]
}