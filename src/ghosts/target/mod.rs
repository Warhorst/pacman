use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::board_dimensions::BoardDimensions;

use crate::common::position::{Neighbour, Position};
use crate::common::Direction;
use crate::common::Direction::*;
use crate::ghost_corners::GhostCorners;
use crate::ghost_house::GhostHouse;
use crate::ghost_house_gate::GhostHouseGate;
use crate::life_cycle::LifeCycle::*;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::state::{State, StateSetter};
use crate::ghosts::state::State::*;
use crate::map::board::Board;
use crate::pacman::Pacman;
use crate::random::Random;

mod spawned;
mod eaten;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(set_target)
                    .label(LTargetSetter)
                    .after(StateSetter)
            )
            .add_system_set(
                SystemSet::on_update(GhostEatenPause)
                    .with_system(set_target_on_ghost_pause)
                    .label(LTargetSetter)
                    .after(StateSetter)
            )
        ;
    }
}

/// Marks every system that sets a ghosts target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub struct LTargetSetter;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TargetComponents<'a> {
    ghost: &'a Ghost,
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
    state: &'a State,
}

fn set_target(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    random: Res<Random>,
    ghost_corners: Res<GhostCorners>,
    ghost_house: Res<GhostHouse>,
    ghost_house_gate: Res<GhostHouseGate>,
    pacman_query: Query<(&Transform, &Direction), With<Pacman>>,
    mut ghost_query: Query<TargetComponents, Without<Pacman>>,
) {
    let (pm_transform, pm_dir) = pacman_query.single();
    let blinky_transform = get_blinky_transform(&ghost_query);

    for mut components in &mut ghost_query {
        if components.target.is_set() {
            continue;
        }

        let (state, ghost) = (*components.state, *components.ghost);
        let mut setter = TargetSetter::new(
            &board,
            &dimensions,
            &random,
            &ghost_corners,
            &ghost_house,
            &ghost_house_gate,
            *pm_transform,
            *pm_dir,
            blinky_transform,
            &mut components,
        );

        match state {
            Chase => match ghost {
                Blinky => setter.set_blinky_chase_target(),
                Pinky => setter.set_pinky_chase_target(),
                Inky => setter.set_inky_chase_target(),
                Clyde => setter.set_clyde_chase_target(),
            },
            Scatter => setter.set_scatter_target(),
            Frightened => setter.set_frightened_target(),
            Eaten => setter.set_eaten_target(),
            Spawned => setter.set_spawned_target(),
        }
    }
}

/// Set the target when on ghost pause (meaning only eaten and spawned)
///
/// TODO: I provide more resources than necessary, but I want to reuse the TargetSetter. Not ideal, but the best solution for now.
fn set_target_on_ghost_pause(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    random: Res<Random>,
    ghost_corners: Res<GhostCorners>,
    ghost_house: Res<GhostHouse>,
    ghost_house_gate: Res<GhostHouseGate>,
    pacman_query: Query<(&Transform, &Direction), With<Pacman>>,
    mut ghost_query: Query<TargetComponents, Without<Pacman>>,
) {
    let (pm_transform, pm_dir) = pacman_query.single();
    let blinky_transform = get_blinky_transform(&ghost_query);

    for mut components in &mut ghost_query {
        if components.target.is_set() {
            continue;
        }

        let state = *components.state;
        let mut setter = TargetSetter::new(
            &board,
            &dimensions,
            &random,
            &ghost_corners,
            &ghost_house,
            &ghost_house_gate,
            *pm_transform,
            *pm_dir,
            blinky_transform,
            &mut components,
        );

        match state {
            Eaten => setter.set_eaten_target(),
            Spawned => setter.set_spawned_target(),
            _ => continue
        }
    }
}

struct TargetSetter<'a, 'b, 'c> {
    board: &'a Board,
    dimensions: &'a BoardDimensions,
    random: &'a Random,
    ghost_corners: &'a GhostCorners,
    ghost_house: &'a GhostHouse,
    ghost_house_gate: &'a GhostHouseGate,
    pacman_transform: Transform,
    pacman_direction: Direction,
    blinky_transform: Transform,
    components: &'a mut TargetComponentsItem<'b, 'c>,
}

impl<'a, 'b, 'c> TargetSetter<'a, 'b, 'c> {
    pub fn new(board: &'a Board, dimensions: &'a BoardDimensions, random: &'a Random, ghost_corners: &'a GhostCorners, ghost_house: &'a GhostHouse, ghost_house_gate: &'a GhostHouseGate, pacman_transform: Transform, pacman_direction: Direction, blinky_transform: Transform, components: &'a mut TargetComponentsItem<'b, 'c>) -> Self {
        Self { board, dimensions, random, ghost_corners, ghost_house, ghost_house_gate, pacman_transform, pacman_direction, blinky_transform, components }
    }

