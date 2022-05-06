use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection;
use crate::common::MoveDirection::*;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::state::State;
use crate::ghosts::state::State::*;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::map::Neighbour;
use crate::pacman::Pacman;
use crate::random::Random;

#[derive(Component, Deref, DerefMut)]
pub struct Target(pub Position);

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(set_spawned_target)
            .add_system(set_scatter_target)
            .add_system(set_blinky_chase_target)
            .add_system(set_pinky_chase_target)
            .add_system(set_frightened_target)
            .add_system(set_eaten_target)
        ;
    }
}

fn set_spawned_target(
    mut commands: Commands,
    board: Res<Board>,
    mut query: Query<(Entity, &mut MoveDirection, &Position, &State), Without<Target>>,
) {
    for (entity, mut direction, position, state) in query.iter_mut() {
        if state != &Spawned { continue; }

        let ghost_wall_positions = board.positions_of_type(GhostWall);
        let nearest_wall_position = ghost_wall_positions.into_iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_wall_position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_wall_in_spawn(&board, position, neighbour),
        );

        *direction = next_target_neighbour.unwrap().direction;
        commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
    }
}

fn set_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    mut query: Query<(Entity, &Ghost, &mut MoveDirection, &Position, &State), Without<Target>>,
) {
    for (entity, ghost, mut direction, position, state) in query.iter_mut() {
        if state != &Scatter { continue; }

        let ghost_corner_position = board.position_of_type(GhostCorner(*ghost));
        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            ghost_corner_position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
        );
        *direction = next_target_neighbour.unwrap().direction;
        commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
    }
}

fn set_blinky_chase_target(
    mut commands: Commands,
    board: Res<Board>,
    mut blinky_query: Query<(Entity, &Ghost, &mut MoveDirection, &Position, &State), Without<Target>>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, ghost, mut direction, blinky_position, state) in blinky_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            if ghost != &Blinky || state != &Chase { continue; }

            let next_target_neighbour = get_neighbour_nearest_to_target(
                blinky_position,
                pacman_position,
                &board,
                &direction,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
            *direction = next_target_neighbour.unwrap().direction;
            commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
        }
    }
}

fn set_pinky_chase_target(
    mut commands: Commands,
    board: Res<Board>,
    mut pinky_query: Query<(Entity, &Ghost, &mut MoveDirection, &Position, &State), (Without<Pacman>, Without<Target>)>,
    pacman_query: Query<(&Position, &MoveDirection), With<Pacman>>,
) {
    for (entity, ghost, mut pinky_direction, pinky_position, state) in pinky_query.iter_mut() {
        for (pacman_position, pacman_direction) in pacman_query.iter() {
            if ghost != &Pinky || state != &Chase { continue; }

            let next_target_neighbour = get_neighbour_nearest_to_target(
                pinky_position,
                &calculate_pinky_target_position(pacman_position, pacman_direction),
                &board,
                &pinky_direction,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
            *pinky_direction = next_target_neighbour.unwrap().direction;
            commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
        }
    }
}

/// Return the pinky target position 4 fields in pacmans direction.
/// If pacman is idle, the field to its right is choosen.
fn calculate_pinky_target_position(
    pacman_position: &Position,
    pacman_direction: &MoveDirection,
) -> Position {
    let x = pacman_position.x();
    let y = pacman_position.y();
    match pacman_direction {
        Up => Position::new(x, y + 4),
        Down => Position::new(x, y - 4),
        Left => Position::new(x - 4, y),
        Right => Position::new(x + 4, y)
    }
}

fn set_frightened_target(
    mut commands: Commands,
    board: Res<Board>,
    random: Res<Random>,
    mut query: Query<(Entity, &mut MoveDirection, &Position, &State), Without<Target>>,
) {
    for (entity, mut direction, position, state) in query.iter_mut() {
        if state != &Frightened { continue; }

        let possible_neighbours = get_possible_neighbours(
            position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
        );

        let next_target_neighbour = match possible_neighbours.len() {
            0 => None,
            1 => Some(possible_neighbours[0]),
            len => Some(possible_neighbours[random.zero_to(len)])
        };
        *direction = next_target_neighbour.unwrap().direction;
        commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
    }
}

fn set_eaten_target(
    mut commands: Commands,
    board: Res<Board>,
    mut query: Query<(Entity, &mut MoveDirection, &Position, &State), Without<Target>>,
) {
    for (entity, mut direction, position, state) in query.iter_mut() {
        if state != &Eaten { continue; }

        let ghost_spawn_positions = board.positions_of_type(GhostSpawn);
        let nearest_spawn_position = &ghost_spawn_positions.iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost spawn on the map");

        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_spawn_position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_normal_wall(&board, &neighbour.position),
        );
        *direction = next_target_neighbour.unwrap().direction;
        commands.entity(entity).insert(Target(next_target_neighbour.unwrap().position));
    }
}

fn get_neighbour_nearest_to_target<F: Fn(&Neighbour) -> bool>(
    ghost_position: &Position,
    target_position: &Position,
    board: &Board,
    direction: &MoveDirection,
    field_filter: F,
) -> Option<Neighbour> {
    get_possible_neighbours(ghost_position, board, direction, field_filter)
        .into_iter()
        .min_by(|n_a, n_b| minimal_distance_to_neighbours(target_position, n_a, n_b))
}

fn get_possible_neighbours<F: Fn(&Neighbour) -> bool>(
    ghost_position: &Position,
    board: &Board,
    direction: &MoveDirection,
    field_filter: F,
) -> Vec<Neighbour> {
    board.neighbours_of(ghost_position)
        .into_iter()
        .filter(|neighbour| neighbour_not_in_opposite_direction(direction, neighbour))
        .filter(|neighbour| (field_filter)(neighbour))
        .collect()
}

fn neighbour_is_no_wall_in_spawn(board: &Board, ghost_position: &Position, neighbour: &Neighbour) -> bool {
    match *board.type_of_position(ghost_position) == GhostWall {
        true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
        false => neighbour.field_type != Wall
    }
}

fn neighbour_is_no_wall(board: &Board, neighbour_position: &Position) -> bool {
    match board.type_of_position(neighbour_position) {
        Wall | GhostWall => false,
        _ => true
    }
}

fn neighbour_is_no_normal_wall(board: &Board, neighbour_position: &Position) -> bool {
    match board.type_of_position(neighbour_position) {
        Wall => false,
        _ => true
    }
}

fn neighbour_not_in_opposite_direction(direction: &MoveDirection, neighbour: &Neighbour) -> bool {
    neighbour.direction != direction.opposite()
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}