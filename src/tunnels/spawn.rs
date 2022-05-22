use std::collections::HashMap;

use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::common::Position;
use crate::constants::FIELD_DIMENSION;
use crate::is;
use crate::map::Element;

use crate::map::board::Board;
use crate::tunnels::Tunnel;
use crate::tunnels::TunnelEntrance;

/// Resource that knows the position of everything that is considered a tunnel.
pub struct TunnelPositions(HashSet<Position>);

impl TunnelPositions {
    fn new<'a, I: IntoIterator<Item=&'a Position>>(iter: I) -> Self {
        TunnelPositions(iter.into_iter().map(|p| *p).collect())
    }

    pub fn contains(&self, pos: &Position) -> bool {
        self.0.contains(pos)
    }
}

pub(in crate::tunnels) fn spawn_tunnels(
    mut commands: Commands,
    board: Res<Board>,
) {
    let tunnel_positions = TunnelPositions::new(board.get_positions_matching(is!(Element::Tunnel {..} | Element::TunnelEntrance | Element::TunnelHallway)));
    commands.insert_resource(tunnel_positions);

    // TODO: This is only a bad workaround, as the board always returns z = 0.0
    let get_transform = |pos: Position| -> Transform {
        let mut translation = Vec3::from(&pos);
        translation.z = 1.0;
        Transform::from_translation(translation)
    };

    create_tunnel_entrances(&board).into_iter()
        .map(|(_, entrances)| entrances)
        .for_each(|entrances| {
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

    for pos in board.get_positions_matching(is!(Element::Tunnel {..})) {
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
    for tunnel_entrance_position in board.get_positions_matching(is!(Element::Tunnel {..})) {
        let tunnel_entrance_neighbours = tunnel_entrance_position.get_neighbours()
            .into_iter()
            .filter(|n| board.position_matches_filter(&n.position, is!(Element::TunnelEntrance)))
            .collect::<Vec<_>>();

        let tunnel_entrance_neighbour = match tunnel_entrance_neighbours.len() {
            1 => tunnel_entrance_neighbours.get(0).unwrap(),
            0 => panic!("A tunnel should have one entrance as neighbour!"),
            _ => panic!("A tunnel should not have more than one entrance as neighbour!")
        };

        let tunnel_index = match board.element_on_position_matching(tunnel_entrance_position, is!(Element::Tunnel {..})) {
            Some(Element::Tunnel { index, .. }) => *index,
            _ => panic!("The type of the tunnel position should be a tunnel.")
        };

        let entrance = TunnelEntrance {
            position: *tunnel_entrance_position,
            entrance_direction: tunnel_entrance_neighbour.direction.opposite(),
        };

        match index_with_entrance.get_mut(&tunnel_index) {
            None => { index_with_entrance.insert(tunnel_index, vec![entrance]); }
            Some(entrances) if entrances.len() > 1 => panic!("There are more than 2 entrances for one tunnel!"),
            Some(entrances) => entrances.push(entrance)
        }
    }
    index_with_entrance
}