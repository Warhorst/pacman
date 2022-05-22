use std::cmp::Ordering;

use bevy::prelude::*;

use crate::common::Position;
use crate::common::MoveDirection;
use crate::common::MoveDirection::*;
use crate::is;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::ghosts::state::{Chase, Eaten, Frightened, Scatter, Spawned};
use crate::map::board::Board;
use crate::map::Element::*;
use crate::map::Neighbour;
use crate::pacman::Pacman;
use crate::random::Random;

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
    board: Res<Board>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Spawned>, Without<Frightened>, Without<Eaten>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        let entrance_positions = board.get_positions_matching(is!(GhostHouseEntrance {..}));

        let nearest_entrance_position = entrance_positions.into_iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost wall on the map");
        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_entrance_position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_wall_in_spawn(&board, position, neighbour),
        );

        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn set_blinky_scatter_target(
    mut commands: Commands,
    board: Res<Board>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Blinky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &board,
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
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Pinky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &board,
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
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Inky>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &board,
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
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Clyde>, With<Scatter>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        set_scatter_target(
            &mut commands,
            &board,
            entity,
            &mut direction,
            position,
            board.get_position_matching(is!(ClydeCorner))
        )
    }
}

fn set_scatter_target(
    commands: &mut Commands,
    board: &Board,
    entity: Entity,
    direction: &mut MoveDirection,
    ghost_position: &Position,
    corner_position: &Position
) {
    let next_target_neighbour = get_neighbour_nearest_to_target(
        ghost_position,
        corner_position,
        &board,
        &direction,
        |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
    );
    *direction = next_target_neighbour.direction;
    commands.entity(entity).insert(Target(next_target_neighbour.position));
}

fn set_blinky_chase_target(
    mut commands: Commands,
    board: Res<Board>,
    mut blinky_query: Query<(Entity, &mut MoveDirection, &Position), (With<Blinky>, With<Chase>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
    pacman_query: Query<&Position, With<Pacman>>,
) {
    for (entity, mut direction, blinky_position) in blinky_query.iter_mut() {
        for pacman_position in pacman_query.iter() {
            let next_target_neighbour = get_neighbour_nearest_to_target(
                blinky_position,
                pacman_position,
                &board,
                &direction,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
            *direction = next_target_neighbour.direction;
            commands.entity(entity).insert(Target(next_target_neighbour.position));
        }
    }
}

// TODO: Bug. Pacman might not have a movement direction, which causes pinky to stand still when in chase.
fn set_pinky_chase_target(
    mut commands: Commands,
    board: Res<Board>,
    mut pinky_query: Query<(Entity, &mut MoveDirection, &Position), (With<Pinky>, With<Chase>, Without<Frightened>, Without<Eaten>, Without<Spawned>, Without<Pacman>, Without<Target>)>,
    pacman_query: Query<(&Position, &MoveDirection), With<Pacman>>,
) {
    for (entity, mut pinky_direction, pinky_position) in pinky_query.iter_mut() {
        for (pacman_position, pacman_direction) in pacman_query.iter() {
            let next_target_neighbour = get_neighbour_nearest_to_target(
                pinky_position,
                &calculate_pinky_target_position(pacman_position, pacman_direction),
                &board,
                &pinky_direction,
                |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
            );
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
    board: Res<Board>,
    random: Res<Random>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Frightened>, Without<Eaten>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        let possible_neighbours = get_possible_neighbours(
            position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_wall(&board, &neighbour.position),
        );

        let next_target_neighbour = match possible_neighbours.len() {
            0 => board.neighbour_behind(&position, &direction),
            1 => possible_neighbours.get(0).unwrap().clone(),
            len => possible_neighbours.get(random.zero_to(len)).unwrap().clone()
        };
        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn set_eaten_target(
    mut commands: Commands,
    board: Res<Board>,
    mut query: Query<(Entity, &mut MoveDirection, &Position), (With<Eaten>, Without<Frightened>, Without<Spawned>, Without<Target>)>,
) {
    for (entity, mut direction, position) in query.iter_mut() {
        let ghost_spawn_positions = board.get_positions_matching(is!(GhostHouse));
        let nearest_spawn_position = &ghost_spawn_positions.iter()
            .min_by(|pos_a, pos_b| minimal_distance_to_positions(&position, pos_a, pos_b))
            .expect("There should at least be one ghost spawn on the map");

        let next_target_neighbour = get_neighbour_nearest_to_target(
            position,
            nearest_spawn_position,
            &board,
            &direction,
            |neighbour| neighbour_is_no_normal_wall(&board, &neighbour.position),
        );
        *direction = next_target_neighbour.direction;
        commands.entity(entity).insert(Target(next_target_neighbour.position));
    }
}

fn get_neighbour_nearest_to_target<'a, F: Fn(&Neighbour<'a>) -> bool>(
    ghost_position: &Position,
    target_position: &Position,
    board: &'a Board,
    direction: &MoveDirection,
    field_filter: F,
) -> Neighbour<'a> {
    get_possible_neighbours(ghost_position, board, direction, field_filter)
        .into_iter()
        .min_by(|n_a, n_b| minimal_distance_to_neighbours(target_position, n_a, n_b))
        .unwrap_or_else(|| board.neighbour_behind(ghost_position, direction))
}

fn get_possible_neighbours<'a, F: Fn(&Neighbour<'a>) -> bool>(
    ghost_position: &Position,
    board: &'a Board,
    direction: &MoveDirection,
    field_filter: F,
) -> Vec<Neighbour<'a>> {
    board.neighbours_of(ghost_position)
        .into_iter()
        .filter(|neighbour| neighbour_not_in_opposite_direction(direction, neighbour))
        .filter(|neighbour| (field_filter)(neighbour))
        .collect()
}

fn neighbour_is_no_wall_in_spawn(board: &Board, ghost_position: &Position, neighbour: &Neighbour) -> bool {
    match board.position_matches_filter(ghost_position, is!(GhostHouseEntrance {..})) {
        true => !neighbour.elements_match_filter(is!(Wall {..} | GhostHouse)),
        false => !neighbour.elements_match_filter(is!(Wall {..}))
    }
}

fn neighbour_is_no_wall(board: &Board, position: &Position) -> bool {
    !board.position_matches_filter(position, is!(Wall {..} | GhostHouseEntrance {..} | InvisibleWall))
}

fn neighbour_is_no_normal_wall(board: &Board, position: &Position) -> bool {
    !board.position_matches_filter(position, is!(Wall {..}))
}

fn neighbour_not_in_opposite_direction(direction: &MoveDirection, neighbour: &Neighbour) -> bool {
    neighbour.direction != direction.opposite()
}

fn minimal_distance_to_neighbours(big_target: &Position, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.position, &neighbour_b.position)
}

fn minimal_distance_to_positions(big_target: &Position, position_a: &Position, position_b: &Position) -> Ordering {
    big_target.distance_to(position_a).cmp(&big_target.distance_to(position_b))
}