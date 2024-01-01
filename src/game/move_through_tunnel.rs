use bevy::prelude::*;

use crate::core::prelude::*;

pub(super) struct MoveThroughTunnelPlugin;

impl Plugin for MoveThroughTunnelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GhostPassedTunnel>()
            .add_systems(Update, (
                move_pacman_through_tunnel,
                move_ghost_trough_tunnel
            ).run_if(in_state(Game(Running))))
        ;
    }
}

/// Event. Fired when a ghost moved through a tunnel.
#[derive( Event, Deref, DerefMut)]
pub struct GhostPassedTunnel(pub Entity);

fn move_pacman_through_tunnel(
    tunnel_query_0: Query<(Entity, &Tunnel, &Tiles), Without<Pacman>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Tiles), Without<Pacman>>,
    mut pacman_query: Query<(&mut Transform, &mut Dir), With<Pacman>>,
) {
    for (entity_0, tunnel_0, tunnel_tiles_0) in tunnel_query_0.iter() {
        for (mut transform, mut pacman_direction) in pacman_query.iter_mut() {
            let entity_pos = Pos::from_vec3(transform.translation);
            let tunnel_pos = tunnel_tiles_0.to_pos();

            if entity_pos != tunnel_pos || *pacman_direction != tunnel_0.direction {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_tiles_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && tunnel_0.index == tunnel_1.index {
                    transform.translation.set_xy(&tunnel_tiles_1.to_vec3(0.0));
                    *pacman_direction = tunnel_1.direction.opposite()
                }
            }
        }
    }
}

fn move_ghost_trough_tunnel(
    mut event_writer: EventWriter<GhostPassedTunnel>,
    tunnel_query_0: Query<(Entity, &Tunnel, &Tiles), Without<Ghost>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Tiles), Without<Ghost>>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Dir), With<Ghost>>,
) {
    for (entity_0, tunnel_0, tunnel_tiles_0) in tunnel_query_0.iter() {
        for (ghost_entity, mut transform, mut ghost_direction) in ghost_query.iter_mut() {
            let entity_pos = Pos::from_vec3(transform.translation);
            let tunnel_pos = tunnel_tiles_0.to_pos();

            if entity_pos != tunnel_pos || *ghost_direction != tunnel_0.direction {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_tiles_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && tunnel_0.index == tunnel_1.index {
                    transform.translation.set_xy(&tunnel_tiles_1.to_vec3(0.0));
                    *ghost_direction = tunnel_1.direction.opposite();
                    event_writer.send(GhostPassedTunnel(ghost_entity));
                }
            }
        }
    }
}