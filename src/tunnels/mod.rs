use std::collections::HashMap;

use bevy::prelude::*;

use spawner::Spawner;

use crate::common::{Direction, Movement, Position};
use crate::common::Direction::*;
use crate::constants::FIELD_DIMENSION;
use crate::map::{FieldType, Neighbour};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::pacman::Pacman;

mod mover;
mod spawner;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_tunnels.system())
            .add_system(pacman_enters_tunnel.system());
    }
}

struct Tunnel;

#[derive(Copy, Clone)]
struct TunnelEntrance {
    position: Position,
    entrance_direction: Direction
}

fn spawn_tunnels(mut commands: Commands, board: Res<Board>) {
    Spawner::new(commands, &board).spawn()
}

fn pacman_enters_tunnel(board: Res<Board>,
                        tunnel_query: Query<With<Tunnel, (&TunnelEntrance, &TunnelEntrance)>>,
                        mut pacman_query: Query<With<Pacman, (&mut Transform, &Position, &mut Movement)>>) {
    for (mut transform, position, movement) in pacman_query.iter_mut() {
        for (first_entrance, second_entrance) in tunnel_query.iter() {
            match get_entrance_entity_passes(position, first_entrance, second_entrance) {
                None => return,
                Some(entrance) => unimplemented!()
            }
        }
    }
}

fn ghost_enters_tunnel() {}

/// Returns the entrance the current entity passes. Return None if it
///currently not passes any tunnel entrance.
fn get_entrance_entity_passes<'a>(entity_position: &Position,
                                  first_entrance: &'a TunnelEntrance,
                                  second_entrance: &'a TunnelEntrance) -> Option<&'a TunnelEntrance> {
    match entity_position {
        pos if pos == &first_entrance.position => Some(first_entrance),
        pos if pos == &second_entrance.position => Some(second_entrance),
        _ => None
    }
}