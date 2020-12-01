use bevy::prelude::*;

use spawner::Spawner;

use crate::common::{Direction, Movement, Position};
use crate::ghosts::{Ghost, Target};
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
    second_entrance: TunnelEntrance
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

fn spawn_tunnels(commands: Commands, board: Res<Board>) {
    Spawner::new(commands, &board).spawn()
}

fn pacman_enters_tunnel(board: Res<Board>,
                        tunnel_query: Query<&Tunnel>,
                        mut pacman_query: Query<With<Pacman, (&mut Transform, &mut Position, &mut Movement)>>) {
    for (mut transform, mut position, mut movement) in pacman_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            Mover::new(&board, tunnel, &mut transform.translation, &mut position, &mut movement).move_pacman_through_tunnel()
        }
    }
}

fn ghost_enters_tunnel(board: Res<Board>,
                       tunnel_query: Query<&Tunnel>,
                       mut ghost_query: Query<With<Ghost, (&mut Transform, &mut Position, &mut Movement, &mut Target)>>) {
    for (mut transform, mut position, mut movement, mut target) in ghost_query.iter_mut() {
        for tunnel in tunnel_query.iter() {
            Mover::new(&board, tunnel, &mut transform.translation, &mut position, &mut movement).move_ghost_through_tunnel(&mut target)
        }
    }
}