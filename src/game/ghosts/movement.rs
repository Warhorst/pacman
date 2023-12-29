use bevy::prelude::*;
use crate::prelude::*;
use crate::game::state::GhostState;

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                move_ghosts
                    .in_set(MoveEntities)
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                Update,
                move_only_not_currently_eaten_ghosts
                    .in_set(MoveEntities)
                    .run_if(in_state(Game(GhostEatenPause)))
            )
        ;
    }
}

fn move_ghosts(
    time: Res<Time>,
    mut query: Query<(&Dir, &mut Target, &mut Transform, &Speed)>,
) {
    for (direction, mut target, mut transform, speed) in query.iter_mut() {
        move_ghost(&time, direction, &mut target, &mut transform, speed)
    }
}

fn move_only_not_currently_eaten_ghosts(
    time: Res<Time>,
    currently_eaten_ghost: Res<CurrentlyEatenGhost>,
    mut query: Query<(Entity, &Dir, &GhostState, &mut Target, &mut Transform, &Speed)>,
) {
    for (entity, direction, state, mut target, mut transform, speed) in query.iter_mut() {
        if entity == **currently_eaten_ghost || *state != Eaten { continue; }
        move_ghost(&time, direction, &mut target, &mut transform, speed)
    }
}

fn move_ghost(time: &Time, direction: &Dir, target: &mut Target, transform: &mut Transform, speed: &Speed) {
    if target.is_not_set() {
        return;
    }

    let mut coordinates = &mut transform.translation;
    let delta_seconds = time.delta_seconds();
    let target_coordinates = target.get();
    move_in_direction(&mut coordinates, delta_seconds, &direction, speed);
    limit_movement(&mut coordinates, &direction, &target_coordinates);

    if on_target(*coordinates, target_coordinates, direction) {
        // Fix slight errors which might cause ghost to get stuck
        coordinates.x = target_coordinates.x;
        coordinates.y = target_coordinates.y;
        target.clear();
    }
}

fn get_direction_modifiers(direction: &Dir) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0),
    }
}

fn move_in_direction(coordinates: &mut Vec3, delta_seconds: f32, direction: &Dir, speed: &Speed) {
    let (x, y) = get_direction_modifiers(direction);
    coordinates.x += delta_seconds * x * **speed;
    coordinates.y += delta_seconds * y * **speed;
}

/// The ghost should not move over its target.
fn limit_movement(coordinates: &mut Vec3, direction: &Dir, target_coordinates: &Vec3) {
    match direction {
        Up => coordinates.y = coordinates.y.min(target_coordinates.y),
        Down => coordinates.y = coordinates.y.max(target_coordinates.y),
        Left => coordinates.x = coordinates.x.max(target_coordinates.x),
        Right => coordinates.x = coordinates.x.min(target_coordinates.x),
    }
}

fn on_target(coordinates: Vec3, target: Vec3, direction: &Dir) -> bool {
    match direction {
        Up | Down => coordinates.y == target.y,
        Left | Right => coordinates.x == target.x,
    }
}