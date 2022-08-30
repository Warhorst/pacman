use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;
use crate::common::Direction::*;

use crate::constants::PACMAN_Z;
use crate::is;
use crate::level::Level;
use crate::map::Element::PacManSpawn;
use crate::map::Map;
use crate::pacman::Pacman;
use crate::pacman::textures::create_pacman_animations;
use crate::game_assets::handles::GameAssetHandles;
use crate::specs_per_level::SpecsPerLevel;
use crate::speed::Speed;
use crate::sprite_sheet::SpriteSheet;

/// Resource that tells at which position pacman spawns.
#[derive(Deref, DerefMut)]
pub struct PacmanSpawn(Vec3);

pub (in crate::pacman) fn spawn_pacman(
    mut commands: Commands,
    game_assets: Res<GameAssetHandles>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    map: Res<Map>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    dimensions: Res<BoardDimensions>
) {
    let transform = dimensions.positions_to_trans(map.get_positions_matching(is!(PacManSpawn)), PACMAN_Z);
    let dimension = Vec2::new(dimensions.pacman(), dimensions.pacman());

    let mut animations = create_pacman_animations(&game_assets, &sprite_sheets);
    animations.stop();

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(dimension),
                ..default()
            },
            transform,
            ..Default::default()
        })
        .insert(Pacman)
        .insert(Speed(dimensions.pacman_base_speed() * specs_per_level.get_for(&level).pacman_normal_speed_modifier))
        .insert(Up)
        .insert(animations)
    ;
}