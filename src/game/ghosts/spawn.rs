use bevy::prelude::*;

use crate::constants::{GHOST_BASE_SPEED, GHOST_DIMENSION};
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::state::State;
use crate::game::target::Target;
use crate::game::ghosts::textures::create_animations_for_ghost;
use crate::game::level::Level;
use crate::game::map::ghost_house::GhostSpawn;
use crate::game::specs_per_level::SpecsPerLevel;
use crate::game::speed::Speed;
use crate::game_assets::sprite_sheet::SpriteSheet;
use crate::game::direction::Direction::*;

pub fn spawn_ghosts(
    mut commands: Commands,
    game_assets: Res<LoadedAssets>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    spawn_query: Query<&GhostSpawn>,
) {
    for spawn in &spawn_query {
        spawn_ghost(&mut commands, spawn, &game_assets, &sprite_sheets, &level, &specs_per_level);
    }
}

fn spawn_ghost(
    commands: &mut Commands,
    spawn: &GhostSpawn,
    game_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) {
    let spawn_direction = spawn.spawn_direction;
    let spawn_coordinates = spawn.coordinates;
    let mut animations = create_animations_for_ghost(&spawn.ghost, game_assets, sprite_sheets);
    animations.change_animation_to(match spawn.spawn_direction {
        Up => "normal_up",
        Down => "normal_down",
        Left => "normal_left",
        Right => "normal_right",
    });
    animations.stop();

    commands.spawn((
        Name::new("Ghost"),
        SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(GHOST_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(spawn_coordinates),
            ..Default::default()
        },
        spawn.ghost,
        spawn_direction,
        Speed(GHOST_BASE_SPEED * specs_per_level.get_for(level).ghost_normal_speed_modifier),
        Target::new(),
        State::Spawned,
        animations
    ));
}