use std::cmp::Ordering;

use crate::common::{Movement, Position};
use crate::common::Movement::*;
use crate::ghosts::{Ghost, Target};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::map::FieldType::GhostCorner;
use crate::map::Neighbour;

/// Sets the next target for a ghost.
pub (in crate::ghosts) struct TargetSetter<'a> {
    board: &'a Board,
    target: &'a mut Target,
    movement: &'a mut Movement,
    position: &'a Position,
}

impl<'a> TargetSetter<'a> {
    pub fn new(board: &'a Board, target: &'a mut Target, movement: &'a mut Movement, position: &'a Position) -> Self {
        TargetSetter { board, target, movement, position }
    }

    /// When in state spawn, a ghost tries to leave the spawn area. This is done by finding the nearest way
    /// out of the spawn area, which is the nearest ghost wall. When at the wall, the ghost is not allowed to return into the spawning area.
    /// /// A ghost cannot go backwards when in state spawn.
    pub fn set_spawn_target(&mut self) {
        let ghost_wall_positions = self.board.positions_of_type(GhostWall);
        let nearest_wall_position = ghost_wall_positions.into_iter()
            .min_by(|pos_a, pos_b| self.minimal_distance_to_positions(self.position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        let target_neighbour = self.board.neighbours_of(self.position)
            .into_iter()
            .filter(|neighbour| self.neighbour_not_in_opposite_direction(neighbour))
            .filter(|neighbour| self.position_is_movable_spawn(neighbour))
            .min_by(|n_a, n_b| self.minimal_distance_to_neighbours(&nearest_wall_position, n_a, n_b));
        self.set_target(target_neighbour)
    }

    fn position_is_movable_spawn(&self, neighbour: &Neighbour) -> bool {
        match *self.board.type_of_position(self.position) == GhostWall {
            true => neighbour.field_type != Wall && neighbour.field_type != GhostSpawn,
            false => neighbour.field_type != Wall
        }
    }

    /// Return the neighbour position to go to when in state scatter.
    /// When in state scatter, the ghost tries to reach his specific ghost corner. Therefore,
    /// the next target of the ghost will be the position nearest to it.
    /// A ghost cannot go backwards when in state scatter.
    pub fn set_scatter_target(&mut self, ghost: &Ghost) {
        let ghost_corner_position = self.board.position_of_type(GhostCorner(*ghost));
        let target_neighbour = self.board.neighbours_of(self.position)
            .into_iter()
            .filter(|neighbour| self.neighbour_not_in_opposite_direction(neighbour))
            .filter(|neighbour| self.position_is_movable_scatter(&neighbour.position))
            .min_by(|n_a, n_b| self.minimal_distance_to_neighbours(&ghost_corner_position, n_a, n_b));
        self.set_target(target_neighbour)
    }

    fn position_is_movable_scatter(&self, position: &Position) -> bool {
        match self.board.type_of_position(position) {
            Wall | GhostWall => false,
            _ => true
        }
    }

    fn neighbour_not_in_opposite_direction(&self, neighbour: &Neighbour) -> bool {
        match *self.movement {
            Idle => true,
            Moving(dir) => neighbour.direction != dir.opposite()
        }
    }

    fn minimal_distance_to_neighbours(&self, big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
        self.minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
    }

    fn minimal_distance_to_positions(&self, big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
        big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
    }

    fn set_target(&mut self, target_neighbour: Option<Neighbour>) {
        match target_neighbour {
            Some(neighbour) => {
                self.target.set_to(neighbour.position);
                *self.movement = Moving(neighbour.direction)
            }
            None => panic!("A ghost has no new target to move to")
        }
    }
}