use std::collections::HashMap;

use bevy::prelude::*;

use crate::map::{FieldType, Neighbour};
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::tunnels::Tunnel;
use crate::tunnels::TunnelEntrance;

pub(in crate::tunnels) struct Spawner<'a> {
    commands: Commands<'a, 'a>,
    board: &'a Board
}

impl<'a> Spawner<'a> {
    pub fn new(commands: Commands<'a, 'a>, board: &'a Board) -> Self {
        Spawner { commands, board }
    }

    /// Get all tunnel entrances from the board and spawn them.
    pub fn spawn(&mut self) {
        self.create_tunnel_entrances().into_iter()
            .map(|(_, entrances)| entrances)
            .for_each(|entrances|  {
                self.commands.spawn().insert(Tunnel::new(entrances[0], entrances[1]));
            })
    }

    fn create_tunnel_entrances(&self) -> HashMap<usize, Vec<TunnelEntrance>> {
        let mut index_with_entrance = HashMap::new();
        for tunnel_entrance_position in self.board.positions_of_type_filter(Self::field_type_is_tunnel_entrance) {
            let tunnel_entrance_neighbours = self.board.neighbours_of(tunnel_entrance_position)
                .into_iter()
                .filter(Self::neighbour_is_type_entrance)
                .collect::<Vec<_>>();

            let tunnel_entrance_neighbour = match tunnel_entrance_neighbours.len() {
                1 => tunnel_entrance_neighbours[0],
                0 => panic!("A tunnel should have one entrance as neighbour!"),
                _ => panic!("A tunnel should not have more than one entrance as neighbour!")
            };

            let tunnel_index = match self.board.type_of_position(tunnel_entrance_position) {
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

    fn neighbour_is_type_entrance(neighbour: &Neighbour) -> bool {
        match neighbour.field_type {
            TunnelDirection => true,
            _ => false
        }
    }
}