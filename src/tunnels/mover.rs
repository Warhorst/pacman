use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::common::Movement::*;
use crate::map::board::Board;
use crate::tunnels::{Tunnel, TunnelEntrance};

/// Moves pacman or a ghost to another tunnel entrance.
pub(in crate::tunnels) struct Mover<'a> {
    board: &'a Board,
    tunnel: &'a Tunnel,
    entity_translation: &'a mut  Vec3,
    entity_position: &'a mut Position,
    entity_movement: &'a mut Movement,
}

impl<'a> Mover<'a> {
    pub fn new(board: &'a Board,
               tunnel: &'a Tunnel,
               entity_translation: &'a mut Vec3,
               entity_position: &'a mut Position,
               entity_movement: &'a mut Movement) -> Self {
        Mover { board, tunnel, entity_translation, entity_position, entity_movement }
    }

    /// Move an entity through a tunnel if it moves into it.
    /// Returns false if the entity does not move into the tunnel
    pub fn move_entity_through_tunnel(&mut self) -> bool {
        let end = match self.get_outgoing_tunnel() {
            None => return false,
            Some(path) => path
        };
        self.teleport_entity_to_tunnel_end(end);
        true
    }

    /// If the spectated entity currently moves into a tunnel,
    /// return its end. If not, return None.
    fn get_outgoing_tunnel(&self) -> Option<TunnelEntrance> {
        let path = match *self.entity_position {
            pos if pos == self.tunnel.first_entrance.position => TunnelPath::new(self.tunnel.first_entrance, self.tunnel.second_entrance),
            pos if pos == self.tunnel.second_entrance.position => TunnelPath::new(self.tunnel.second_entrance, self.tunnel.first_entrance),
            _ => return None
        };
        match &self.entity_movement {
            Moving(dir) if dir == &path.start.entrance_direction => Some(path.end),
            _ => None
        }
    }

    fn teleport_entity_to_tunnel_end(&mut self, end: TunnelEntrance) {
        *self.entity_position = end.position.clone();
        *self.entity_translation = self.board.coordinates_of_position(&end.position);
        *self.entity_movement = Moving(end.entrance_direction.opposite());
    }
}

/// Describes a path through a tunnel an entity takes.
struct TunnelPath {
    start: TunnelEntrance,
    end: TunnelEntrance,
}

impl TunnelPath {
    pub fn new(start: TunnelEntrance, end: TunnelEntrance) -> Self {
        TunnelPath { start, end }
    }
}