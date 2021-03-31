use bevy::prelude::*;

use spawner::Spawner;

use crate::common::{Direction, MoveComponents, Movement, Position};
use crate::events::GhostPassedTunnel;
use crate::ghosts::components::Ghost;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::pacman::Pacman;
use crate::tunnels::mover::Mover;

mod mover;
mod spawner;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_tunnels.system())
            .add_system(pacman_enters_tunnel.system())
            .add_system(ghost_enters_tunnel.system());
    }
}

struct Tunnel {
    first_entrance: TunnelEntrance,
    second_entrance: TunnelEntrance,
}

impl Tunnel {
    pub fn new(first_entrance: TunnelEntrance, second_entrance: TunnelEntrance) -> Self {
        Tunnel { first_entrance, second_entrance }
    }
}

#[derive(Copy, Clone, Debug)]
struct TunnelEntrance {
    position: Position,
    entrance_direction: Direction,
}

fn spawn_tunnels(commands: &mut Commands, board: Res<Board>) {
    Spawner::new(commands, &board).spawn()
}

fn pacman_enters_tunnel(board: Res<Board>,
                        tunnel_query: Query<&Tunnel>,
                        mut pacman_query: Query<MoveComponents, With<Pacman>>) {
    for (mut transform, mut position, mut movement) in pacman_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            Mover::new(&board, tunnel, &mut transform.translation, &mut position, &mut movement).move_entity_through_tunnel();
        }
    }
}

fn ghost_enters_tunnel(board: Res<Board>,
                       mut ghost_passes_tunnel_events: ResMut<Events<GhostPassedTunnel>>,
                       tunnel_query: Query<&Tunnel>,
                       mut ghost_query: Query<(Entity, &mut Transform, &mut Position, &mut Movement), With<Ghost>>) {
    for (entity, mut transform, mut position, mut movement) in ghost_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            if Mover::new(&board, tunnel, &mut transform.translation, &mut position, &mut movement).move_entity_through_tunnel() {
                ghost_passes_tunnel_events.send(GhostPassedTunnel { entity })
            }
        }
    }
}