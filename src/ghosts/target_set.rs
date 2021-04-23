use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::common::Movement::*;
use crate::ghosts::components::{Ghost, Target};
use crate::ghosts::components::Ghost::*;
use crate::ghosts::components::State;
use crate::ghosts::components::State::*;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::map::Neighbour;
use crate::pacman::Pacman;
use crate::random::Random;

/// TODO: where to put this fix?
/// if !self.movement.is_idle() && self.board.coordinates_directing_to_center(self.movement.get_direction(), self.coordinates) {
//      self.current_target.set_to(self.board.position_of_coordinates(&self.coordinates));
//      return
//  }
pub struct TargetSetPlugin;

impl Plugin for TargetSetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<TargetUpdate>()
            .add_system(update_targets.system())
            .add_system(determine_spawned_target.system())
            .add_system(determine_scatter_target.system())
            .add_system(determine_blinky_chase_target.system())
            .add_system(determine_frightened_target.system());
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
        let next_target_neighbour = board.neighbours_of(&position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(movement, neighbour))
            .filter(|neighbour| neighbour_is_no_wall_in_spawn(&board, position, neighbour))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&nearest_wall_position, n_a, n_b));
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
        let next_target_neighbour = board.neighbours_of(&position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(movement, neighbour))
            .filter(|neighbour| neighbour_is_no_wall(&board, &neighbour.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&ghost_corner_position, n_a, n_b));
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
}

fn determine_blinky_chase_target(
    mut event_writer: EventWriter<TargetUpdate>,
    board: Res<Board>,
    blink_query: Query<(Entity, &Ghost, &Target, &Movement, &Position, &State)>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, ghost, target, movement, blinky_position, state) in blink_query.iter() {
        for pacman_position in pacman_query.iter() {
            if target.is_set() || ghost != &Blinky || state != &Chase { continue; }

            let next_target_neighbour = board.neighbours_of(blinky_position)
                .into_iter()
                .filter(|neighbour| neighbour_not_in_opposite_direction(movement, neighbour))
                .filter(|neighbour| neighbour_is_no_wall(&board, &neighbour.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(pacman_position, n_a, n_b));
            event_writer.send(TargetUpdate(entity, next_target_neighbour))
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

        let possible_neighbours: Vec<Neighbour> = board.neighbours_of(position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(movement, neighbour))
            .filter(|neighbour| neighbour_is_no_wall(&board, &neighbour.position))
            .collect();

        let next_target_neighbour = match possible_neighbours.len() {
            0 => None,
            1 => Some(possible_neighbours[0]),
            len => Some(possible_neighbours[random.zero_to(len)])
        };
        event_writer.send(TargetUpdate(entity, next_target_neighbour))
    }
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