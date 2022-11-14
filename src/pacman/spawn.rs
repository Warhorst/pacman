use bevy::prelude::*;
use crate::common::Direction::*;

use crate::constants::{PACMAN_BASE_SPEED, PACMAN_DIMENSION};
use crate::level::Level;
use crate::map::PacmanSpawn;
use crate::pacman::Pacman;
use crate::pacman::textures::create_pacman_animations;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::specs_per_level::SpecsPerLevel;
use crate::speed::Speed;
use crate::sprite_sheet::SpriteSheet;

pub(in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    game_assets: Res<LoadedAssets>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    spawn_query: Query<&PacmanSpawn>,
) {
    let spawn = spawn_query.single();
    let transform = Transform::from_translation(**spawn);
    let mut animations = create_pacman_animations(&game_assets, &sprite_sheets);
    animations.stop();

    commands.spawn((
        SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(PACMAN_DIMENSION)),
                ..default()
            },
            transform,
            ..Default::default()
        },
        Name::new("Pacman"),
        Pacman,
        Speed(PACMAN_BASE_SPEED * specs_per_level.get_for(&level).pacman_normal_speed_modifier),
        Up,
        animations
    ));
}