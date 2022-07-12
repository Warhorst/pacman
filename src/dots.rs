use bevy::prelude::*;

use crate::constants::DOT_DIMENSION;
use crate::common::position::ToPosition;
use crate::is;
use crate::map::{Element, Map};
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
    asset_server: Res<AssetServer>,
    map: Res<Map>
) {
    let point_dimension = Vec2::new(DOT_DIMENSION, DOT_DIMENSION);
    for position in map.get_positions_matching(is!(Element::DotSpawn)) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/dot.png"),
                sprite: Sprite {
                    custom_size: Some(point_dimension),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::from(position)),
                ..Default::default()
            })
            .insert(Dot);
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<DotEaten>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for pacman_tf in pacman_positions.iter() {
        for (entity, dot_tf) in dot_positions.iter() {
            if pacman_tf.pos() == dot_tf.pos() {
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