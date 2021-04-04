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
    board: &'a Board,
    ghost_position: &'a Position,
    movement: &'a mut Movement,
    current_target: &'a mut Target,
    state: &'a State,
    ghost: &'a Ghost,
    pacman_position: &'a Position,
}

impl<'a> TargetSetter<'a> {
    // How many components do we need?
    // Rügenwalder.jpg
    pub fn new(
        board: &'a Board,
        ghost_position: &'a Position,
        movement: &'a mut Movement,
        current_target: &'a mut Target,
        state: &'a State,
        ghost: &'a Ghost,
        pacman_position: &'a Position
    ) -> Self {
        TargetSetter { board, ghost_position, movement, current_target, state, ghost, pacman_position }
    }

    pub fn set_target(&mut self) {
        if self.current_target.is_set() {
            return;
        }

        let next_target_neighbour = match self.state {
            Spawned => self.determine_spawned_target_neighbour(),
            Scatter => self.determine_scatter_target_neighbour(),
            Chase => self.determine_chase_target_neighbour(),
            Frightened => self.determine_frightened_target_neighbour(),
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

    fn determine_spawned_target_neighbour(&self) -> Option<Neighbour> {
        let ghost_wall_positions = self.board.positions_of_type(GhostWall);
        let nearest_wall_position = ghost_wall_positions.into_iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(self.ghost_position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        self.board.neighbours_of(self.ghost_position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(self.movement, neighbour))
            .filter(|neighbour| self.neighbour_is_no_wall_in_spawn(neighbour))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&nearest_wall_position, n_a, n_b))
    }

    fn neighbour_is_no_wall_in_spawn(&self, neighbour: &Neighbour) -> bool {
        match *self.board.type_of_position(self.ghost_position) == GhostWall {
            true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
            false => neighbour.field_type != Wall
        }
    }

    fn determine_scatter_target_neighbour(&self) -> Option<Neighbour> {
        let ghost_corner_position = self.board.position_of_type(GhostCorner(*self.ghost));
        self.board.neighbours_of(self.ghost_position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(self.movement, neighbour))
            .filter(|neighbour| self.neighbour_is_no_wall(&neighbour.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&ghost_corner_position, n_a, n_b))
    }

    fn determine_chase_target_neighbour(&self) -> Option<Neighbour> {
        match self.ghost {
            Blinky => self.get_blinky_target(),
            _ => self.get_blinky_target()
        }
    }

    fn get_blinky_target(&self) -> Option<Neighbour> {
        self.board.neighbours_of(self.ghost_position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(self.movement, neighbour))
            .filter(|neighbour| self.neighbour_is_no_wall(&neighbour.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(self.pacman_position, n_a, n_b))
    }

    fn determine_frightened_target_neighbour(&self) -> Option<Neighbour> {
        let possible_neighbours: Vec<Neighbour> = self.board.neighbours_of(self.ghost_position)
            .into_iter()
            .filter(|neighbour| neighbour_not_in_opposite_direction(self.movement, neighbour))
            .filter(|neighbour| self.neighbour_is_no_wall(&neighbour.position))
            .collect();

        match possible_neighbours.len() {
            0 => None,
            1 => Some(possible_neighbours[0]),
            // TODO ask RNGesus
            _ => Some(possible_neighbours[0])
        }
    }

    fn neighbour_is_no_wall(&self, neighbour_position: &Position) -> bool {
        match self.board.type_of_position(neighbour_position) {
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