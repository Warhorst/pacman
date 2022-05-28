use bevy::prelude::*;
use crate::common::Position;
use crate::common::Direction::*;

use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::Element::PacManSpawn;
use crate::pacman::Pacman;
use crate::speed::SpeedByLevel;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

impl PacmanSpawn {
    /// Create a new pacman spawn. If
    /// - there are not exactly two spawns on the board
    /// - OR the two spawns are not neighbored
    /// the function panics.
    ///
    /// Pacman spawns centered between both spawn positions. The spawns can be aligned
    /// horizontally or vertically.
    fn new<'a, I: IntoIterator<Item=&'a Position>>(iter: I) -> Self {
        let positions = iter.into_iter().map(|p| *p).collect::<Vec<_>>();

        if positions.len() != 2 {
            panic!("There should be exactly two pacman spawns on the map")
        }

        let (pos_0, pos_1) = (positions[0], positions[1]);
        let neighbour_direction = pos_0.get_neighbour_direction(&pos_1).expect("The pacman spawns should be neighbored");
        let (vec_0, vec_1) = (Vec3::from(&pos_0), Vec3::from(&pos_1));

        match neighbour_direction {
            Up | Down => {
                let x = vec_0.x;
                let y = (vec_0.y + vec_1.y) / 2.0;
                PacmanSpawn(Vec3::new(x, y, 0.0))
            },
            Left | Right => {
                let x = (vec_0.x + vec_1.x) / 2.0;
                let y = vec_0.y;
                PacmanSpawn(Vec3::new(x, y, 0.0))
            }
        }
    }
}

pub fn spawn_pacman(
    mut commands: Commands,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let pacman_spawn = PacmanSpawn::new(board.get_positions_matching(is!(PacManSpawn)));
    let start_position = Position::from(&*pacman_spawn);
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("FFEE00").unwrap(),
                custom_size: Some(pacman_dimension),
                ..default()
            },
            transform: Transform::from_translation(*pacman_spawn),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(speed_by_level.for_pacman(&level).normal)
        .insert(start_position);
    commands.insert_resource(pacman_spawn);
}