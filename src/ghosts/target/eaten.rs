use bevy::prelude::*;
use crate::ghost_house::GhostHouse;
use crate::ghosts::GhostType;
use crate::ghosts::target::{get_nearest_neighbour, TargetComponents, TargetComponentsItem};
use crate::walls::WallPositions;
use crate::common::Direction::*;
use crate::ghosts::state::State;
use crate::{state_skip_if, target_skip_if};
use crate::common::position::Vec3Helper;
use crate::common::position::ToPosition;

/// Determine the next target coordinates for a ghost when in "Eaten" state.
///
/// When eaten, a ghost walks to the ghost house and enters it. When at the ghost house, he aligns perfectly
/// before the entrance, moves than to the house center and finally to his spawn coordinates, which depend on the ghost type.
pub fn set_eaten_target<G: Component + GhostType + 'static>(
    ghost_house: Res<GhostHouse>,
    wall_positions: Res<WallPositions>,
    mut query: Query<TargetComponents, With<G>>,
) {
    for mut components in query.iter_mut() {
        target_skip_if!(components.target set);
        state_skip_if!(components.state != State::Eaten);

        if is_directly_before_entrance(&components, &ghost_house) {
            move_in_house_center(&mut components, &ghost_house)
        } else if is_before_entrance(&components, &ghost_house) {
            move_directly_before_entrance(&mut components, &ghost_house)
        } else if is_in_center(&components, &ghost_house) {
            move_to_respawn::<G>(&mut components, &ghost_house)
        } else {
            // TODO: Maybe only take this branch when not already in the ghost house, just to avoid bugs
            move_to_nearest_position_before_entrance(&mut components, &ghost_house, &wall_positions)
        }
    }
}

/// Return if the ghost is perfectly centered in front of the ghost house entrance.
fn is_directly_before_entrance(components: &TargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    components.transform.translation == ghost_house.coordinates_in_front_of_entrance()
}

fn move_in_house_center(components: &mut TargetComponentsItem, ghost_house: &GhostHouse) {
    *components.direction = ghost_house.entrance_direction.opposite();
    components.target.set(ghost_house.center_coordinates());
}

/// Return if the ghost is just on a position in front of the house.
fn is_before_entrance(components: &TargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    ghost_house.positions_in_front_of_entrance().into_iter().any(|pos| pos == &components.transform.translation.pos())
}

fn move_directly_before_entrance(components: &mut TargetComponentsItem, ghost_house: &GhostHouse) {
    let in_front_of_house = ghost_house.coordinates_in_front_of_entrance();
    let position_coordinates = components.transform.translation.pos_center();

    *components.direction = match ghost_house.entrance_direction {
        Up | Down => match in_front_of_house.x < position_coordinates.x {
            true => Left,
            false => Right
        },
        Left | Right => match in_front_of_house.y < position_coordinates.y {
            true => Down,
            false => Up
        }
    };
    components.target.set(in_front_of_house);
}

fn is_in_center(components: &TargetComponentsItem, ghost_house: &GhostHouse) -> bool {
    components.transform.translation == ghost_house.center_coordinates()
}

fn move_to_respawn<G: Component + GhostType + 'static>(components: &mut TargetComponentsItem, ghost_house: &GhostHouse) {
    let center = ghost_house.center_coordinates();
    let respawn = ghost_house.respawn_coordinates_of::<G>();

    *components.direction = match ghost_house.entrance_direction {
        Up | Down => match respawn.x < center.x {
            true => Left,
            false => Right
        },
        Left | Right => match respawn.y < center.y {
            true => Down,
            false => Up
        }
    };
    components.target.set(respawn);
}

fn move_to_nearest_position_before_entrance(components: &mut TargetComponentsItem, ghost_house: &GhostHouse, wall_positions: &WallPositions) {
    let nearest_spawn_position = components.transform.translation.pos().get_nearest_position_from(ghost_house.positions_in_front_of_entrance());
    let next_target_neighbour = get_nearest_neighbour(
        components,
        nearest_spawn_position,
        |n| !wall_positions.position_is_wall(&n.position)
    );

    *components.direction = next_target_neighbour.direction;
    components.target.set(Vec3::from(&next_target_neighbour.position));
}