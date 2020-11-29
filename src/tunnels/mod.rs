use std::collections::HashMap;

use bevy::prelude::*;

use crate::common::{Direction, Movement, Position};
use crate::common::Direction::*;
use crate::constants::FIELD_DIMENSION;
use crate::map::{FieldType, Neighbour};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::pacman::Pacman;

mod mover;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_tunnels.system());
    }
}

struct Tunnel;

#[derive(Copy, Clone)]
struct TunnelEntrance {
    position: Position,
    entrance_direction: Direction
}

fn spawn_tunnels(mut commands: Commands, board: Res<Board>) {
    create_tunnel_entrances(&board).into_iter()
        .map(|(_, entrances)| entrances)
        .for_each(|entrances|  {
            commands.spawn((Tunnel, entrances[0], entrances[1]));
        })
}

fn create_tunnel_entrances(board: &Board) -> HashMap<usize, Vec<TunnelEntrance>> {
    let mut index_with_entrance = HashMap::new();
    for tunnel_position in board.positions_of_type_filter(field_type_is_tunnel_entrance) {
        let tunnel_entrance_neighbours = board.neighbours_of(tunnel_position)
            .into_iter()
            .filter(neighbour_is_type_entrance)
            .collect::<Vec<_>>();

        let tunnel_entrance_neighbour = match tunnel_entrance_neighbours.len() {
            1 => tunnel_entrance_neighbours[0],
            0 => panic!("A tunnel should have one entrance as neighbour!"),
            _ => panic!("A tunnel should not have more than one entrance as neighbour!")
        };

        let tunnel_index = match board.type_of_position(tunnel_position) {
            TunnelEntrance(index) => *index,
            _ => panic!("The type of the tunnel position should be a tunnel.")
        };

        let entrance = TunnelEntrance {
            position: *tunnel_position,
            entrance_direction: tunnel_entrance_neighbour.direction.opposite()
        };

        match index_with_entrance.get_mut(&tunnel_index) {
            None => { index_with_entrance.insert(tunnel_index, vec![entrance]); },
            Some(entrances) if entrances.len() > 1 => panic!("There are more than 2 entrances for one tunnel!"),
            Some(entrances) => entrances.push(entrance)
        }
    }
    index_with_entrance
}

fn field_type_is_tunnel_entrance(field_type: &FieldType) -> bool {
    match field_type {
        TunnelEntrance(_) => true,
        _ => false
    }
}

fn neighbour_is_type_entrance(neighbour: &Neighbour) -> bool {
    match neighbour.field_type {
        TunnelDirection => true,
        _ => false
    }
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