use bevy::prelude::*;
use bevy_sprite_sheet::SpriteSheets;

use crate::constants::{GHOST_BASE_SPEED, GHOST_DIMENSION};
use crate::game::direction::MovementDirection;
use crate::game::state::State;
use crate::game::target::Target;
use crate::game::ghosts::textures::create_animations_for_ghost;
use crate::game::level::Level;
use crate::game::map::ghost_house::GhostSpawn;
use crate::game::specs_per_level::SpecsPerLevel;
use crate::game::speed::Speed;
use pad::Direction::*;

pub fn spawn_ghosts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets: Res<SpriteSheets>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    spawn_query: Query<&GhostSpawn>,
) {
    for spawn in &spawn_query {
        spawn_ghost(&mut commands, spawn, &asset_server, &sprite_sheets, &level, &specs_per_level);
    }
}

fn spawn_ghost(
    commands: &mut Commands,
    spawn: &GhostSpawn,
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) {
    let spawn_direction = spawn.spawn_direction;
    let spawn_coordinates = spawn.coordinates;
    let mut animations = create_animations_for_ghost(&spawn.ghost, asset_server, sprite_sheets);
    animations.change_animation_to(match spawn.spawn_direction {
        YP => "normal_up",
        YM => "normal_down",
        XM => "normal_left",
        XP => "normal_right",
        _ => panic!("invalid direction")
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
            visibility: Visibility::Visible,
            ..Default::default()
        },
        spawn.ghost,
        MovementDirection(spawn_direction),
        Speed(GHOST_BASE_SPEED * specs_per_level.get_for(level).ghost_normal_speed_modifier),
        Target::new(),
        State::Spawned,
        animations
    ));
}