use bevy::prelude::*;

use crate::constants::{BLINKY_Z, CLYDE_Z, GHOST_DIMENSION, GHOST_SPEED, INKY_Z, PINKY_Z};
use crate::game_assets::handles::GameAssetHandles;
use crate::ghost_house::GhostHouse;
use crate::ghosts::{Blinky, Clyde, Ghost, GhostType, Inky, Pinky};
use crate::ghosts::state::State;
use crate::ghosts::target::Target;
use crate::ghosts::textures::create_animations_for_ghost;
use crate::level::Level;
use crate::specs_per_level::SpecsPerLevel;
use crate::speed::Speed;
use crate::sprite_sheet::SpriteSheet;

pub fn spawn_ghosts(
    mut commands: Commands,
    game_assets: Res<GameAssetHandles>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    ghost_house: Res<GhostHouse>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Blinky, BLINKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Pinky, PINKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Inky, INKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Clyde, CLYDE_Z);
}

fn spawn_ghost<G: GhostType + Component>(
    commands: &mut Commands,
    ghost_house: &GhostHouse,
    game_assets: &GameAssetHandles,
    sprite_sheets: &Assets<SpriteSheet>,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
    ghost_type: G,
    z_value: f32
) {
    let spawn_direction = ghost_house.spawn_direction_of::<G>();
    let mut spawn_coordinates = ghost_house.spawn_coordinates_of::<G>();
    spawn_coordinates.z = z_value;
    let mut animations = create_animations_for_ghost::<G>(game_assets, sprite_sheets);
    animations.stop();

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
        .insert(Speed(GHOST_SPEED * specs_per_level.get_for(level).ghost_normal_speed_modifier))
        .insert(Target::new())
        .insert(State::Spawned)
        .insert(animations)
    ;
}