use bevy::prelude::*;
use crate::animation::{Animation, Animations};

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::game_asset_handles::GameAssetHandles;
use crate::game_asset_handles::keys::*;
use crate::spritesheet::SpriteSheet;

pub(in crate::pacman) fn update_pacman_appearance(
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

pub(in crate::pacman) fn create_pacman_animations(
    game_assets: &GameAssetHandles
) -> Animations {
    Animations::new(
        [
            ("eating_left", create_eating_animation(game_assets.get_handle(PACMAN_WALKING_LEFT))),
            ("eating_right", create_eating_animation(game_assets.get_handle(PACMAN_WALKING_RIGHT))),
            ("eating_up", create_eating_animation(game_assets.get_handle(PACMAN_WALKING_UP))),
            ("eating_down", create_eating_animation(game_assets.get_handle(PACMAN_WALKING_DOWN))),
            ("dying", create_dying_animation(game_assets.get_handle(PACMAN_DYING)))
        ],
        "eating_up",
    )
}

fn create_eating_animation(
    sheet_handle: Handle<SpriteSheet>,
) -> Animation {
    Animation::from_sprite_sheet(
        0.2,
        true,
        4,
        sheet_handle
    )
}

fn create_dying_animation(
    sheet_handle: Handle<SpriteSheet>,
) -> Animation {
    Animation::from_sprite_sheet(
        2.0,
        false,
        12,
        sheet_handle,
    )
}