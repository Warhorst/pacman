use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use crate::common::Position;
use crate::{is, map};
use crate::map::board::Board;
use map::Element;
use map::Element::*;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};

pub struct GhostHousePlugin;

impl Plugin for GhostHousePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ghost_house);
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
pub struct GhostHouse {
    pub entrance_positions: HashSet<Position>,
    spawns: HashMap<TypeId, Spawn>,
}

impl GhostHouse {
    pub fn new(board: &Board) -> Self {
        let entrance_positions = board.get_positions_matching(is!(Element::GhostHouseEntrance {..}));
        let ghost_house_positions = board.get_positions_matching(is!(Element::GhostHouse));
        let top_right = ghost_house_positions
            .iter()
            .fold(Position::new(0, 0), |acc, pos| Position::new(isize::max(acc.x, pos.x), isize::max(acc.y, pos.y)));
        Self::assert_positions_valid(&top_right, &entrance_positions, &ghost_house_positions);

        let mut spawns = HashMap::with_capacity(4);
        spawns.insert(TypeId::of::<Blinky>(), Self::create_blinky_spawn(&top_right));
        spawns.insert(TypeId::of::<Pinky>(), Self::create_pinky_spawn(&top_right));
        spawns.insert(TypeId::of::<Inky>(), Self::create_inky_spawn(&top_right));
        spawns.insert(TypeId::of::<Clyde>(), Self::create_clyde_spawn(&top_right));

        GhostHouse {
            entrance_positions: HashSet::from_iter(entrance_positions.into_iter().map(|p| *p)),
            spawns
        }
    }

    fn create_possible_ghost_house_positions(top_right: &Position) -> Vec<Position> {
        let mut result = Vec::with_capacity(18);
        let x = top_right.x;
        let y = top_right.y;

        for i in (x - 5)..=x {
            for j in (y - 2)..=y {
                result.push(Position::new(i, j))
            }
        }

        result
    }

    /// Check if all GhostHouse and GhostHouseEntrance positions on the map are possible positions.
    /// Note: It is not checked if the ghost house is surrounded by walls.
    fn assert_positions_valid(top_right: &Position, entrance_positions: &Vec<&Position>, ghost_house_positions: &Vec<&Position>) {
        let possible_entrance_positions = vec![Position::new(top_right.x - 3, top_right.y + 1), Position::new(top_right.x - 2, top_right.y + 1)];
        let possible_house_positions = Self::create_possible_ghost_house_positions(&top_right);

        for pos in entrance_positions {
            if !possible_entrance_positions.contains(pos) {
                panic!(
                    "Not all ghost house entrance positions build a valid ghost house entrance. Top right: {}, problem: {}, possible: {}",
                    top_right,
                    pos,
                    Self::print_positions(possible_entrance_positions.iter())
                )
            }
        }

        for pos in ghost_house_positions {
            if !possible_house_positions.contains(pos) {
                panic!(
                    "Not all ghost house positions build a valid ghost house. Top right: {}, problem: {}, possible: {}",
                    top_right,
                    pos,
                    Self::print_positions(possible_house_positions.iter())
                )
            }
        }
    }

    fn print_positions<'a, I: IntoIterator<Item=&'a Position>>(positions: I) -> String {
        positions.into_iter().map(Position::to_string).collect::<Vec<String>>().join(", ")
    }

    fn create_blinky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 3, top_right.y + 2),
            Position::new(top_right.x - 2, top_right.y + 2)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_pinky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 3, top_right.y - 1),
            Position::new(top_right.x - 2, top_right.y - 1)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_inky_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 5, top_right.y - 1),
            Position::new(top_right.x - 4, top_right.y - 1)
        ];
        let coordinates = Self::centered_position_for(&positions);

        Spawn {
            positions,
            coordinates
        }
    }

    fn create_clyde_spawn(top_right: &Position) -> Spawn {
        let positions = [
            Position::new(top_right.x - 1, top_right.y - 1),
            Position::new(top_right.x, top_right.y - 1)
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

    /// Blinky always spawns in front of the ghost house.
    pub fn positions_in_front_of_entrance(&self) -> impl IntoIterator<Item=&Position> {
        self.spawn_of::<Blinky>().positions.iter()
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

/// Resource that knows where the ghost house and its entrances are.
/// The walls around a ghost house are not considered part of the ghost house.
pub struct GhostHousePositions {
    pub entrances: HashSet<Position>,
    pub interior: HashSet<Position>
}

impl GhostHousePositions {
    fn new<'a, E: IntoIterator<Item=&'a Position>, I: IntoIterator<Item=&'a Position>>(entrance_iter: E, interior_iter: I) -> Self {
        let entrances = entrance_iter.into_iter().map(|p| *p).collect();
        let interior = interior_iter.into_iter().map(|p| *p).collect();

        GhostHousePositions {
            entrances, interior
        }
    }

    pub fn position_is_entrance(&self, pos: &Position) -> bool {
        self.entrances.contains(pos)
    }

    pub fn position_is_interior(&self, pos: &Position) -> bool {
        self.interior.contains(pos)
    }

    pub fn position_is_in_house(&self, pos: &Position) -> bool {
        self.interior.contains(pos) || self.entrances.contains(pos)
    }
}

fn spawn_ghost_house(
    mut commands: Commands,
    board: Res<Board>
) {
    commands.insert_resource(GhostHouse::new(&board));

    let ghost_house_positions = GhostHousePositions::new(
        board.get_positions_matching(is!(GhostHouseEntrance {..})),
        board.get_positions_matching(is!(GhostHouse))
    );
    commands.insert_resource(ghost_house_positions);
}