use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::{Neighbour, Position};
use crate::common::MoveDirection;
use crate::common::MoveDirection::*;
use crate::ghost_house::GhostHousePositions;
use crate::is;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::ghosts::state::{Chase, Eaten, Frightened, Scatter, Spawned};
use crate::map::board::Board;
use crate::map::Element::*;
use crate::pacman::Pacman;
use crate::random::Random;
use crate::walls::WallPositions;

#[derive(Component, Deref, DerefMut)]
pub struct Target(pub Position);

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(set_spawned_target)
            .add_system(set_blinky_scatter_target)
            .add_system(set_pinky_scatter_target)
            .add_system(set_inky_scatter_target)
            .add_system(set_clyde_scatter_target)
            .add_system(set_blinky_chase_target)
            .add_system(set_pinky_chase_target)
            .add_system(set_frightened_target)
            .add_system(set_eaten_target)
        ;
    }
}

fn set_spawned_target(
    mut commands: Commands,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Spawned>, Without<Frightened>, Without<Eaten>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        let nearest_entrance_position = position.get_nearest_from(ghost_house_positions.entrances.iter());
        let next_target_neighbour = position.get_neighbours()
            .into_iter()
            .filter(|n| n.direction != direction.opposite())
            .filter(|n| match ghost_house_positions.position_is_entrance(&n.position) {
                true => !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_interior(&n.position),
                false => !wall_positions.position_is_wall(&n.position),
            })
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(nearest_entrance_position, n_a, n_b))
            .unwrap_or_else(|| position.neighbour_behind(&direction));

        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn set_blinky_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Blinky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &wall_positions,
            &ghost_house_positions,
            entity,
            &mut direction,
            position,
            board.get_position_matching(is!(BlinkyCorner))
        )
    }
}

fn set_pinky_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Pinky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &wall_positions,
            &ghost_house_positions,
            entity,
            &mut direction,
            position,
            board.get_position_matching(is!(PinkyCorner))
        )
    }
}

fn set_inky_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Inky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &wall_positions,
            &ghost_house_positions,
            entity,
            &mut direction,
            position,
            board.get_position_matching(is!(InkyCorner))
        )
    }
}

fn set_clyde_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Clyde>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &wall_positions,
            &ghost_house_positions,
            entity,
            &mut direction,
            position,
            board.get_position_matching(is!(ClydeCorner))
        )
    }
}

fn set_scatter_target(
    commands: &mut Commands,
    wall_positions: &WallPositions,
    ghost_house_positions: &GhostHousePositions,
    entity: Entity,
    direction: &mut MoveDirection,
    ghost_position: &Position,
    corner_position: &Position
) {
    let next_target_neighbour = ghost_position.get_neighbours()
        .into_iter()
        .filter(|n| n.direction != direction.opposite())
        .filter(|n| !wall_positions.position_is_wall(&n.position) && !ghost_house_positions.position_is_entrance(&n.position))
        .min_by(|n_a, n_b| minimal_distance_to_neighbours(corner_position, n_a, n_b))
        .unwrap_or_else(|| ghost_position.neighbour_behind(&direction));

    *direction = next_target_neighbour.direction;
    commands.entity(entity).insert(Target(next_target_neighbour.position));
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
            commands.entity(entity).insert(Target(next_target_neighbour.position));
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
            commands.entity(entity).insert(Target(next_target_neighbour.position));
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
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn set_eaten_target(
    mut commands: Commands,
    wall_positions: Res<WallPositions>,
    ghost_house_positions: Res<GhostHousePositions>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Eaten>, Without<Frightened>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        let nearest_spawn_position = position.get_nearest_from(ghost_house_positions.interior.iter());
        let next_target_neighbour = position.get_neighbours()
            .into_iter()
            .filter(|n| n.direction != direction.opposite())
            .filter(|n| !wall_positions.position_is_wall(&n.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(nearest_spawn_position, n_a, n_b))
            .unwrap_or_else(|| position.neighbour_behind(&direction));

        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}