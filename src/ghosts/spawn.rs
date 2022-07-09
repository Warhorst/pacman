use bevy::prelude::*;

use crate::constants::GHOST_DIMENSION;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::ghosts::state::State;
use crate::ghosts::target::Target;
use crate::ghosts::textures::GhostTextures;
use crate::level::Level;
use crate::map::Map;
use crate::speed::SpeedByLevel;

pub fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let ghost_house = GhostHouse::new(&map);
    let ghost_textures = GhostTextures::new(&asset_server);
    spawn_ghost(&mut commands, &ghost_house, &ghost_textures, &level, &speed_by_level, Blinky);
    spawn_ghost(&mut commands, &ghost_house, &ghost_textures, &level, &speed_by_level, Pinky);
    spawn_ghost(&mut commands, &ghost_house, &ghost_textures, &level, &speed_by_level, Inky);
    spawn_ghost(&mut commands, &ghost_house, &ghost_textures, &level, &speed_by_level, Clyde);
    commands.insert_resource(ghost_house);
    commands.insert_resource(ghost_textures);
}

fn spawn_ghost<G: GhostType + Component>(
    commands: &mut Commands,
    ghost_house: &GhostHouse,
    ghost_textures: &GhostTextures,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    ghost_type: G,

) {
    let spawn_direction = ghost_house.spawn_direction_of::<G>();
    let spawn_coordinates = ghost_house.spawn_coordinates_of::<G>();
    let texture = ghost_textures.get_normal_texture_for::<G>(&spawn_direction);

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        })
        .insert(Ghost)
        .insert(ghost_type)
        .insert(spawn_direction)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(Target::new())
        .insert(State::Spawned)
    ;
}