use bevy::prelude::*;

use crate::common::Direction;
use crate::ghosts::Ghost;
use crate::pacman::Pacman;
use crate::tunnels::{GhostPassedTunnel, Tunnel};
use crate::common::TransformHelper;

pub (in crate::tunnels) fn move_pacman_through_tunnel(
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Pacman>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Pacman>>,
    mut pacman_query: Query<(&mut Transform, &mut Direction), With<Pacman>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (mut transform, mut pacman_direction) in pacman_query.iter_mut() {
            if transform.pos() != tunnel_transform_0.pos() || *pacman_direction != *tunnel_direction_0 {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform.set_xy(tunnel_transform_1);
                    *pacman_direction = tunnel_direction_1.opposite()
                }
            }
        }
    }
}

pub(in crate::tunnels) fn move_ghost_trough_tunnel(
    mut event_writer: EventWriter<GhostPassedTunnel>,
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Ghost>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Ghost>>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Direction), With<Ghost>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (ghost_entity, mut transform, mut ghost_direction) in ghost_query.iter_mut() {
            if transform.pos() != tunnel_transform_0.pos() || *ghost_direction != *tunnel_direction_0 {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform.set_xy(tunnel_transform_1);
                    *ghost_direction = tunnel_direction_1.opposite();
                    event_writer.send(GhostPassedTunnel(ghost_entity));
                }
            }
        }
    }
}