use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection::*;
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::{Blinky, Clyde, Ghost, Inky, Pinky};
use crate::ghosts::state::Spawned;
use crate::is;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::Element::*;
use crate::speed::SpeedByLevel;

pub struct GhostSpawns {
    pub blinky: Vec3,
    pub pinky: Vec3,
    pub inky: Vec3,
    pub clyde: Vec3,
}

impl GhostSpawns {
    fn new(board: &Board) -> Self {
        GhostSpawns {
            blinky: Self::get_coordinates_of_spawn(board.get_positions_matching(is!(BlinkySpawn))),
            pinky: Self::get_coordinates_of_spawn(board.get_positions_matching(is!(PinkySpawn))),
            inky: Self::get_coordinates_of_spawn(board.get_positions_matching(is!(InkySpawn))),
            clyde: Self::get_coordinates_of_spawn(board.get_positions_matching(is!(ClydeSpawn))),
        }
    }

    fn get_coordinates_of_spawn<'a, I: IntoIterator<Item=&'a Position>>(iter: I) -> Vec3 {
        let positions = iter.into_iter().map(|p| *p).collect::<Vec<_>>();

        if positions.len() != 2 {
            panic!("There should be exactly two spawns of the same ghost on the map")
        }

        let (pos_0, pos_1) = (positions[0], positions[1]);
        let neighbour_direction = pos_0.get_neighbour_direction(&pos_1).expect("The spawns of the same ghost should be neighbored");
        let (vec_0, vec_1) = (Vec3::from(&pos_0), Vec3::from(&pos_1));

        // Not using this for now
        // match neighbour_direction {
        //     Up | Down => {
        //         let x = vec_0.x;
        //         let y = (vec_0.y + vec_1.y) / 2.0;
        //         Vec3::new(x, y, 0.0)
        //     },
        //     Left | Right => {
        //         let x = (vec_0.x + vec_1.x) / 2.0;
        //         let y = vec_0.y;
        //         Vec3::new(x, y, 0.0)
        //     }
        // }

        vec_0
    }
}

pub fn spawn_ghosts(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let ghost_spawns = GhostSpawns::new(&board);
    spawn_ghost(&mut commands, ghost_spawns.blinky, &level, &speed_by_level, Color::hex("FF0000").unwrap(), Blinky);
    spawn_ghost(&mut commands, ghost_spawns.pinky, &level, &speed_by_level, Color::hex("FFB8FF").unwrap(), Pinky);
    spawn_ghost(&mut commands, ghost_spawns.inky, &level, &speed_by_level, Color::hex("00FFFF").unwrap(), Inky);
    spawn_ghost(&mut commands, ghost_spawns.clyde, &level, &speed_by_level, Color::hex("FFB852").unwrap(), Clyde);
    commands.insert_resource(ghost_spawns);
}

fn spawn_ghost(
    commands: &mut Commands,
    spawn_coordinates: Vec3,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    color: Color,
    ghost_type: impl Component
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        })
        .insert(Ghost)
        .insert(ghost_type)
        .insert(Position::from(&spawn_coordinates))
        .insert(Up)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(Spawned);
}