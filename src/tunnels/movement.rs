use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
use crate::ghosts::Ghost;
use crate::pacman::Pacman;
use crate::tunnels::{GhostPassedTunnel, Tunnel, TunnelEntrance};

pub(in crate::tunnels) fn move_pacman_through_tunnel(
    tunnel_query: Query<&Tunnel>,
    mut pacman_query: Query<(&mut Transform, &mut Position, &mut MoveDirection), With<Pacman>>,
) {
    for (mut transform, mut position, mut direction) in pacman_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            let end = match get_outgoing_tunnel(&tunnel, &position, &direction) {
                None => continue,
                Some(e) => e
            };

            *position = end.position;
            transform.translation = Vec3::from(&end.position);
            *direction = end.entrance_direction.opposite();
        }
    }
}

pub(in crate::tunnels) fn move_ghost_trough_tunnel(
    mut event_writer: EventWriter<GhostPassedTunnel>,
    tunnel_query: Query<&Tunnel>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Position, &mut MoveDirection), With<Ghost>>,
) {
    for (entity, mut transform, mut position, mut direction) in ghost_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            let end = match get_outgoing_tunnel(&tunnel, &position, &direction) {
                None => continue,
                Some(e) => e
            };

            *position = end.position;
            transform.translation = Vec3::from(&end.position);
            *direction = end.entrance_direction.opposite();
            event_writer.send(GhostPassedTunnel(entity));
        }
    }
}

/// If the spectated entity currently moves into a tunnel,
/// return its end. If not, return None.
fn get_outgoing_tunnel(tunnel: &Tunnel, position: &Position, direction: &MoveDirection) -> Option<TunnelEntrance> {
    let path = match position {
        pos if pos == &tunnel.first_entrance.position => (tunnel.first_entrance, tunnel.second_entrance),
        pos if pos == &tunnel.second_entrance.position => (tunnel.second_entrance, tunnel.first_entrance),
        _ => return None
    };
    match direction == &path.0.entrance_direction {
        true => Some(path.1),
        false => None
    }
}