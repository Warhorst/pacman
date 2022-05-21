use bevy::prelude::*;

use crate::constants::POINT_DIMENSION;
use crate::common::Position;
use crate::is;
use crate::map::board::Board;
use crate::map::Element;
use crate::pacman::Pacman;

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DotEaten>()
            .add_event::<AllDotsEaten>()
            .add_startup_system(spawn_dots)
            .add_system(pacman_eat_dot)
            .add_system(send_event_when_all_dots_are_eaten)
        ;
    }
}

#[derive(Component)]
pub struct Dot;

/// Event. Fired when pacman eats a dot.
pub struct DotEaten;

/// Event. Fired when all dots are eaten.
pub struct AllDotsEaten;

fn spawn_dots(
    mut commands: Commands,
    board: Res<Board>
) {
    let point_dimension = Vec2::new(POINT_DIMENSION, POINT_DIMENSION);
    for position in board.get_positions_matching(is!(Element::DotSpawn)) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(point_dimension),
                    ..default()
                },
                transform: Transform::from_translation(Board::coordinates_of_position(position)),
                ..Default::default()
            })
            .insert(Dot)
            .insert(position.clone());
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<DotEaten>,
    pacman_positions: Query<&Position, With<Pacman>>,
    dot_positions: Query<(Entity, &Position), With<Dot>>,
) {
    for pacman_pos in pacman_positions.iter() {
        for (entity, dot_pos) in dot_positions.iter() {
            if pacman_pos == dot_pos {
                commands.entity(entity).despawn();
                event_writer.send(DotEaten)
            }
        }
    }
}

fn send_event_when_all_dots_are_eaten(
    mut event_writer: EventWriter<AllDotsEaten>,
    query: Query<&Dot>,
) {
    if query.iter().count() > 0 { return; }

    event_writer.send(AllDotsEaten);
}