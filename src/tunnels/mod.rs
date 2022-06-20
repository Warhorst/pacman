use bevy::prelude::*;

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

#[derive(Component, Deref)]
struct Tunnel(usize);

/// Event. Fired when a ghost moved through a tunnel.
#[derive(Deref, DerefMut)]
pub struct GhostPassedTunnel(pub Entity);