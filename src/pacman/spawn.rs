use bevy::prelude::*;
use crate::common::Direction::*;

use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::level::Level;
use crate::map::Element::PacManSpawn;
use crate::map::Map;
use crate::pacman::Pacman;
use crate::pacman::textures::create_pacman_animations;
use crate::speed::SpeedByLevel;
use crate::spritesheet::SpriteSheets;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut image_assets: ResMut<Assets<Image>>,
    mut sprite_sheets: ResMut<SpriteSheets>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let pacman_spawn = PacmanSpawn(map.coordinates_between_positions_matching(is!(PacManSpawn)));
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    let animations = create_pacman_animations(&asset_server, &mut image_assets, &mut sprite_sheets);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(pacman_dimension),
                ..default()
            },
            transform: Transform::from_translation(*pacman_spawn),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(speed_by_level.for_pacman(&level).normal)
        .insert(Up)
        .insert(animations)
    ;
    commands.insert_resource(pacman_spawn);
}