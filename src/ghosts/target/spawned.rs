use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;
use crate::ghosts::state::State;
use crate::{state_skip_if, target_skip_if};
use crate::constants::FIELD_DIMENSION;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{DotCounter, GhostType};
use crate::ghosts::target::Target;
use crate::common::Direction;
use crate::common::Direction::*;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct SpawnedTargetComponents<'a> {
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
    state: &'a State,
    dot_counter: &'a DotCounter
}

/// Determine the next target coordinates for a ghost when in "Spawned" state.
///
/// A ghost can only leave the house if their dot counter reached its predefined limit.
/// When ready to leave, the ghost moves from its spawn to the house center, from the center to
/// the entrance and from the entrance were ever his destiny leads him.
///
/// If a ghost cannot leave the house yet, he just moves around, eager to leave and hunt pacman.
pub fn set_spawned_target<G: GhostType + Component + 'static>(
    ghost_house: Res<GhostHouse>,
    mut query: Query<SpawnedTargetComponents, With<G>>,
) {
    for mut components in query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Spawned);

        if components.dot_counter.is_active() {
            bounce_around::<G>(&mut components, &ghost_house)
        } else {
            leave_house::<G>(&mut components, &ghost_house)
        }
    }
}

/// If a ghost cannot leave the ghost house, he just moves around.
fn bounce_around<G: GhostType + Component + 'static>(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    let coordinates = components.transform.translation;
    let respawn = ghost_house.respawn_coordinates_of::<G>();
    let above_respawn = coordinates_slightly_in_direction(respawn, ghost_house.entrance_direction);
    let below_respawn = coordinates_slightly_in_direction(respawn, ghost_house.entrance_direction.opposite());

    if coordinates == respawn {
        match *components.direction {
            dir if dir == ghost_house.entrance_direction => components.target.set(above_respawn),
            _ => components.target.set(below_respawn)
        };
    } else if coordinates == above_respawn {
        components.target.set(below_respawn);
        *components.direction = ghost_house.entrance_direction.opposite();
    } else if coordinates == below_respawn {
        components.target.set(above_respawn);
        *components.direction = ghost_house.entrance_direction;
    }
}

fn leave_house<G: GhostType + Component + 'static>(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    if is_near_center(components, ghost_house) {
        move_to_entrance(components, ghost_house)
    } else if is_near_spawn::<G>(components, ghost_house) {
        move_near_center::<G>(components, ghost_house)
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

fn is_near_spawn<G: GhostType + Component + 'static>(components: &SpawnedTargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    let coordinates = components.transform.translation;
    let respawn = ghost_house.respawn_coordinates_of::<G>();
    match ghost_house.entrance_direction {
        Up | Down => coordinates.x == respawn.x,
        Left | Right => coordinates.y == respawn.y
    }
}

fn move_near_center<G: GhostType + Component + 'static>(components: &mut SpawnedTargetComponentsItem, ghost_house: &GhostHouse) {
    let coordinates = components.transform.translation;
    let center = ghost_house.center_coordinates();
    let respawn = ghost_house.respawn_coordinates_of::<G>();

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
fn coordinates_slightly_in_direction(v: Vec3, d: Direction) -> Vec3 {
    let distance = FIELD_DIMENSION / 2.0;
    match d {
        Up => Vec3::new(v.x, v.y + distance, 0.0),
        Down => Vec3::new(v.x, v.y - distance, 0.0),
        Left => Vec3::new(v.x - distance, v.y, 0.0),
        Right => Vec3::new(v.x + distance, v.y, 0.0),
    }
}