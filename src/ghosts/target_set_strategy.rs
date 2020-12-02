use std::cmp::Ordering;

use crate::common::{Movement, Position};
use crate::common::Movement::*;
use crate::ghosts::Ghost;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::map::Neighbour;

pub trait TargetSetStrategy {
    fn get_next_target_neighbour(&self) -> Option<Neighbour>;
}

pub struct SpawnedStrategy<'a> {
    board: &'a Board,
    position: &'a Position,
    movement: Movement
}

impl<'a> TargetSetStrategy for SpawnedStrategy<'a> {
    fn get_next_target_neighbour(&self) -> Option<Neighbour> {
        let ghost_wall_positions = self.board.positions_of_type(GhostWall);
        let nearest_wall_position = ghost_wall_positions.into_iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(self.position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        self.board.neighbours_of(self.position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(&self.movement, neighbour))
            .filter(|neighbour| self.position_is_movable(neighbour))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&nearest_wall_position, n_a, n_b))
    }
}

impl<'a> SpawnedStrategy<'a> {
    pub fn new(board: &'a Board, position: &'a Position, movement: Movement) -> Self {
        SpawnedStrategy { board, position, movement }
    }

    fn position_is_movable(&self, neighbour: &Neighbour) -> bool {
        match *self.board.type_of_position(self.position) == GhostWall {
            true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
            false => neighbour.field_type != Wall
        }
    }
}

pub struct ScatterStrategy<'a> {
    board: &'a Board,
    position: &'a Position,
    movement: Movement,
    ghost: &'a Ghost
}

impl<'a> TargetSetStrategy for ScatterStrategy<'a> {
    fn get_next_target_neighbour(&self) -> Option<Neighbour> {
        let ghost_corner_position = self.board.position_of_type(GhostCorner(*self.ghost));
        self.board.neighbours_of(self.position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(&self.movement, neighbour))
            .filter(|neighbour| self.position_is_movable(&neighbour.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&ghost_corner_position, n_a, n_b))
    }
}

impl<'a> ScatterStrategy<'a> {
    pub fn new(board: &'a Board, position: &'a Position, movement: Movement, ghost: &'a Ghost) -> Self {
        ScatterStrategy { board, position, movement, ghost }
    }

    fn position_is_movable(&self, position: &Position) -> bool {
        match self.board.type_of_position(position) {
            Wall | GhostWall => false,
            _ => true
        }
    }
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}

fn neighbour_not_in_opposite_direction(movement: &Movement, neighbour: &Neighbour) -> bool {
    match *movement {
        Idle => true,
        Moving(dir) => neighbour.direction != dir.opposite()
    }
}