use std::cmp::Ordering;

use crate::common::{Movement, Position};
use crate::common::Movement::*;
use crate::ghosts::components::{Ghost, State};
use crate::ghosts::Target;
use crate::map::board::Board;
use crate::map::FieldType::{GhostCorner, GhostSpawn, GhostWall, Wall};
use crate::map::Neighbour;

use super::components::Ghost::*;
use super::components::State::*;

/// Sets the next target for a ghost.
pub(in crate::ghosts) struct TargetSetter<'a> {
    current_target: &'a mut Target,
    state: &'a State,
    movement: &'a mut Movement,
    ghost: &'a Ghost,
    ghost_position: &'a Position,
    pacman_position: &'a Position,
    board: &'a Board
}

impl<'a> TargetSetter<'a> {
    pub fn new(current_target: &'a mut Target,
               state: &'a State,
               movement: &'a mut Movement,
               ghost: &'a Ghost,
               ghost_position: &'a Position,
               pacman_position: &'a Position,
               board: &'a Board) -> Self {
        TargetSetter { current_target, state, movement, ghost, ghost_position, pacman_position, board }
    }

    pub fn set_target(&mut self) {
        if self.current_target.is_set() {
            return;
        }

        let next_target_neighbour = match self.state {
            Spawned => SpawnedStrategy::new(self.board, self.ghost_position, self.movement).get_next_target_neighbour(),
            Scatter => ScatterStrategy::new(self.board, self.ghost_position, self.movement, self.ghost).get_next_target_neighbour(),
            Chase => ChaseStrategy::new(self.board, self.ghost_position, self.movement, self.ghost, self.pacman_position).get_next_target_neighbour(),
            _ => return
        };

        match next_target_neighbour {
            Some(neighbour) => {
                self.current_target.set_to(neighbour.position);
                *self.movement = Moving(neighbour.direction)
            }
            None => panic!("A ghost has no new target to move to")
        }
    }
}

pub trait TargetSetStrategy {
    fn get_next_target_neighbour(&self) -> Option<Neighbour>;
}

struct SpawnedStrategy<'a> {
    board: &'a Board,
    position: &'a Position,
    movement: &'a Movement
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
    pub fn new(board: &'a Board, position: &'a Position, movement: &'a Movement) -> Self {
        SpawnedStrategy { board, position, movement }
    }

    fn position_is_movable(&self, neighbour: &Neighbour) -> bool {
        match *self.board.type_of_position(self.position) == GhostWall {
            true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
            false => neighbour.field_type != Wall
        }
    }
}

struct ScatterStrategy<'a> {
    board: &'a Board,
    position: &'a Position,
    movement: &'a Movement,
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
    pub fn new(board: &'a Board, position: &'a Position, movement: &'a Movement, ghost: &'a Ghost) -> Self {
        ScatterStrategy { board, position, movement, ghost }
    }

    fn position_is_movable(&self, position: &Position) -> bool {
        match self.board.type_of_position(position) {
            Wall | GhostWall => false,
            _ => true
        }
    }
}

pub struct ChaseStrategy<'a> {
    board: &'a Board,
    position: &'a Position,
    movement: &'a Movement,
    ghost: &'a Ghost,
    pacman_position: &'a Position,
}

impl<'a> TargetSetStrategy for ChaseStrategy<'a> {
    fn get_next_target_neighbour(&self) -> Option<Neighbour> {
        match self.ghost {
            Blinky => self.get_blinky_target(),
            _ => self.get_blinky_target()
        }
    }
}

impl<'a> ChaseStrategy<'a> {
    pub fn new(board: &'a Board, position: &'a Position, movement: &'a Movement, ghost: &'a Ghost, pacman_position: &'a Position) -> Self {
        ChaseStrategy { board, position, movement, ghost, pacman_position }
    }

    /// Return the neighbour position nearest to pacmans current position.
    fn get_blinky_target(&self) -> Option<Neighbour> {
        self.board.neighbours_of(self.position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(&self.movement, neighbour))
            .filter(|neighbour| self.position_is_movable(&neighbour.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(self.pacman_position, n_a, n_b))
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