use std::collections::HashMap;

use bevy::prelude::*;
use crate::common::Position;
use crate::constants::FIELD_DIMENSION;

use crate::map::{FieldType, Neighbour};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::tunnels::Tunnel;
use crate::tunnels::TunnelEntrance;

pub (in crate::tunnels) fn spawn_tunnels(
    mut commands: Commands,
    board: Res<Board>
) {
    // TODO: This is only a bad workaround, as the board always returns z = 0.0
    let get_transform = |pos: Position| -> Transform {
        let mut translation = board.coordinates_of_position(&pos);
        translation.z = 1.0;
        Transform::from_translation(translation)
    };

    create_tunnel_entrances(&board).into_iter()
        .map(|(_, entrances)| entrances)
        .for_each(|entrances|  {
            commands.spawn().insert(Tunnel::new(entrances[0], entrances[1]));

            commands.spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(FIELD_DIMENSION, FIELD_DIMENSION)),
                        ..default()
                    },
                    transform: (get_transform)(entrances[0].position),
                    ..Default::default()
                });

            commands.spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(FIELD_DIMENSION, FIELD_DIMENSION)),
                        ..default()
                    },
                    transform: (get_transform)(entrances[1].position),
                    ..Default::default()
                });
        });

    for pos in board.positions_of_type(TunnelOpening) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(FIELD_DIMENSION, FIELD_DIMENSION)),
                    ..default()
                },
                transform: (get_transform)(*pos),
                ..Default::default()
            });
    }
}

fn create_tunnel_entrances(board: &Board) -> HashMap<usize, Vec<TunnelEntrance>> {
    let mut index_with_entrance = HashMap::new();
    for tunnel_entrance_position in board.positions_of_type_filter(field_type_is_tunnel_entrance) {
        let tunnel_entrance_neighbours = board.neighbours_of(tunnel_entrance_position)
            .into_iter()
            .filter(neighbour_type_is_opening)
            .collect::<Vec<_>>();

        let tunnel_entrance_neighbour = match tunnel_entrance_neighbours.len() {
            1 => tunnel_entrance_neighbours[0],
            0 => panic!("A tunnel should have one entrance as neighbour!"),
            _ => panic!("A tunnel should not have more than one entrance as neighbour!")
        };

        let tunnel_index = match board.type_of_position(tunnel_entrance_position) {
            TunnelEntrance(index) => *index,
            _ => panic!("The type of the tunnel position should be a tunnel.")
        };

        let entrance = TunnelEntrance {
            position: *tunnel_entrance_position,
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

fn neighbour_type_is_opening(neighbour: &Neighbour) -> bool {
    match neighbour.field_type {
        TunnelOpening => true,
        _ => false
    }
}