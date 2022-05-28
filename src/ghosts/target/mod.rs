use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::{Neighbour, Position};
use crate::common::Direction;
use crate::common::Direction::*;
use crate::ghost_corners::GhostCorner;
use crate::ghost_house::GhostHousePositions;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::ghosts::state::{State, StateSetter};
use crate::ghosts::target::eaten::set_eaten_target;
use crate::ghosts::target::spawned::set_spawned_target;
use crate::pacman::Pacman;
use crate::random::Random;
use crate::state_skip_if;
use crate::target_skip_if;
use crate::walls::WallPositions;

mod spawned;
mod eaten;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_system(set_spawned_target::<Blinky>)
                    .with_system(set_spawned_target::<Pinky>)
                    .with_system(set_spawned_target::<Inky>)
                    .with_system(set_spawned_target::<Clyde>)
                    .with_system(set_scatter_target::<Blinky>)
                    .with_system(set_scatter_target::<Pinky>)
                    .with_system(set_scatter_target::<Inky>)
                    .with_system(set_scatter_target::<Clyde>)
                    .with_system(set_blinky_chase_target)
                    .with_system(set_pinky_chase_target)
                    .with_system(set_frightened_target)
                    .with_system(set_eaten_target::<Blinky>)
                    .with_system(set_eaten_target::<Pinky>)
                    .with_system(set_eaten_target::<Inky>)
                    .with_system(set_eaten_target::<Clyde>)
                    .label(TargetSetter)
                    .after(StateSetter)
            )
        ;
    }
}

/// Marks every system that sets a ghosts target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub struct TargetSetter;

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

fn set_scatter_target<G: Component>(
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut ghost_query: Query<(&mut Target, &mut Direction, &Position, &State), With<G>>,
    corner_query: Query<&Position, (With<G>, With<GhostCorner>)>,
) {
    for (mut target, mut direction, position, state) in ghost_query.iter_mut() {
        target_skip_if!(target set);
        state_skip_if!(state != State::Scatter);
        let nearest_corner = position.get_nearest_from(corner_query.iter());

        let next_target_neighbour = position.get_neighbours()
            .into_iter()
            .filter(|n| n.direction != direction.opposite())
            .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(nearest_corner, n_a, n_b))
            .unwrap_or_else(|| position.neighbour_behind(&direction));

        *direction = next_target_neighbour.direction;
        target.set(Vec3::from(&next_target_neighbour.position));
    }
}

fn set_blinky_chase_target(
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut blinky_query: Query<(&mut Target, &mut Direction, &Position, &State), With<Blinky>>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (mut target, mut direction, blinky_position, state) in blinky_query.iter_mut() {
        target_skip_if!(target set);
        state_skip_if!(state != State::Chase);
        for pacman_position in pacman_query.iter() {
            let next_target_neighbour = blinky_position.get_neighbours()
                .into_iter()
                .filter(|n| n.direction != direction.opposite())
                .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(pacman_position, n_a, n_b))
                .unwrap_or_else(|| blinky_position.neighbour_behind(&direction));

            *direction = next_target_neighbour.direction;
            target.set(Vec3::from(&next_target_neighbour.position));
        }
    }
}

// TODO: Bug. Pacman might not have a movement direction, which causes pinky to stand still when in chase.
fn set_pinky_chase_target(
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut pinky_query: Query<(&mut Target, &mut Direction, &Position, &State), (With<Pinky>, Without<Pacman>)>,
    pacman_query: Query<(&Position, &Direction), With<Pacman>>,
) {
    for (mut target, mut pinky_direction, pinky_position, state) in pinky_query.iter_mut() {
        target_skip_if!(target set);
        state_skip_if!(state != State::Chase);
        for (pacman_position, pacman_direction) in pacman_query.iter() {
            let pinky_target_pos = calculate_pinky_target_position(pacman_position, pacman_direction);

            let next_target_neighbour = pinky_position.get_neighbours()
                .into_iter()
                .filter(|n| n.direction != pinky_direction.opposite())
                .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(&pinky_target_pos, n_a, n_b))
                .unwrap_or_else(|| pinky_position.neighbour_behind(&pinky_direction));

            *pinky_direction = next_target_neighbour.direction;
            target.set(Vec3::from(&next_target_neighbour.position));
        }
    }
}

/// Return the pinky target position 4 fields in pacmans direction.
/// If pacman is idle, the field to its right is choosen.
fn calculate_pinky_target_position(
    pacman_position: &Position,
    pacman_direction: &Direction,
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
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    random: Res<Random>,
    mut query: Query<(&mut Target, &mut Direction, &Position, &State)>,
) {
    for (mut target, mut direction, position, state) in query.iter_mut() {
        target_skip_if!(target set);
        state_skip_if!(state != State::Frightened);
        let possible_neighbours = position.get_neighbours()
            .into_iter()
            .filter(|n| n.direction != direction.opposite())
            .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
            .collect::<Vec<_>>();

        let next_target_neighbour = match possible_neighbours.len() {
            0 => position.neighbour_behind(&direction),
            1 => possible_neighbours.get(0).unwrap().clone(),
            len => possible_neighbours.get(random.zero_to(len)).unwrap().clone()
        };
        *direction = next_target_neighbour.direction;
        target.set(Vec3::from(&next_target_neighbour.position));
    }
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}

#[macro_export]
macro_rules! target_skip_if {
    ($components:ident.$target:ident set) => {
        if $components.$target.is_set() {
            continue
        }
    };

    ($target:ident set) => {
        if $target.is_set() {
            continue
        }
    };

    ($target:ident not set) => {
        if $target.is_not_set() {
            continue
        }
    };
}