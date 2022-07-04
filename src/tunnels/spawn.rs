use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::common::position::Position;
use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::map::Element;

use crate::map::board::Board;
use crate::tunnels::Tunnel;
use crate::common::Direction;

/// Resource that knows the position of everything that is considered a tunnel.
pub struct TunnelPositions(HashSet<Position>);

impl TunnelPositions {
    fn new<'a, I: IntoIterator<Item=&'a Position>>(iter: I) -> Self {
        TunnelPositions(iter.into_iter().map(|p| *p).collect())
    }

    pub fn contains(&self, pos: &Position) -> bool {
        self.0.contains(pos)
    }
}

pub(in crate::tunnels) fn spawn_tunnels(
    mut commands: Commands,
    board: Res<Board>,
) {
    let tunnel_positions = TunnelPositions::new(board.get_positions_matching(is!(Element::Tunnel {..} | Element::TunnelEntrance | Element::TunnelHallway)));
    commands.insert_resource(tunnel_positions);

    board.position_element_iter()
        .into_iter()
        .flat_map(|(pos, elem)| match elem {
            Element::Tunnel {index, opening_direction} => Some((*index, *pos, *opening_direction)),
            _ => None
        })
        .for_each(|(index, position, direction)| spawn_tunnel(&mut commands, index, position, direction));

    spawn_tunnel_entrances(&mut commands, &board);
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
fn spawn_tunnel_entrances(commands: &mut Commands, board: &Board) {
    board.get_positions_matching(is!(Element::TunnelEntrance))
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

// TODO: This is only a bad workaround, as the board always returns z = 0.0
fn get_transform(pos: &Position) -> Transform {
    let mut translation = Vec3::from(pos);
    translation.z = 10.0;
    Transform::from_translation(translation)
}