use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::tunnels::movement::{move_ghost_trough_tunnel, move_pacman_through_tunnel};
use crate::tunnels::spawn::spawn_tunnels;

mod movement;
pub mod spawn;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GhostPassedTunnel>()
            .add_startup_system(spawn_tunnels)
            .add_system(move_pacman_through_tunnel)
            .add_system(move_ghost_trough_tunnel);
    }
}

#[derive(Copy, Clone, Component, Debug)]
struct Tunnel {
    first_entrance: TunnelEntrance,
    second_entrance: TunnelEntrance,
}

impl Tunnel {
    pub fn new(first_entrance: TunnelEntrance, second_entrance: TunnelEntrance) -> Self {
        Tunnel { first_entrance, second_entrance }
    }
}

/// Event. Fired when a ghost moved through a tunnel.
#[derive(Deref, DerefMut)]
pub struct GhostPassedTunnel(pub Entity);

#[derive(Copy, Clone, Debug)]
struct TunnelEntrance {
    position: Position,
    entrance_direction: MoveDirection,
}