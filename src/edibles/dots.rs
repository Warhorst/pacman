use bevy::prelude::*;

use crate::constants::DOT_DIMENSION;
use crate::edibles::Edible;
use crate::life_cycle::LifeCycle::*;
use crate::is;
use crate::map::{Element, Map};

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_dots)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(spawn_dots)
            )
        ;
    }
}

#[derive(Component)]
pub struct Dot;

fn spawn_dots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
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
            .insert(Dot)
            .insert(Edible)
        ;
    }
}