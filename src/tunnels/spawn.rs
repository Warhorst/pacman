use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::common::Position;
use crate::constants::FIELD_DIMENSION;
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

    board.get_positions_matching(is!(Element::Tunnel {..}))
        .into_iter()
        .map(|pos| get_index_position_direction_for_tunnel(&board, pos))
        .for_each(|(index, position, direction)| spawn_tunnel(&mut commands, index, position, direction));
}

fn spawn_tunnel(commands: &mut Commands, index: usize, position: Position, direction: Direction) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(FIELD_DIMENSION, FIELD_DIMENSION)),
                ..default()
            },
            transform: get_transform(&position),
            ..Default::default()
        })
        .insert(Tunnel(index))
        .insert(position)
        .insert(direction);
}

fn get_index_position_direction_for_tunnel(board: &Board, pos: &Position) -> (usize, Position, Direction) {
    match board.element_on_position_matching(pos, is!(Element::Tunnel {..})) {
        Some(Element::Tunnel { index, opening_direction }) => (*index, pos.clone(), *opening_direction),
        _ => panic!("Unreachable")
    }
}

// TODO: This is only a bad workaround, as the board always returns z = 0.0
fn get_transform(pos: &Position) -> Transform {
    let mut translation = Vec3::from(pos);
    translation.z = 10.0;
    Transform::from_translation(translation)
}