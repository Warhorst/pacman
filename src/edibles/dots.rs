use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;

use crate::constants::DOT_Z;
use crate::edibles::Edible;
use crate::game_assets::loaded_assets::LoadedAssets;
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
    map: Res<Map>,
    dimensions: Res<BoardDimensions>,
    game_asset_handles: Res<LoadedAssets>
) {
    let point_dimension = Vec2::new(dimensions.dot(), dimensions.dot());
    for position in map.get_positions_matching(is!(Element::DotSpawn)) {
        let transform = dimensions.pos_to_trans(position, DOT_Z);

        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: game_asset_handles.get_handle("textures/dot.png"),
                sprite: Sprite {
                    custom_size: Some(point_dimension),
                    ..default()
                },
                transform,
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

    pub fn get_max(&self) -> usize {
        self.max
    }

    fn reset(&mut self) {
        self.eaten = 0
    }
}