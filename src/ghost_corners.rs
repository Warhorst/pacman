use bevy::prelude::*;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::is;
use crate::map::board::Board;
use crate::map::Element;

pub struct GhostCornersPlugin;

impl Plugin for GhostCornersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ghost_corners);
    }
}

// TODO: As the amount of corners does not change during the game, a resource might fit better
#[derive(Component)]
pub struct GhostCorner;

fn spawn_ghost_corners(
    mut commands: Commands,
    board: Res<Board>,
) {
    spawn_corner(&mut commands, &board, is!(Element::BlinkyCorner), Blinky);
    spawn_corner(&mut commands, &board, is!(Element::PinkyCorner), Pinky);
    spawn_corner(&mut commands, &board, is!(Element::InkyCorner), Inky);
    spawn_corner(&mut commands, &board, is!(Element::ClydeCorner), Clyde);
}

fn spawn_corner<C: Component + Copy>(
    commands: &mut Commands,
    board: &Board,
    filter: impl Fn(&Element) -> bool,
    ghost: C
) {
    board.get_positions_matching(filter)
        .into_iter()
        .for_each(|pos| {
            commands.spawn()
                .insert(GhostCorner)
                .insert(Transform::from_translation(Vec3::from(pos)))
                .insert(ghost);
        });
}