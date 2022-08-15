use bevy::prelude::*;
use crate::common::Direction::*;

use crate::constants::{PACMAN_DIMENSION, PACMAN_Z};
use crate::is;
use crate::level::Level;
use crate::map::Element::PacManSpawn;
use crate::map::Map;
use crate::pacman::Pacman;
use crate::pacman::textures::create_pacman_animations;
use crate::game_assets::handles::GameAssetHandles;
use crate::speed::SpeedByLevel;
use crate::sprite_sheet::SpriteSheet;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    game_assets: Res<GameAssetHandles>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let mut spawn_coordinates = map.coordinates_between_positions_matching(is!(PacManSpawn));
    spawn_coordinates.z = PACMAN_Z;
    let dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    let mut animations = create_pacman_animations(&game_assets, &sprite_sheets);
    animations.stop();

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(dimension),
                ..default()
            },
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(speed_by_level.for_pacman(&level).normal)
        .insert(Up)
        .insert(animations)
    ;
}