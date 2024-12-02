use bevy::prelude::*;
use bevy_sprite_sheet::SpriteSheets;
use crate::game::pacman::textures::create_pacman_animations;
use crate::core::prelude::*;

pub(crate) fn spawn_pacman(
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheets>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    spawns: Query<&Tiles, With<PacmanSpawn>>,
) {
    let tiles = spawns.single();
    let transform = Transform::from_translation(tiles.to_vec3(PACMAN_Z));
    let mut animations = create_pacman_animations(&sprite_sheets);
    animations.stop();

    commands.spawn((
        Pacman,
        Speed(PACMAN_BASE_SPEED * specs_per_level.get_for(&level).pacman_normal_speed_modifier),
        Sprite {
            image: animations.current().texture(),
            custom_size: Some(Vec2::splat(PACMAN_DIMENSION)),
            ..default()
        },
        transform,
        animations
    ));
}