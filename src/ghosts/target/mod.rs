use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::board_dimensions::BoardDimensions;

use crate::common::position::{Neighbour, Position};
use crate::common::Direction;
use crate::common::Direction::*;
use crate::life_cycle::LifeCycle::*;
use crate::ghost_corners::GhostCorner;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::ghosts::state::{State, StateSetter};
use crate::ghosts::target::eaten::set_eaten_target;
use crate::ghosts::target::spawned::set_spawned_target;
use crate::map::board::Board;
use crate::pacman::Pacman;
use crate::random::Random;
use crate::state_skip_if;
use crate::target_skip_if;

mod spawned;
mod eaten;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(Running)
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
                    .with_system(set_inky_chase_target)
                    .with_system(set_clyde_chase_target)
                    .with_system(set_frightened_target)
                    .with_system(set_eaten_target::<Blinky>)
                    .with_system(set_eaten_target::<Pinky>)
                    .with_system(set_eaten_target::<Inky>)
                    .with_system(set_eaten_target::<Clyde>)
                    .label(TargetSetter)
                    .after(StateSetter)
            )
            .add_system_set(
                SystemSet::on_update(GhostEatenPause)
                    .with_system(set_spawned_target::<Blinky>)
                    .with_system(set_spawned_target::<Pinky>)
                    .with_system(set_spawned_target::<Inky>)
                    .with_system(set_spawned_target::<Clyde>)
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

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TargetComponents<'a> {
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
    state: &'a State
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

fn set_scatter_target<G: Component>(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    mut ghost_query: Query<TargetComponents, With<G>>,
    corner_query: Query<&Transform, (With<G>, With<GhostCorner>)>,
) {
    for mut components in ghost_query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Scatter);
        let nearest_corner_position = dimensions
            .trans_to_pos(components.transform)
            .get_nearest_position_from(corner_query.iter().map(|t| dimensions.trans_to_pos(t)));

        let next_target_neighbour = get_nearest_neighbour(
            &components,
            nearest_corner_position,
            &dimensions,
            |n| !board.position_is_wall_or_entrance(&n.position)
        );

        *components.direction = next_target_neighbour.direction;
        components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
    }
}

fn set_blinky_chase_target(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    mut blinky_query: Query<TargetComponents, With<Blinky>>,
    pacman_query: Query<&Transform, With<Pacman>>,
) {
    for mut components in blinky_query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Chase);
        for pacman_transform in pacman_query.iter() {
            let next_target_neighbour = get_nearest_neighbour(
                &components,
                dimensions.trans_to_pos(pacman_transform),
                &dimensions,
                |n| !board.position_is_wall_or_entrance(&n.position)
            );

            *components.direction = next_target_neighbour.direction;
            components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
        }
    }
}

fn set_pinky_chase_target(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    mut pinky_query: Query<TargetComponents, (With<Pinky>, Without<Pacman>)>,
    pacman_query: Query<(&Transform, &Direction), With<Pacman>>,
) {
    for mut components in pinky_query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Chase);
        for (pacman_transform, pacman_direction) in pacman_query.iter() {
            let pinky_target = calculate_pinky_target(&dimensions.trans_to_pos(pacman_transform), pacman_direction);

            let next_target_neighbour = get_nearest_neighbour(
                &components,
                pinky_target,
                &dimensions,
                |n| !board.position_is_wall_or_entrance(&n.position)
            );

            *components.direction = next_target_neighbour.direction;
            components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
        }
    }
}

/// Return the pinky target position 4 fields in pacmans direction.
/// If pacman is idle, the field to its right is choosen.
fn calculate_pinky_target(
    pacman_position: &Position,
    pacman_direction: &Direction,
) -> Position {
    let x = pacman_position.x;
    let y = pacman_position.y;

    match pacman_direction {
        Up => Position::new(x, y + 4),
        Down => Position::new(x, y - 4),
        Left => Position::new(x - 4, y),
        Right => Position::new(x + 4, y)
    }
}

fn set_inky_chase_target(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    blinky_query: Query<&Transform, With<Blinky>>,
    pacman_query: Query<(&Transform, &Direction), With<Pacman>>,
    mut inky_query: Query<TargetComponents, (With<Inky>, Without<Pacman>)>
) {
    for (pacman_transform, pacman_direction) in pacman_query.iter() {
        for blinky_transform in blinky_query.iter() {
            for mut components in inky_query.iter_mut() {
                target_skip_if!(components.target set);
                state_skip_if!(components.state != State::Chase);
                let target = calculate_inky_target(&dimensions.trans_to_pos(pacman_transform), pacman_direction, &dimensions.trans_to_pos(blinky_transform));
                let next_target_neighbour = get_nearest_neighbour(
                    &components,
                    target,
                    &dimensions,
                    |n| !board.position_is_wall_or_entrance(&n.position)
                );

                *components.direction = next_target_neighbour.direction;
                components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
            }
        }
    }
}

