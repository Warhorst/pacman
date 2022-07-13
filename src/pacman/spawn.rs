use bevy::prelude::*;
use crate::animation::Animation;
use crate::common::Direction::*;

use crate::constants::PACMAN_DIMENSION;
use crate::is;
use crate::level::Level;
use crate::map::Element::PacManSpawn;
use crate::map::Map;
use crate::pacman::Pacman;
use crate::speed::SpeedByLevel;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let pacman_spawn = PacmanSpawn(map.coordinates_between_positions_matching(is!(PacManSpawn)));
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/pacman/pacman_closed.png"),
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
        .insert(create_pacman_animation(&asset_server))
    ;
    commands.insert_resource(pacman_spawn);
}

fn create_pacman_animation(asset_server: &AssetServer) -> Animation {
    Animation::new(
        0.2,
        true,
        vec![
            asset_server.load("textures/pacman/pacman_closed.png"),
            asset_server.load("textures/pacman/pacman_opening.png"),
            asset_server.load("textures/pacman/pacman_open.png"),
            asset_server.load("textures/pacman/pacman_opening.png"),
        ]
    )
}