use bevy::prelude::*;
use crate::game_assets::animation::{Animation, Animations};

use crate::game::pacman::Pacman;
use crate::game::direction::Direction;
use crate::game::direction::Direction::*;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game_assets::sprite_sheet::SpriteSheet;

pub(crate) fn update_pacman_appearance(
    mut query: Query<(&Direction, &mut Animations), With<Pacman>>
) {
    for (direction, mut animations) in query.iter_mut() {
        match direction {
            Up => animations.change_animation_to("eating_up"),
            Down => animations.change_animation_to("eating_down"),
            Left => animations.change_animation_to("eating_left"),
            Right => animations.change_animation_to("eating_right"),
        }
    }
}

pub(crate) fn create_pacman_animations(
    game_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>
) -> Animations {
    Animations::new(
        [
            ("eating_left", create_eating_animation(game_assets.get_asset("textures/pacman/pacman_walking_left", sprite_sheets))),
            ("eating_right", create_eating_animation(game_assets.get_asset("textures/pacman/pacman_walking_right", sprite_sheets))),
            ("eating_up", create_eating_animation(game_assets.get_asset("textures/pacman/pacman_walking_up", sprite_sheets))),
            ("eating_down", create_eating_animation(game_assets.get_asset("textures/pacman/pacman_walking_down", sprite_sheets))),
            ("dying", create_dying_animation(game_assets.get_asset("textures/pacman/pacman_dying", sprite_sheets)))
        ],
        "eating_up",
    )
}

fn create_eating_animation(
    sheet: &SpriteSheet,
) -> Animation {
    Animation::from_textures(
        0.2,
        true,
        sheet.images_at(0..4)
    )
}

fn create_dying_animation(
    sheet: &SpriteSheet,
) -> Animation {
    Animation::from_textures(
        1.5,
        false,
        sheet.images_at(0..12),
    )
}

/// pacman starts with stopped animations. Restart them here
pub(crate) fn start_animation(
    mut query: Query<&mut Animations, With<Pacman>>
) {
    for mut anim in &mut query {
        anim.resume()
    }
}