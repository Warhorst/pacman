use bevy::prelude::*;

use crate::common::Position;
use crate::common::Direction::*;
use crate::constants::GHOST_DIMENSION;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, Inky, Pinky};
use crate::ghosts::state::State;
use crate::ghosts::target::Target;
use crate::ghosts::textures::GhostTextures;
use crate::level::Level;
use crate::map::board::Board;
use crate::speed::SpeedByLevel;

pub fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>
) {
    let ghost_house = GhostHouse::new(&board);
    let ghost_textures = GhostTextures::new(&asset_server);
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Blinky>(), &level, &speed_by_level, Blinky, ghost_textures.get_normal_texture_for::<Blinky>(&Left));
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Pinky>(), &level, &speed_by_level, Pinky, ghost_textures.get_normal_texture_for::<Pinky>(&Left));
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Inky>(), &level, &speed_by_level, Inky, ghost_textures.get_normal_texture_for::<Inky>(&Left));
    spawn_ghost(&mut commands, ghost_house.spawn_coordinates_of::<Clyde>(), &level, &speed_by_level, Clyde, ghost_textures.get_normal_texture_for::<Clyde>(&Left));
    commands.insert_resource(ghost_house);
    commands.insert_resource(ghost_textures);
}

fn spawn_ghost(
    commands: &mut Commands,
    spawn_coordinates: Vec3,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    ghost_type: impl Component,
    texture: Handle<Image>

) {
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
        .insert(Position::from(&spawn_coordinates))
        .insert(Left)
        .insert(speed_by_level.for_ghosts(level).normal)
        .insert(Target::new())
        .insert(State::Spawned)
    ;
}