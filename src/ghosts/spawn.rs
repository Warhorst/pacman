use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;

use crate::constants::{BLINKY_Z, CLYDE_Z, INKY_Z, PINKY_Z};
use crate::game_assets::loaded_assets::LoadedAssets;
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
    game_assets: Res<LoadedAssets>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    ghost_house: Res<GhostHouse>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    dimensions: Res<BoardDimensions>
) {
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Blinky, &dimensions, BLINKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Pinky, &dimensions, PINKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Inky, &dimensions, INKY_Z);
    spawn_ghost(&mut commands, &ghost_house, &game_assets, &sprite_sheets, &level, &specs_per_level, Clyde, &dimensions, CLYDE_Z);
}

fn spawn_ghost<G: GhostType + Component>(
    commands: &mut Commands,
    ghost_house: &GhostHouse,
    game_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
    ghost_type: G,
    dimensions: &BoardDimensions,
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
                custom_size: Some(Vec2::new(dimensions.ghost(), dimensions.ghost())),
                ..default()
            },
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        })
        .insert(Ghost)
        .insert(ghost_type)
        .insert(spawn_direction)
        .insert(Speed(dimensions.ghost_base_speed() * specs_per_level.get_for(level).ghost_normal_speed_modifier))
        .insert(Target::new())
        .insert(State::Spawned)
        .insert(animations)
    ;
}