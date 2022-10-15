use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::ghosts::state::State;
use crate::{state_skip_if, target_skip_if};
use crate::board_dimensions::BoardDimensions;
use crate::ghost_house::GhostHouse;
use crate::ghosts::target::Target;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::ghost_house_gate::GhostHouseGate;
use crate::common::XYEqual;
use crate::ghosts::Ghost;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct SpawnedTargetComponents<'a> {
    ghost: &'a Ghost,
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
    state: &'a State,
}

/// Determine the next target coordinates for a ghost when in "Spawned" state.
///
/// A ghost can only leave the house if their dot counter reached its predefined limit.
/// When ready to leave, the ghost moves from its spawn to the house center, from the center to
/// the entrance and from the entrance were ever his destiny leads him.
///
/// If a ghost cannot leave the house yet, he just moves around, eager to leave and hunt pacman.
pub fn set_spawned_target(
    ghost_house: Res<GhostHouse>,
    ghost_house_gate: Res<GhostHouseGate>,
    dimensions: Res<BoardDimensions>,
    mut query: Query<SpawnedTargetComponents>,
) {
    for mut components in query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Spawned);

        if ghost_house_gate.ghost_can_leave_house(components.ghost) {
            leave_house(&mut components, &ghost_house)
        } else {
            bounce_around(&mut components, &ghost_house, &dimensions)
        }
    }
}

/// If a ghost cannot leave the ghost house, he just moves around.
fn bounce_around(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse, dimensions: &BoardDimensions) {
    let coordinates = components.transform.translation;
    let respawn = ghost_house.respawn_coordinates_of(components.ghost);
    let above_respawn = coordinates_slightly_in_direction(respawn, ghost_house.entrance_direction, dimensions);
    let below_respawn = coordinates_slightly_in_direction(respawn, ghost_house.entrance_direction.opposite(), dimensions);

    if coordinates.xy_equal_to(&respawn) {
        match *components.direction {
            dir if dir == ghost_house.entrance_direction => components.target.set(above_respawn),
            _ => components.target.set(below_respawn)
        };
    } else if coordinates.xy_equal_to(&above_respawn) {
        components.target.set(below_respawn);
        *components.direction = ghost_house.entrance_direction.opposite();
    } else if coordinates.xy_equal_to(&below_respawn) {
        components.target.set(above_respawn);
        *components.direction = ghost_house.entrance_direction;
    }
}

fn leave_house(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    if is_near_center(components, ghost_house) {
        move_to_entrance(components, ghost_house)
    } else if is_near_spawn(components, ghost_house) {
        move_near_center(components, ghost_house)
    }
}

fn is_near_center(components: &SpawnedTargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    let coordinates = components.transform.translation;
    let center = ghost_house.center_coordinates();

    match ghost_house.entrance_direction {
        Up | Down => coordinates.x == center.x,
        Left | Right => coordinates.y == center.y,
    }
}

fn move_to_entrance(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    *components.direction = ghost_house.entrance_direction;
    components.target.set(ghost_house.coordinates_in_front_of_entrance());
}

fn is_near_spawn(components: &SpawnedTargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    let coordinates = components.transform.translation;
    let respawn = ghost_house.respawn_coordinates_of(components.ghost);
    match ghost_house.entrance_direction {
        Up | Down => coordinates.x == respawn.x,
        Left | Right => coordinates.y == respawn.y
    }
}

fn move_near_center(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    let coordinates = components.transform.translation;
    let center = ghost_house.center_coordinates();
    let respawn = ghost_house.respawn_coordinates_of(components.ghost);

    *components.direction = match ghost_house.entrance_direction {
        Up | Down => match respawn.x < center.x {
            true => Right,
            false => Left
        },
        Left | Right => match respawn.y < center.y {
            true => Up,
            false => Down
        }
    };

    match ghost_house.entrance_direction {
        Up | Down => components.target.set(Vec3::new(center.x, coordinates.y, 0.0)),
        Left | Right => components.target.set(Vec3::new(coordinates.x, center.y, 0.0)),
    }
}

/// A ghost in the ghost house does not walk a full field in the ghost house (because he would clip into the wall).
/// When bouncing around in the ghost house, he only moves slightly in one direction.
fn coordinates_slightly_in_direction(v: Vec3, d: Direction, dimensions: &BoardDimensions) -> Vec3 {
    let distance = dimensions.field() / 2.0;
    match d {
        Up => Vec3::new(v.x, v.y + distance, v.z),
        Down => Vec3::new(v.x, v.y - distance, v.z),
        Left => Vec3::new(v.x - distance, v.y, v.z),
        Right => Vec3::new(v.x + distance, v.y, v.z),
    }
}