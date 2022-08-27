use bevy::prelude::*;
use crate::common::position::Position;
use crate::constants::{PACMAN_DIMENSION, TUNNEL_Z};
use crate::is;
use crate::map::{Element, Map};

use crate::tunnels::Tunnel;
use crate::common::Direction;

pub(in crate::tunnels) fn spawn_tunnels(
    mut commands: Commands,
    map: Res<Map>,
) {
    map.position_element_iter()
        .into_iter()
        .flat_map(|(pos, elem)| match elem {
            Element::Tunnel {index, opening_direction} => Some((*index, *pos, *opening_direction)),
            _ => None
        })
        .for_each(|(index, position, direction)| spawn_tunnel(&mut commands, index, position, direction));

    spawn_tunnel_entrances(&mut commands, &map);
}

/// Spawn a tunnel with an index, position direction and a black sprite covering it.
///
/// Note: Currently, I don't care if there are more or less than two tunnels with the same index, as this
/// results from bad map design. This might get caught in the future when validating maps.
fn spawn_tunnel(commands: &mut Commands, index: usize, position: Position, direction: Direction) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION)),
                ..default()
            },
            transform: get_transform(&position),
            ..Default::default()
        })
        .insert(Tunnel(index))
        .insert(direction);
}

/// Spawn at every tunnel entrance a black square to cover pacman and ghosts. This looks like
/// these entities disappear and reappear at the corresponding tunnels.
fn spawn_tunnel_entrances(commands: &mut Commands, map: &Map) {
    map.get_positions_matching(is!(Element::TunnelEntrance))
        .into_iter()
        .for_each(|pos| spawn_tunnel_entrance(commands, pos));
}

fn spawn_tunnel_entrance(commands: &mut Commands, pos: &Position) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION)),
                ..default()
            },
            transform: get_transform(&pos),
            ..Default::default()
        });
}

fn get_transform(pos: &Position) -> Transform {
    let mut translation = Vec3::from(pos);
    translation.z = TUNNEL_Z;
    Transform::from_translation(translation)
}