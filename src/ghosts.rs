use bevy::prelude::*;

use Ghost::*;
use State::*;

use crate::common;
use crate::common::Direction::*;
use crate::common::Movement;
use crate::common::Movement::*;
use crate::common::Position;
use crate::constants::GHOST_DIMENSION;
use crate::constants::GHOST_SPEED;
use crate::map::{FieldType, Neighbour};
use crate::map::board::Board;
use crate::map::FieldType::*;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Target(Option<Position>);

/// The different states of a ghost#
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
#[derive(Debug, PartialOrd, PartialEq)]
enum State {
    Spawned,
    Chase,
    Scatter,
    Eaten,
    Frightened,
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ghosts.system())
            .add_system(set_target.system())
            .add_system(update_position.system())
            .add_system(update_state.system())
            .add_system(move_ghosts.system());
    }
}

fn spawn_ghosts(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let spawn_positions = board.positions_of_type(FieldType::GhostSpawn);
    spawn_ghost(spawn_positions[0], Blinky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[1], Pinky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[2], Inky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[3], Clyde, &mut commands, &board, &mut materials)
}

fn spawn_ghost(position: &Position, ghost: Ghost, commands: &mut Commands, board: &Res<Board>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let color_material = match ghost {
        Blinky => Color::hex("FF0000").unwrap().into(),
        Pinky => Color::hex("FFB8FF").unwrap().into(),
        Inky => Color::hex("00FFFF").unwrap().into(),
        Clyde => Color::hex("FFB852").unwrap().into(),
    };
    commands
        .spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            sprite: Sprite::new(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
            ..Default::default()
        })
        .with(ghost)
        .with(*position)
        .with(Target(None))
        .with(Idle)
        .with(Spawned);
}

fn move_ghosts(time: Res<Time>, board: Res<Board>, mut query: Query<With<Ghost, (&Movement, &mut Target, &mut Transform)>>) {
    for (movement, mut target, mut transform) in query.iter_mut() {
        if target.0.is_none() {
            continue;
        }
        let direction = match movement {
            Idle => continue,
            Moving(dir) => dir
        };

        let target_coordinates = board.coordinates_of_position(&target.0.unwrap());
        move_in_direction(&direction, &mut transform.translation, time.delta_seconds);
        limit_movement(&direction, &mut transform.translation, &target_coordinates);
        if transform.translation == target_coordinates {
            target.0 = None;
        }
    }
}

fn move_in_direction(direction: &common::Direction, translation: &mut Vec3, delta_seconds: f32) {
    let (x, y) = get_direction_modifiers(direction);
    *translation.x_mut() += delta_seconds * x * GHOST_SPEED;
    *translation.y_mut() += delta_seconds * y * GHOST_SPEED;
}

fn get_direction_modifiers(direction: &common::Direction) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

fn limit_movement(direction: &common::Direction, translation: &mut Vec3, target_coordinates: &Vec3) {
    match direction {
        Up => *translation.y_mut() = translation.y().min(target_coordinates.y()),
        Down => *translation.y_mut() = translation.y().max(target_coordinates.y()),
        Left => *translation.x_mut() = translation.x().max(target_coordinates.x()),
        Right => *translation.x_mut() = translation.x().min(target_coordinates.x()),
    }
}

fn update_position(board: Res<Board>, mut query: Query<With<Ghost, (&mut Position, &Transform)>>) {
    for (mut position, transform) in query.iter_mut() {
        *position = board.position_of_coordinates(&transform.translation);
    }
}

fn update_state(board: Res<Board>, mut query: Query<With<Ghost, (&Position, &mut State)>>) {
    for (position, mut state) in query.iter_mut() {
        if *state == Spawned && *board.type_of_position(position) == GhostWall {
            *state = Scatter
        }
    }
}

/// Set the ghosts target if he does not have one.
fn set_target(board: Res<Board>, mut query: Query<(&Ghost, &Position, &mut Target, &mut Movement, &State)>) {
    for (ghost, position, mut target, mut movement, state) in query.iter_mut() {
        if target.0.is_some() {
            continue;
        }
        let neighbour_to_move_to = match state {
            Spawned => spawn_neighbour(&board, position, &movement),
            Scatter => scatter_neighbour(&board, *ghost, position, &movement),
            _ => unimplemented!()
        };
        match neighbour_to_move_to {
            Some(neighbour) => {
                target.0 = Some(neighbour.position);
                *movement = Moving(neighbour.direction)
            }
            None => panic!("A ghost has no new target to move to")
        }
    }
}

/// When in state spawn, a ghost tries to leave the spawn area. This is done by finding the nearest way
/// out of the spawn area, which is the nearest ghost wall. When at the wall, the ghost is not allowed to return into the spawning area.
fn spawn_neighbour(board: &Board, position: &Position, movement: &Movement) -> Option<Neighbour> {
    let ghost_wall_positions = board.positions_of_type(GhostWall);
    let nearest_wall_position = ghost_wall_positions.into_iter()
        .min_by(|pos_a, pos_b| position.distance_to(pos_a).cmp(&position.distance_to(pos_b)))
        .expect("There should at least be one ghost wall on the map");
    board.neighbours_of(position)
        .into_iter()
        .filter(|neighbour| match movement {
            Idle => true,
            Moving(dir) => neighbour.direction != dir.opposite()
        })
        .filter(|neighbour| match *board.type_of_position(position) == GhostWall {
            true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
            false => neighbour.field_type != Wall
        })
        .min_by(|n_a, n_b| nearest_wall_position.distance_to(&n_a.position).cmp(&nearest_wall_position.distance_to(&n_b.position)))
}

/// Return the neighbour position to go to when in state scatter.
/// When in state scatter, the ghost tries to reach his specific ghost corner. Therefore,
/// the next target of the ghost will be the position nearest to it.
/// A ghost cannot go backwards when in state scatter.
fn scatter_neighbour(board: &Board, ghost: Ghost, position: &Position, movement: &Movement) -> Option<Neighbour> {
    let ghost_corner_position = board.position_of_type(GhostCorner(ghost));
    board.neighbours_of(position)
        .into_iter()
        .filter(|neighbour| match movement {
            Idle => true,
            Moving(dir) => neighbour.direction != dir.opposite()
        })
        .filter(|neighbour| !position_is_obstacle(board, &neighbour.position))
        .min_by(|n_a, n_b| ghost_corner_position.distance_to(&n_a.position).cmp(&ghost_corner_position.distance_to(&n_b.position)))
}

/// Returns if the given position is an obstacle for a ghost.
fn position_is_obstacle(board: &Board, position: &Position) -> bool {
    match board.type_of_position(position) {
        Wall | GhostWall => true,
        _ => false
    }
}