use std::cmp::Ordering;

use bevy::prelude::*;

use crate::ghosts::GhostType;
use crate::common::{Neighbour, Position};
use crate::common::MoveDirection;
use crate::common::MoveDirection::*;
use crate::ghost_corners::GhostCorner;
use crate::ghost_house::{GhostHouse, GhostHousePositions};
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::ghosts::state::{Chase, Eaten, Frightened, Scatter, Spawned};
use crate::pacman::Pacman;
use crate::random::Random;
use crate::walls::WallPositions;

#[derive(Component, Deref, DerefMut)]
pub struct Target(pub Vec3);

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(set_spawned_target::<Blinky>)
            .add_system(set_spawned_target::<Pinky>)
            .add_system(set_spawned_target::<Inky>)
            .add_system(set_spawned_target::<Clyde>)
            .add_system(set_scatter_target::<Blinky>)
            .add_system(set_scatter_target::<Pinky>)
            .add_system(set_scatter_target::<Inky>)
            .add_system(set_scatter_target::<Clyde>)
            .add_system(set_blinky_chase_target)
            .add_system(set_pinky_chase_target)
            .add_system(set_frightened_target)
            .add_system(set_eaten_target::<Blinky>)
            .add_system(set_eaten_target::<Pinky>)
            .add_system(set_eaten_target::<Inky>)
            .add_system(set_eaten_target::<Clyde>)
        ;
    }
}

fn set_spawned_target<G: GhostType + Component + 'static>(
    mut commands: Commands,
    ghost_house: Res<GhostHouse>,
    mut query: Query<(Entity, &mut MoveDirection, &Transform), (With<G>, With<Spawned>, Without<Frightened>, Without<Eaten>, Without<Target>)>,
) {
    for (entity, mut direction, transform) in query.iter_mut() {
        let coordinates = transform.translation;
        let center = ghost_house.center_coordinates();
        let respawn = ghost_house.respawn_coordinates_of::<G>();

        if coordinates == center {
            *direction = Up;
            commands.entity(entity).insert(Target(ghost_house.coordinates_in_front_of_entrance()));
        } else if coordinates == respawn {
            *direction = match respawn.x < center.x {
                true => Right,
                false => Left
            };
            commands.entity(entity).insert(Target(ghost_house.center_coordinates()));
        }
    }
}

fn set_scatter_target<G: Component>(
    mut commands: Commands,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut ghost_query: Query<(Entity, &mut MoveDirection, &Position), (With<G>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
    corner_query: Query<&Position, (With<G>, With<GhostCorner>)>,
) {
    for (entity, mut direction, position) in ghost_query.iter_mut() {
        let nearest_corner = position.get_nearest_from(corner_query.iter());

        let next_target_neighbour = position.get_neighbours()
            .into_iter()
            .filter(|n| n.direction != direction.opposite())
            .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(nearest_corner, n_a, n_b))
            .unwrap_or_else(|| position.neighbour_behind(&direction));

        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(Vec3::from(&next_target_neighbour.position)));
    }
}

fn set_blinky_chase_target(
    mut commands: Commands,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut blinky_query: Query<(Entity, &mut MoveDirection, &Position), (With<Blinky>, With<Chase>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, mut direction, blinky_position) in blinky_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            let next_target_neighbour = blinky_position.get_neighbours()
                .into_iter()
                .filter(|n| n.direction != direction.opposite())
                .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(pacman_position, n_a, n_b))
                .unwrap_or_else(|| blinky_position.neighbour_behind(&direction));

            *direction = next_target_neighbour.direction;
            commands.entity(entity).insert(Target(Vec3::from(&next_target_neighbour.position)));
        }
    }
}

// TODO: Bug. Pacman might not have a movement direction, which causes pinky to stand still when in chase.
fn set_pinky_chase_target(
    mut commands: Commands,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut pinky_query: Query<(Entity, &mut MoveDirection, &Position), (With<Pinky>, With<Chase>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Pacman>, Without<Target>)>,
    pacman_query: Query<(&Position, &MoveDirection), With<Pacman>>,
) {
    for (entity, mut pinky_direction, pinky_position) in pinky_query.iter_mut() {
        for (pacman_position, pacman_direction) in pacman_query.iter() {
            let pinky_target_pos = calculate_pinky_target_position(pacman_position, pacman_direction);

            let next_target_neighbour = pinky_position.get_neighbours()
                .into_iter()
                .filter(|n| n.direction != pinky_direction.opposite())
                .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(&pinky_target_pos, n_a, n_b))
                .unwrap_or_else(|| pinky_position.neighbour_behind(&pinky_direction));

            *pinky_direction = next_target_neighbour.direction;
            commands.entity(entity).insert(Target(Vec3::from(&next_target_neighbour.position)));
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
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    random: Res<Random>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
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
        commands.entity(entity).insert(Target(Vec3::from(&next_target_neighbour.position)));
    }
}

fn set_eaten_target<G: Component + GhostType + 'static>(
    mut commands: Commands,
    ghost_house: Res<GhostHouse>,
    wall_positions: Res<WallPositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position, &Transform), (With<G>, With<Eaten>, Without<Frightened>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position, transform) in query.iter_mut() {
        let coordinates = transform.translation;
        let center = ghost_house.center_coordinates();
        let respawn = ghost_house.respawn_coordinates_of::<G>();
        let in_front_of_house = ghost_house.coordinates_in_front_of_entrance();

        if coordinates == respawn {
            // TODO: Bad. I need to this because the state changes at the next frame
            return;
        }

        if coordinates == in_front_of_house {
            *direction = Down;
            commands.entity(entity).insert(Target(center));
        } else if ghost_house.positions_in_front_of_entrance().into_iter().any(|pos| pos == position) {
            let position_coordinates = Vec3::from(position);
            *direction = match position_coordinates.x < in_front_of_house.x {
                true => Left,
                false => Right
            };
            commands.entity(entity).insert(Target(in_front_of_house));
        } else if coordinates == center {
            *direction = match respawn.x < center.x {
                true => Left,
                false => Right
            };
            commands.entity(entity).insert(Target(respawn));
        } else {
            let nearest_spawn_position = position.get_nearest_from(ghost_house.positions_in_front_of_entrance());
            let next_target_neighbour = position.get_neighbours()
                .into_iter()
                .filter(|n| n.direction != direction.opposite())
                .filter(|n| !wall_positions.position_is_wall(&n.position))
                .min_by(|n_a, n_b| minimal_distance_to_neighbours(nearest_spawn_position, n_a, n_b))
                .unwrap_or_else(|| position.neighbour_behind(&direction));

            *direction = next_target_neighbour.direction;
            commands.entity(entity).insert(Target(Vec3::from(&next_target_neighbour.position)));
        }
    }
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}