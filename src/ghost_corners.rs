use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::is;
use crate::life_cycle::LifeCycle::Start;
use crate::map::{Element, Map};

pub struct GhostCornersPlugin;

impl Plugin for GhostCornersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_ghost_corners)
            )
        ;
    }
}

// TODO: As the amount of corners does not change during the game, a resource might fit better
#[derive(Component)]
pub struct GhostCorner;

fn spawn_ghost_corners(
    mut commands: Commands,
    map: Res<Map>,
    dimensions: Res<BoardDimensions>
) {
    spawn_corner(&mut commands, &map, is!(Element::BlinkyCorner), &dimensions, Blinky);
    spawn_corner(&mut commands, &map, is!(Element::PinkyCorner), &dimensions, Pinky);
    spawn_corner(&mut commands, &map, is!(Element::InkyCorner), &dimensions, Inky);
    spawn_corner(&mut commands, &map, is!(Element::ClydeCorner), &dimensions, Clyde);
}

fn spawn_corner<C: Component + Copy>(
    commands: &mut Commands,
    map: &Map,
    filter: impl Fn(&Element) -> bool,
    dimensions: &BoardDimensions,
    ghost: C
) {
    map.get_positions_matching(filter)
        .into_iter()
        .for_each(|pos| {
            commands.spawn()
                .insert(GhostCorner)
                .insert(dimensions.pos_to_trans(pos, 0.0))
                .insert(ghost);
        });
}