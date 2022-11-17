use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use bevy::utils::{HashMap, HashSet};

use crate::game::position::{Neighbour, Position};
use crate::game::direction::Direction;
use crate::game::direction::Direction::*;
use crate::constants::FIELD_DIMENSION;
use crate::game::ghost_house_gate::GhostHouseGate;
use crate::game_state::GameState::*;
use crate::game::ghosts::Ghost;
use crate::game::ghosts::Ghost::*;
use crate::game::state::{State, StateSetter};
use crate::game::state::State::*;
use crate::game::map::ghost_house::GhostSpawn;
use crate::game::map::{GhostCorner, Wall};
use crate::game::pacman::Pacman;
use crate::game::random::Random;

mod spawned;
mod eaten;

pub (in crate::game) struct TargetPlugin;

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
    random: Res<Random>,
    ghost_house_gate: Res<GhostHouseGate>,
    corner_query: Query<&GhostCorner>,
    wall_query: Query<&Transform, With<Wall>>,
    ghost_spawn_query: Query<&GhostSpawn>,
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
            &random,
            &ghost_house_gate,
            *pm_transform,
            *pm_dir,
            blinky_transform,
            &corner_query,
            &wall_query,
            &ghost_spawn_query,
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
    random: Res<Random>,
    ghost_house_gate: Res<GhostHouseGate>,
    corner_query: Query<&GhostCorner>,
    wall_query: Query<&Transform, With<Wall>>,
    ghost_spawn_query: Query<&GhostSpawn>,
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
            &random,
            &ghost_house_gate,
            *pm_transform,
            *pm_dir,
            blinky_transform,
            &corner_query,
            &wall_query,
            &ghost_spawn_query,
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
    random: &'a Random,
    ghost_house_gate: &'a GhostHouseGate,
    pacman_transform: Transform,
    pacman_direction: Direction,
    blinky_transform: Transform,
    corner_positions: HashMap<Ghost, Position>,
    wall_positions: HashSet<Position>,
    ghost_spawns: HashMap<Ghost, GhostSpawn>,
    components: &'a mut TargetComponentsItem<'b, 'c>,
}

impl<'a, 'b, 'c> TargetSetter<'a, 'b, 'c> {
    pub fn new(
        random: &'a Random,
        ghost_house_gate: &'a GhostHouseGate,
        pacman_transform: Transform,
        pacman_direction: Direction,
        blinky_transform: Transform,
        corner_query: &Query<&GhostCorner>,
        wall_query: &Query<&Transform, With<Wall>>,
        ghost_spawn_query: &Query<&GhostSpawn>,
        components: &'a mut TargetComponentsItem<'b, 'c>
    ) -> Self {
        let corner_positions = corner_query.iter().map(|corner| (corner.ghost, corner.position)).collect();
        let wall_positions = wall_query.iter().map(|transform| Position::from_vec(&transform.translation)).collect();
        let ghost_spawns = ghost_spawn_query.iter().map(|spawn| (spawn.ghost, *spawn)).collect();
        Self { random, ghost_spawns, ghost_house_gate, pacman_transform, pacman_direction, blinky_transform, corner_positions, wall_positions, components }
    }

    fn set_blinky_chase_target(&mut self) {
        let pacman_position = Position::from_vec(&self.pacman_transform.translation);
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
        let pacman_position = Position::from_vec(&self.pacman_transform.translation);
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
        let pacman_position = Position::from_vec(&self.pacman_transform.translation);
        let blinky_position = Position::from_vec(&self.blinky_transform.translation);
        let position_pacman_is_facing = pacman_position.get_position_in_direction_with_offset(&self.pacman_direction, 2);
        let x_diff = position_pacman_is_facing.x - blinky_position.x;
        let y_diff = position_pacman_is_facing.y - blinky_position.y;
        Position::new(blinky_position.x + 2 * x_diff, blinky_position.y + 2 * y_diff)
    }

    fn set_clyde_chase_target(&mut self) {
        let target = if self.clyde_is_near_pacman() {
            *self.corner_positions.get(&self.components.ghost).unwrap()
        } else {
            Position::from_vec(&self.pacman_transform.translation)
        };

        let next_target_neighbour = self.get_nearest_neighbour_to(target);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn clyde_is_near_pacman(&self) -> bool {
        let pacman_position = Position::from_vec(&self.pacman_transform.translation);
        let clyde_coordinates = self.components.transform.translation;
        let pacman_coordinates = pacman_position.to_vec(clyde_coordinates.z);
        let distance = clyde_coordinates.distance(pacman_coordinates);
        distance < FIELD_DIMENSION * 8.0
    }

    fn set_scatter_target(&mut self) {
        let corner_pos = *self.corner_positions.get(&self.components.ghost).unwrap();
        let next_target_neighbour = self.get_nearest_neighbour_to(corner_pos);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn set_frightened_target(&mut self) {
        let possible_neighbours = Position::from_vec(&self.components.transform.translation)
            .get_neighbours()
            .into_iter()
            .filter(|n| n.direction != self.components.direction.opposite())
            .filter(|n| !self.wall_positions.contains(&n.position))
            .collect::<Vec<_>>();
        let next_target_neighbour = match possible_neighbours.len() {
            0 => Position::from_vec(&self.components.transform.translation).neighbour_behind(&self.components.direction),
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
        Position::from_vec(&self.components.transform.translation).get_neighbours()
            .into_iter()
            .filter(|n| n.direction != self.components.direction.opposite())
            .filter(|n| !self.wall_positions.contains(&n.position))
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&target, n_a, n_b))
            .unwrap_or_else(|| Position::from_vec(&self.components.transform.translation).neighbour_behind(&self.components.direction))
    }

    fn set_target_to_neighbour(&mut self, neighbour: Neighbour) {
        *self.components.direction = neighbour.direction;
        self.components.target.set(neighbour.position.to_vec(0.0));
    }

    fn get_spawn(&self, ghost: Ghost) -> &GhostSpawn {
        self.ghost_spawns.get(&ghost).unwrap()
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