    fn set_blinky_chase_target(&mut self) {
        let pacman_position = self.dimensions.trans_to_pos(&self.pacman_transform);
        let next_target_neighbour = self.get_nearest_neighbour_to(pacman_position);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn set_pinky_chase_target(&mut self) {
        let pinky_target = self.calculate_pinky_target();
        let next_target_neighbour = self.get_nearest_neighbour_to(pinky_target);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    /// Return the pinky target position 4 fields in pacmans direction.
    /// If pacman is idle, the field to its right is choosen.
    fn calculate_pinky_target(&self) -> Position {
        let pacman_position = &self.dimensions.trans_to_pos(&self.pacman_transform);
        let x = pacman_position.x;
        let y = pacman_position.y;

        match self.pacman_direction {
            Up => Position::new(x, y + 4),
            Down => Position::new(x, y - 4),
            Left => Position::new(x - 4, y),
            Right => Position::new(x + 4, y)
        }
    }

    fn set_inky_chase_target(&mut self) {
        let target = self.calculate_inky_target();
        let next_target_neighbour = self.get_nearest_neighbour_to(target);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    /// Inky is moving to a field calculated by using pacmans and blinkys position.
    ///
    /// 1. You take a field pacman is facing with two fields distance
    /// 2. You shoot a line from blinkys position trough this field
    /// 3. You double this distance. The field this line is ending on is inkys target.
    fn calculate_inky_target(&self) -> Position {
        let pacman_position = self.dimensions.trans_to_pos(&self.pacman_transform);
        let blinky_position = self.dimensions.trans_to_pos(&self.blinky_transform);
        let position_pacman_is_facing = pacman_position.get_position_in_direction_with_offset(&self.pacman_direction, 2);
        let x_diff = position_pacman_is_facing.x - blinky_position.x;
        let y_diff = position_pacman_is_facing.y - blinky_position.y;
        Position::new(blinky_position.x + 2 * x_diff, blinky_position.y + 2 * y_diff)
    }

    fn set_clyde_chase_target(&mut self) {
        let target = if self.clyde_is_near_pacman() {
            self.ghost_corners.get_corner(self.components.ghost)
        } else {
            self.dimensions.trans_to_pos(&self.pacman_transform)
        };

        let next_target_neighbour = self.get_nearest_neighbour_to(target);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn clyde_is_near_pacman(&self) -> bool {
        let pacman_position = self.dimensions.trans_to_pos(&self.pacman_transform);
        let clyde_coordinates = self.components.transform.translation;
        let pacman_coordinates = self.dimensions.pos_to_vec(&pacman_position, clyde_coordinates.z);
        let distance = clyde_coordinates.distance(pacman_coordinates);
        distance < self.dimensions.field() * 8.0
    }

    fn set_scatter_target(&mut self) {
        let corner_pos = self.ghost_corners.get_corner(self.components.ghost);
        let next_target_neighbour = self.get_nearest_neighbour_to(corner_pos);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn set_frightened_target(&mut self) {
        let possible_neighbours = self.dimensions.trans_to_pos(self.components.transform)
            .get_neighbours()
            .into_iter()
            .filter(|n| n.direction != self.components.direction.opposite())
            .filter(|n| !self.board.position_is_wall_or_entrance(&n.position))
            .collect::<Vec<_>>();
        let next_target_neighbour = match possible_neighbours.len() {
            0 => self.dimensions.trans_to_pos(self.components.transform).neighbour_behind(&self.components.direction),
            1 => possible_neighbours.get(0).unwrap().clone(),
            len => possible_neighbours.get(self.random.zero_to(len)).unwrap().clone()
        };
        self.set_target_to_neighbour(next_target_neighbour)
    }

    /// Get the neighbour with the shortest distance (euclidean) to a given position. To filter not allowed
    /// positions, a specific filter is provided.
    ///
    /// It is generally not allowed for ghosts to turn around, so the position behind the ghost is always filtered. However,
    /// if due to some circumstances (like bad map design) a ghost has no other way to go, we allow the pour soul to
    /// turn around.
    fn get_nearest_neighbour_to(&self, target: Position) -> Neighbour {
        self.dimensions.trans_to_pos(self.components.transform).get_neighbours()
            .into_iter()
            .filter(|n| n.direction != self.components.direction.opposite())
            .filter(|n| !self.board.position_is_wall_or_entrance(&n.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&target, n_a, n_b))
            .unwrap_or_else(|| self.dimensions.trans_to_pos(self.components.transform).neighbour_behind(&self.components.direction))
    }

    fn set_target_to_neighbour(&mut self, neighbour: Neighbour) {
        *self.components.direction = neighbour.direction;
        self.components.target.set(self.dimensions.pos_to_vec(&neighbour.position, 0.0));
    }
}

/// Get the transform of blinky.
///
/// TODO: If one day more than one blinky exists return a set and choose the nearest one
fn get_blinky_transform(query: &Query<TargetComponents, Without<Pacman>>) -> Transform {
    query.iter()
        .filter(|comps| comps.ghost == &Blinky)
        .map(|comps| *comps.transform)
        .next()
        .expect("there should be one blinky")
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}

#[derive(Component)]
pub struct Target {
    coordinates: Option<Vec3>,
}

impl Target {
    pub fn new() -> Self {
        Target { coordinates: None }
    }

    pub fn is_set(&self) -> bool {
        self.coordinates.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    /// Return the coordinates without checking if they are present.
    /// The check should happen somewhere else anyway.
    pub fn get(&self) -> Vec3 {
        self.coordinates.unwrap()
    }

    pub fn set(&mut self, coordinates: Vec3) {
        self.coordinates = Some(coordinates)
    }

    pub fn clear(&mut self) {
        self.coordinates = None
    }
}