use bevy::prelude::*;
use crate::common::Direction::*;

use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::level::Level;
use crate::map::Element::PacManSpawn;
use crate::map::Map;
use crate::pacman::Pacman;
use crate::pacman::textures::{PacmanTextures, Phase};
use crate::speed::SpeedByLevel;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

pub fn spawn_pacman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let pacman_spawn = PacmanSpawn(map.coordinates_between_positions_matching(is!(PacManSpawn)));
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    let textures = PacmanTextures::new(&asset_server);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: textures.get_texture_for_phase(Phase::Closed),
            sprite: Sprite {
                custom_size: Some(pacman_dimension),
                ..default()
            },
            transform: Transform::from_translation(*pacman_spawn),
            ..Default::default()
        })
        .insert(Pacman)
        .insert(speed_by_level.for_pacman(&level).normal)
        .insert(Left)
    ;
    commands.insert_resource(pacman_spawn);
    commands.insert_resource(textures);
}