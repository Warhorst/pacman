use bevy::prelude::*;
use bevy_sprite_sheet::{SpriteSheet, SpriteSheets};
use crate::animation::{Animation, Animations};
use crate::game::direction::MovementDirection;

use crate::game::pacman::Pacman;

use pad::Direction::*;

pub(crate) fn update_pacman_appearance(
    mut query: Query<(&MovementDirection, &mut Animations), With<Pacman>>
) {
    for (direction, mut animations) in query.iter_mut() {
        match **direction {
            YP => animations.change_animation_to("eating_up"),
            YM => animations.change_animation_to("eating_down"),
            XM => animations.change_animation_to("eating_left"),
            XP => animations.change_animation_to("eating_right"),
            _ => {}
        }
    }
}

pub(crate) fn create_pacman_animations(
    sprite_sheets: &SpriteSheets
) -> Animations {
    Animations::new(
        [
            ("eating_left", create_eating_animation(sprite_sheets.get_sheet("textures/pacman/pacman_walking_left"))),
            ("eating_right", create_eating_animation(sprite_sheets.get_sheet("textures/pacman/pacman_walking_right"))),
            ("eating_up", create_eating_animation(sprite_sheets.get_sheet("textures/pacman/pacman_walking_up"))),
            ("eating_down", create_eating_animation(sprite_sheets.get_sheet("textures/pacman/pacman_walking_down"))),
            ("dying", create_dying_animation(sprite_sheets.get_sheet("textures/pacman/pacman_dying")))
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