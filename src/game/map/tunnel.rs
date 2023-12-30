use bevy::prelude::*;
use crate::game::map::{Element, TileMap};
use crate::prelude::*;

pub struct TunnelPlugin;

impl Plugin for TunnelPlugin {
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

// TODO remove, use the ones from core::map instead
#[derive(Component, Deref)]
pub struct Tunnel(usize);

// TODO remove, use the ones from core::map instead
#[derive(Component)]
pub struct TunnelHallway;

/// Event. Fired when a ghost moved through a tunnel.
#[derive( Event, Deref, DerefMut)]
pub struct GhostPassedTunnel(pub Entity);

pub(crate) fn spawn_tunnels(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Vec<Entity> {
    tile_map.position_element_iter()
        .into_iter()
        .flat_map(|(pos, elem)| match elem {
            Element::Tunnel { index, opening_direction } => Some((*index, *pos, *opening_direction)),
            _ => None
        })
        .flat_map(|(index, position, direction)| spawn_tunnel(commands, index, position, direction))
        .collect()
}

fn spawn_tunnel(
    commands: &mut Commands,
    index: usize,
    position: Pos,
    direction: Dir,
) -> [Entity; 2] {
    let tunnel_transform = Transform::from_translation(position.to_vec3(TUNNEL_Z));
    let tunnel_entrance_transform = Transform::from_translation(position.neighbour_in_direction(direction.opposite()).to_vec3(TUNNEL_Z));

    let tunnel = commands.spawn((
        Name::new("Tunnel"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(TUNNEL_DIMENSION)),
                ..default()
            },
            transform: tunnel_transform,
            ..Default::default()
        },
        Tunnel(index),
        direction
    )).id();

    let tunnel_entrance = commands.spawn((
        Name::new("TunnelEntrance"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(TUNNEL_DIMENSION)),
                ..default()
            },
            transform: tunnel_entrance_transform,
            ..Default::default()
        }
    )).id();

    [tunnel, tunnel_entrance]
}

pub fn spawn_tunnel_hallways(
    commands: &mut Commands,
    tile_map: &TileMap,
) -> Vec<Entity> {
    tile_map.get_positions_matching(is!(Element::TunnelHallway))
        .into_iter()
        .map(|position| commands.spawn((
            Name::new("TunnelHallway"),
            TunnelHallway,
            SpatialBundle {
                transform: Transform::from_translation(position.to_vec3(TUNNEL_Z)),
                ..default()
            }
        )).id()
        )
        .collect()
}

fn move_pacman_through_tunnel(
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Dir), Without<Pacman>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Dir), Without<Pacman>>,
    mut pacman_query: Query<(&mut Transform, &mut Dir), With<Pacman>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (mut transform, mut pacman_direction) in pacman_query.iter_mut() {
            let entity_pos = Pos::from_vec3(transform.translation);
            let tunnel_pos = Pos::from_vec3(tunnel_transform_0.translation);

            if entity_pos != tunnel_pos || *pacman_direction != *tunnel_direction_0 {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform.translation.set_xy(&tunnel_transform_1.translation);
                    *pacman_direction = tunnel_direction_1.opposite()
                }
            }
        }
    }
}

fn move_ghost_trough_tunnel(
    mut event_writer: EventWriter<GhostPassedTunnel>,
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Dir), Without<Ghost>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Dir), Without<Ghost>>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Dir), With<Ghost>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (ghost_entity, mut transform, mut ghost_direction) in ghost_query.iter_mut() {
            let entity_pos = Pos::from_vec3(transform.translation);
            let tunnel_pos = Pos::from_vec3(tunnel_transform_0.translation);

            if entity_pos != tunnel_pos || *ghost_direction != *tunnel_direction_0 {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in tunnel_query_1.iter() {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform.translation.set_xy(&tunnel_transform_1.translation);
                    *ghost_direction = tunnel_direction_1.opposite();
                    event_writer.send(GhostPassedTunnel(ghost_entity));
                }
            }
        }
    }
}
