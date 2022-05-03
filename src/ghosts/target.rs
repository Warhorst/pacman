use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::common::Direction::*;
use crate::common::Movement::*;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::state::State;
use crate::ghosts::state::State::*;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::map::Neighbour;
use crate::pacman::Pacman;
use crate::random::Random;

#[derive(Component)]
pub struct Target {
    target: Option<Position>,
}

impl Target {
    pub fn new() -> Self {
        Target {
            target: None,
        }
    }

    pub fn is_set(&self) -> bool {
        self.target.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    pub fn set_to(&mut self, position: Position) {
        self.target = Some(position);
    }

    pub fn get_position(&self) -> &Position {
        &self.target.as_ref().expect("The target should be set at this point")
    }

    pub fn get_position_opt(&self) -> &Option<Position> {
        &self.target
    }

    pub fn clear(&mut self) {
        self.target = None
    }
}

pub struct TargetSetPlugin;

impl Plugin for TargetSetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TargetUpdate>()
            .add_system(update_targets)
            .add_system(determine_spawned_target)
            .add_system(determine_scatter_target)
            .add_system(determine_blinky_chase_target)
            .add_system(determine_pinky_chase_target)
            .add_system(determine_frightened_target)
            .add_system(determine_eaten_target);
    }
}

#[derive(Copy, Clone)]
struct TargetUpdate(Entity, Option<Neighbour>);

/// Run all determine target systems in parallel and update the targets here.
fn update_targets(
    mut event_reader: EventReader<TargetUpdate>,
    mut query: Query<(Entity, &mut Target, &mut Movement)>,
) {
    for event in event_reader.iter() {
        for (entity, mut target, mut movement) in query.iter_mut() {
            update_target(*event, entity, &mut target, &mut movement)
        }
    }
}

fn update_target(event: TargetUpdate, entity: Entity, target: &mut Target, movement: &mut Movement) {
    if !target.is_set() && event.0 == entity {
        match event.1 {
            Some(neighbour) => {
                target.set_to(neighbour.position);
                *movement = Moving(neighbour.direction)
            }
            None => panic!("A ghost has no new target to move to!")
        }
    }
}

fn determine_spawned_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    query: Query<(Entity, &Target, &Movement, &Position, &State)>,
) {
    for (entity, target, movement, position, state) in query.iter() {
        if target.is_set() || state != &Spawned { continue; }

        let ghost_wall_positions = board.positions_of_type(GhostWall);
        let nearest_wall_position = ghost_wall_positions.into_iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_wall_position,
            &board,
            movement,
            |neighbour| neighbour_is_no_wall_in_spawn(&board, position, neighbour),
        );
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
}

fn determine_scatter_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    query: Query<(Entity, &Ghost, &Target, &Movement, &Position, &State)>,
) {
    for (entity, ghost, target, movement, position, state) in query.iter() {
        if target.is_set() || state != &Scatter { continue; }

        let ghost_corner_position = board.position_of_type(GhostCorner(*ghost));
        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            ghost_corner_position,
            &board,
            movement,
            |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
        );
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
}

fn determine_blinky_chase_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    blinky_query: Query<(Entity, &Ghost, &Target, &Movement, &Position, &State)>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, ghost, target, movement, blinky_position, state) in blinky_query.iter() {
        for pacman_position in pacman_query.iter() {
            if target.is_set() || ghost != &Blinky || state != &Chase { continue; }

            let next_target_neighbour = get_neighbour_nearest_to_target(
                blinky_position,
                pacman_position,
                &board,
                movement,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
            event_writer.send(TargetUpdate(entity, next_target_neighbour))
        }
    }
}

fn determine_pinky_chase_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    pinky_query: Query<(Entity, &Ghost, &Target, &Movement, &Position, &State)>,
    pacman_query: Query<(&Position, &Movement), With<Pacman>>,
) {
    for (entity, ghost, target, pinky_movement, pinky_position, state) in pinky_query.iter() {
        for (pacman_position, pacman_movement) in pacman_query.iter() {
            if target.is_set() || ghost != &Pinky || state != &Chase { continue; }

            let next_target_neighbour = get_neighbour_nearest_to_target(
                pinky_position,
                &calculate_pinky_target_position(pacman_position, pacman_movement),
                &board,
                pinky_movement,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
            event_writer.send(TargetUpdate(entity, next_target_neighbour))
        }
    }
}

/// Return the pinky target position 4 fields in pacmans direction.
/// If pacman is idle, the field to its right is choosen.
fn calculate_pinky_target_position(
    pacman_position: &Position,
    pacman_movement: &Movement
) -> Position {
    let x = pacman_position.x();
    let y = pacman_position.y();
    match *pacman_movement {
        Idle => Position::new(x + 4, y),
        Moving(dir) => match dir {
            Up => Position::new(x, y + 4),
            Down => Position::new(x, y - 4),
            Left => Position::new(x - 4, y),
            Right => Position::new(x + 4, y)
        }
    }
}

fn determine_frightened_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    random: Res<Random>,
    query: Query<(Entity, &Target, &Movement, &Position, &State)>,
) {
    for (entity, target, movement, position, state) in query.iter() {
        if target.is_set() || state != &Frightened { continue; }

        let possible_neighbours = get_possible_neighbours(
            position,
            &board,
            movement,
            |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
        );

        let next_target_neighbour = match possible_neighbours.len() {
            0 => None,
            1 => Some(possible_neighbours[0]),
            len => Some(possible_neighbours[random.zero_to(len)])
        };
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
}

fn determine_eaten_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    query: Query<(Entity, &Target, &Movement, &Position, &State)>,
) {
    for (entity, target, movement, position, state) in query.iter() {
        if target.is_set() || state != &Eaten { continue; }

        let ghost_spawn_positions = board.positions_of_type(GhostSpawn);
        let nearest_spawn_position = &ghost_spawn_positions.iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost spawn on the map");

        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_spawn_position,
            &board,
            movement,
            |neighbour| neighbour_is_no_normal_wall(&board, &neighbour.position),
        );
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
}

fn get_possible_neighbours<F: Fn(&Neighbour) -> bool>(
    ghost_position: &Position,
    board: &Board,
    movement: &Movement,
    field_filter: F,
) -> Vec<Neighbour> {
    board.neighbours_of(ghost_position)
        .into_iter()
        .filter(|neighbour| neighbour_not_in_opposite_direction(movement, neighbour))
        .filter(|neighbour| (field_filter)(neighbour))
        .collect()
}

fn get_neighbour_nearest_to_target<F: Fn(&Neighbour) -> bool>(
    ghost_position: &Position,
    target_position: &Position,
    board: &Board,
    movement: &Movement,
    field_filter: F,
) -> Option<Neighbour> {
    get_possible_neighbours(ghost_position, board, movement, field_filter)
        .into_iter()
        .min_by(|n_a, n_b| minimal_distance_to_neighbours(target_position, n_a, n_b))
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

fn neighbour_not_in_opposite_direction(movement: &Movement, neighbour: &Neighbour) -> bool {
    match *movement {
        Idle => true,
        Moving(dir) => neighbour.direction != dir.opposite()
    }
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}