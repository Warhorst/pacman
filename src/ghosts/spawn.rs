use bevy::prelude::*;

use crate::constants::GHOST_DIMENSION;
use crate::game_assets::handles::GameAssetHandles;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::ghosts::state::State;
use crate::ghosts::target::Target;
use crate::ghosts::textures::create_animations_for_ghost;
use crate::level::Level;
use crate::map::Map;
use crate::speed::SpeedByLevel;
use crate::sprite_sheet::SpriteSheet;

pub fn spawn_ghosts(
    mut commands: Commands,
    game_assets: Res<GameAssetHandles>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    map: Res<Map>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>,
) {
    let ghost_house = GhostHouse::new(&map);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &speed_by_level, Blinky);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &speed_by_level, Pinky);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &speed_by_level, Inky);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &speed_by_level, Clyde);
    commands.insert_resource(ghost_house);
}

fn spawn_ghost<G: GhostType + Component>(
    commands: &mut Commands,
    ghost_house: &GhostHouse,
    game_assets: &GameAssetHandles,
    sprite_sheets: &Assets<SpriteSheet>,
    level: &Level,
    speed_by_level: &SpeedByLevel,
    ghost_type: G,
) {
    let spawn_direction = ghost_house.spawn_direction_of::<G>();
    let spawn_coordinates = ghost_house.spawn_coordinates_of::<G>();
    let animations = create_animations_for_ghost::<G>(game_assets, sprite_sheets);

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
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
        .insert(animations)
    ;
}