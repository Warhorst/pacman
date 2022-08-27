use bevy::prelude::*;

use crate::constants::{DOT_DIMENSION, DOT_Z};
use crate::edibles::Edible;
use crate::life_cycle::LifeCycle::*;
use crate::is;
use crate::map::{Element, Map};

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start)
                    .with_system(spawn_dots)
                    .with_system(spawn_eaten_dots)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition)
                    .with_system(spawn_dots)
                    .with_system(reset_eaten_dots)
            )
        ;
    }
}

fn spawn_dots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    let point_dimension = Vec2::new(DOT_DIMENSION, DOT_DIMENSION);
    for position in map.get_positions_matching(is!(Element::DotSpawn)) {
        let mut coordinates = Vec3::from(position);
        coordinates.z = DOT_Z;

        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/dot.png"),
                sprite: Sprite {
                    custom_size: Some(point_dimension),
                    ..default()
                },
                transform: Transform::from_translation(coordinates),
                ..Default::default()
            })
            .insert(Dot)
            .insert(Edible)
        ;
    }
}

fn spawn_eaten_dots(
    mut commands: Commands,
    map: Res<Map>
) {
    let num_dots = map.get_positions_matching(is!(Element::DotSpawn)).into_iter().count();
    commands.insert_resource(EatenDots::new(num_dots))
}

fn reset_eaten_dots(
    mut eaten_dots: ResMut<EatenDots>
) {
    eaten_dots.reset()
}

#[derive(Component)]
pub struct Dot;

pub struct EatenDots {
    max: usize,
    eaten: usize
}

impl EatenDots {
    fn new(num_dots: usize) -> Self {
        EatenDots {
            max: num_dots,
            eaten: 0
        }
    }

    pub fn increment(&mut self) {
        self.eaten += 1
    }

    pub fn get_eaten(&self) -> usize {
        self.eaten
    }

    pub fn get_remaining(&self) -> usize {
        self.max - self.eaten
    }

    fn reset(&mut self) {
        self.eaten = 0
    }
}