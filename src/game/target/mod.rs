use std::cmp::Ordering;

use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::core::prelude::*;

mod spawned;
mod eaten;

type Neighbour = (Pos, Dir);

pub(in crate::game) struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                set_target
                    .in_set(SetTarget)
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                Update,
                set_target_on_ghost_pause
                    .in_set(SetTarget)
                    .run_if(in_state(Game(GhostEatenPause))),
            )
        ;
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TargetComponents<'a> {
    ghost: &'a Ghost,
    target: &'a mut Target,
    direction: &'a mut Dir,
    transform: &'a Transform,
    state: &'a GhostState,
}

fn set_target(
    random: Res<Random>,
    ghost_house_gate: Res<GhostHouseGate>,
    corner_query: Query<(&GhostCorner, &Tiles)>,
    wall_query: Query<&Transform, With<Wall>>,
    ghost_spawn_query: Query<&GhostSpawn>,
    pacman_query: Query<(&Transform, &Dir), With<Pacman>>,
    one_ways: Query<&Tiles, With<OneWay>>,
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
            &one_ways,
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
fn set_target_on_ghost_pause(
    random: Res<Random>,
    ghost_house_gate: Res<GhostHouseGate>,
    corner_query: Query<(&GhostCorner, &Tiles)>,
    wall_query: Query<&Transform, With<Wall>>,
    ghost_spawn_query: Query<&GhostSpawn>,
    pacman_query: Query<(&Transform, &Dir), With<Pacman>>,
    one_ways: Query<&Tiles, With<OneWay>>,
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
            &one_ways,
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
    pacman_direction: Dir,
    blinky_transform: Transform,
    corner_positions: HashMap<Ghost, Pos>,
    wall_positions: HashSet<Pos>,
    ghost_spawns: HashMap<Ghost, GhostSpawn>,
    one_ways: HashSet<Pos>,
    components: &'a mut TargetComponentsItem<'b, 'c>,
}

impl<'a, 'b, 'c> TargetSetter<'a, 'b, 'c> {
    pub fn new(
        random: &'a Random,
        ghost_house_gate: &'a GhostHouseGate,
        pacman_transform: Transform,
        pacman_direction: Dir,
        blinky_transform: Transform,
        corner_query: &Query<(&GhostCorner, &Tiles)>,
        wall_query: &Query<&Transform, With<Wall>>,
        ghost_spawn_query: &Query<&GhostSpawn>,
        one_ways: &Query<&Tiles, With<OneWay>>,
        components: &'a mut TargetComponentsItem<'b, 'c>,
    ) -> Self {
        let corner_positions = corner_query.iter().map(|(corner, tiles)| (**corner, tiles.to_pos())).collect();
        let wall_positions = wall_query.iter().map(|transform| Pos::from_vec3(transform.translation)).collect();
        let ghost_spawns = ghost_spawn_query.iter().map(|spawn| (spawn.ghost, *spawn)).collect();
        let one_ways = one_ways.iter().map(|t| t.to_pos()).collect();

        Self {
            random,
            ghost_spawns,
            ghost_house_gate,
            pacman_transform,
            pacman_direction,
            blinky_transform,
            corner_positions,
            wall_positions,
            one_ways,
            components
        }
    }

    fn set_blinky_chase_target(&mut self) {
        let pacman_position = Pos::from_vec3(self.pacman_transform.translation);
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
    fn calculate_pinky_target(&self) -> Pos {
        let pacman_position = Pos::from_vec3(self.pacman_transform.translation);
        let x = pacman_position.x();
        let y = pacman_position.y();

        match self.pacman_direction {
            Up => Pos::new(x, y + 4),
            Down => Pos::new(x, y - 4),
            Left => Pos::new(x - 4, y),
            Right => Pos::new(x + 4, y),
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
    fn calculate_inky_target(&self) -> Pos {
        let pacman_position = Pos::from_vec3(self.pacman_transform.translation);
        let blinky_position = Pos::from_vec3(self.blinky_transform.translation);
        let position_pacman_is_facing = pacman_position.position_in_direction(self.pacman_direction, 2);
        let x_diff = position_pacman_is_facing.x() - blinky_position.x();
        let y_diff = position_pacman_is_facing.y() - blinky_position.y();
        Pos::new(blinky_position.x() + 2 * x_diff, blinky_position.y() + 2 * y_diff)
    }

    fn set_clyde_chase_target(&mut self) {
        let target = if self.clyde_is_near_pacman() {
            *self.corner_positions.get(self.components.ghost).unwrap()
        } else {
            Pos::from_vec3(self.pacman_transform.translation)
        };

        let next_target_neighbour = self.get_nearest_neighbour_to(target);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn clyde_is_near_pacman(&self) -> bool {
        let pacman_position = Pos::from_vec3(self.pacman_transform.translation);
        let clyde_coordinates = self.components.transform.translation;
        let pacman_coordinates = pacman_position.to_vec3(clyde_coordinates.z);
        let distance = clyde_coordinates.distance(pacman_coordinates);
        distance < FIELD_SIZE * 8.0
    }

    fn set_scatter_target(&mut self) {
        let corner_pos = *self.corner_positions.get(self.components.ghost).unwrap();
        let next_target_neighbour = self.get_nearest_neighbour_to(corner_pos);
        self.set_target_to_neighbour(next_target_neighbour)
    }

    fn set_frightened_target(&mut self) {
        let ghost_pos = Pos::from_vec3(self.components.transform.translation);
        let opposite_dir = self.components.direction.opposite();

        let possible_neighbours = ghost_pos
            .neighbours_with_directions()
            .into_iter()
            .filter(|(_, dir)| *dir != opposite_dir)
            .filter(|(pos, _)| !self.wall_positions.contains(pos))
            .filter(|(_, dir)| if self.is_on_one_way(ghost_pos) {
                *dir == Left || *dir == Right
            } else {
                true
            })
            .collect::<Vec<_>>();
        let next_target_neighbour = match possible_neighbours.len() {
            0 => (ghost_pos.neighbour_in_direction(opposite_dir), opposite_dir),
            1 => possible_neighbours.get(0).unwrap().clone(),
            len => possible_neighbours.get(self.random.zero_to(len)).unwrap().clone()
        };
        self.set_target_to_neighbour(next_target_neighbour)
    }

    /// Get the neighbour with the shortest distance (euclidean) to a given position. To filter not allowed
    /// positions, a specific filter is provided.
    ///
    /// It is generally not allowed for ghosts to turn around, so the position behind the ghost is always filtered. However,
    /// if due to some circumstances (like bad map design) a ghost has no other way to go, we allow the poor soul to
    /// turn around.
    fn get_nearest_neighbour_to(&self, target: Pos) -> Neighbour {
        let ghost_pos = Pos::from_vec3(self.components.transform.translation);
        let opposite_dir = self.components.direction.opposite();

        ghost_pos
            .neighbours_with_directions()
            .into_iter()
            .filter(|(_, dir)| *dir != opposite_dir)
            .filter(|(pos, _)| !self.wall_positions.contains(pos))
            .filter(|(_, dir)| if self.is_on_one_way(ghost_pos) {
                *dir == Left || *dir == Right
            } else {
                true
            })
            .min_by(|n_a, n_b| minimal_distance_to_neighbours(&target, n_a, n_b))
            .unwrap_or_else(|| (ghost_pos.neighbour_in_direction(opposite_dir), opposite_dir))
    }

    fn is_on_one_way(&self, pos: Pos) -> bool {
        self.one_ways.contains(&pos)
    }

    fn set_target_to_neighbour(&mut self, neighbour: Neighbour) {
        *self.components.direction = neighbour.1;
        self.components.target.set(neighbour.0.to_vec3(0.0));
    }

    fn get_spawn(&self, ghost: Ghost) -> &GhostSpawn {
        self.ghost_spawns.get(&ghost).unwrap()
    }
}

/// Get the transform of blinky.
fn get_blinky_transform(query: &Query<TargetComponents, Without<Pacman>>) -> Transform {
    query.iter()
        .filter(|comps| comps.ghost == &Blinky)
        .map(|comps| *comps.transform)
        .next()
        .expect("there should be one blinky")
}

fn minimal_distance_to_neighbours(big_target: &Pos, neighbour_a: &Neighbour, neighbour_b: &Neighbour) -> Ordering {
    minimal_distance_to_positions(big_target, &neighbour_a.0, &neighbour_b.0)
}

fn minimal_distance_to_positions(big_target: &Pos, position_a: &Pos, position_b: &Pos) -> Ordering {
    big_target.distance(position_a).partial_cmp(&big_target.distance(position_b)).unwrap()
}