/// Inky is moving to a field calculated by using pacmans and blinkys position.
///
/// 1. You take a field pacman is facing with two fields distance
/// 2. You shoot a line from blinkys position trough this field
/// 3. You double this distance. The field this line is ending on is inkys target.
fn calculate_inky_target(
    pacman_position: &Position,
    pacman_direction: &Direction,
    blinky_position: &Position,
) -> Position {
    let position_pacman_is_facing = pacman_position.get_position_in_direction_with_offset(pacman_direction, 2);
    let x_diff = position_pacman_is_facing.x - blinky_position.x;
    let y_diff = position_pacman_is_facing.y - blinky_position.y;
    Position::new(blinky_position.x + 2 * x_diff, blinky_position.y + 2 * y_diff)
}

/// Clydes target is determined by his distance to pacman. If pacman is in an eight field distance
/// to clyde, clyde returns to his corner. If clyde is farther away, he targets pacmans direct
/// position instead.
fn set_clyde_chase_target(
    board: Res<Board>,
    dimensions: Res<BoardDimensions>,
    mut clyde_query: Query<TargetComponents, (With<Clyde>, Without<Pacman>)>,
    pacman_query: Query<&Transform, With<Pacman>>,
    corner_query: Query<&Transform, (With<Clyde>, With<GhostCorner>)>
) {
    for mut components in clyde_query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Chase);
        for pacman_transform in pacman_query.iter() {
            let target = if clyde_is_near_pacman(&components, &dimensions.trans_to_pos(pacman_transform), &dimensions) {
                dimensions
                    .trans_to_pos(components.transform)
                    .get_nearest_position_from(corner_query.iter().map(|t| dimensions.trans_to_pos(t)))
            } else {
                dimensions.trans_to_pos(pacman_transform)
            };

            let next_target_neighbour = get_nearest_neighbour(
                &components,
                target,
                &dimensions,
                |n| !board.position_is_wall_or_entrance(&n.position)
            );

            *components.direction = next_target_neighbour.direction;
            components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
        }
    }
}

fn clyde_is_near_pacman(components: &TargetComponentsItem, pacman_position: &Position, dimensions: &BoardDimensions) -> bool {
    let clyde_coordinates = components.transform.translation;
    let pacman_coordinates = dimensions.pos_to_vec(pacman_position, clyde_coordinates.z);
    let distance = clyde_coordinates.distance(pacman_coordinates);
    distance < dimensions.field() * 8.0
}

fn set_frightened_target(
    board: Res<Board>,
    random: Res<Random>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<TargetComponents>,
) {
    for mut components in query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Frightened);
        let possible_neighbours = dimensions.trans_to_pos(components.transform)
            .get_neighbours()
            .into_iter()
            .filter(|n| n.direction != components.direction.opposite())
            .filter(|n| !board.position_is_wall_or_entrance(&n.position))
            .collect::<Vec<_>>();

        let next_target_neighbour = match possible_neighbours.len() {
            0 => dimensions.trans_to_pos(components.transform).neighbour_behind(&components.direction),
            1 => possible_neighbours.get(0).unwrap().clone(),
            len => possible_neighbours.get(random.zero_to(len)).unwrap().clone()
        };
        *components.direction = next_target_neighbour.direction;
        components.target.set(dimensions.pos_to_vec(&next_target_neighbour.position, 0.0));
    }
}

/// Get the neighbour with the shortest distance (euclidean) to a given position. To filter not allowed
/// positions, a specific filter is provided.
///
/// It is generally not allowed for ghosts to turn around, so the position behind the ghost is always filtered. However,
/// if due to some circumstances (like bad map design) a ghost has no other way to go, we allow the pour soul to
/// turn around.
fn get_nearest_neighbour(components: &TargetComponentsItem, target: Position, dimensions: &BoardDimensions, position_filter: impl Fn(&Neighbour) -> bool) -> Neighbour {
    dimensions.trans_to_pos(components.transform).get_neighbours()
        .into_iter()
        .filter(|n| n.direction != components.direction.opposite())
        .filter(position_filter)
        .min_by(|n_a, n_b| minimal_distance_to_neighbours(&target, n_a, n_b))
        .unwrap_or_else(|| dimensions.trans_to_pos(components.transform).neighbour_behind(&components.direction))
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