use std::any::TypeId;
use bevy::prelude::*;
use crate::animation::{Animation, Animations};
use crate::common::Direction;
use crate::edibles::energizer::EnergizerTimer;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::ghosts::{Blinky, Ghost, GhostType, Inky, Pinky};
use crate::ghosts::state::State;
use crate::sprite_sheet::SpriteSheet;

pub(in crate::ghosts) fn update_ghost_appearance<G: 'static + Component + GhostType>(
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut query: Query<(&Direction, &State, &mut Animations), With<G>>,
) {
    for (direction, state, mut animations) in query.iter_mut() {
        match state {
            State::Frightened => match energizer_timer {
                // animate a frightened ghost differently if the energizer timer is almost ending
                Some(ref timer) if timer.remaining() < 2.0 => animations.change_animation_to("frightened_blinking"),
                _ => animations.change_animation_to("frightened"),
            },
            State::Eaten => {
                animations.change_animation_to(format!("eaten_{}", direction.to_string()))
            }
            _ => animations.change_animation_to(format!("normal_{}", direction.to_string()))
        }
    }
}

pub(in crate::ghosts) fn create_animations_for_ghost<G: GhostType + 'static>(game_assets: &LoadedAssets, sprite_sheets: &Assets<SpriteSheet>) -> Animations {
    match TypeId::of::<G>() {
        id if id == TypeId::of::<Blinky>() => create_animations_for(game_assets, sprite_sheets, ["textures/ghost/blinky_up", "textures/ghost/blinky_down", "textures/ghost/blinky_left", "textures/ghost/blinky_right"]),
        id if id == TypeId::of::<Pinky>() => create_animations_for(game_assets, sprite_sheets, ["textures/ghost/pinky_up", "textures/ghost/pinky_down", "textures/ghost/pinky_left", "textures/ghost/pinky_right"]),
        id if id == TypeId::of::<Inky>() => create_animations_for(game_assets, sprite_sheets, ["textures/ghost/inky_up", "textures/ghost/inky_down", "textures/ghost/inky_left", "textures/ghost/inky_right"]),
        _ => create_animations_for(game_assets, sprite_sheets, ["textures/ghost/clyde_up", "textures/ghost/clyde_down", "textures/ghost/clyde_left", "textures/ghost/clyde_right"]),
    }
}

fn create_animations_for(game_assets: &LoadedAssets, sprite_sheets: &Assets<SpriteSheet>, normal_animation_keys: [&'static str; 4]) -> Animations {
    Animations::new(
        [
            ("normal_up", create_normal_animation(game_assets.get_asset(normal_animation_keys[0], sprite_sheets))),
            ("normal_down", create_normal_animation(game_assets.get_asset(normal_animation_keys[1], sprite_sheets))),
            ("normal_left", create_normal_animation(game_assets.get_asset(normal_animation_keys[2], sprite_sheets))),
            ("normal_right", create_normal_animation(game_assets.get_asset(normal_animation_keys[3], sprite_sheets))),
            ("eaten_up", create_eaten_animation(game_assets, "textures/ghost/eaten_up.png")),
            ("eaten_down", create_eaten_animation(game_assets, "textures/ghost/eaten_down.png")),
            ("eaten_left", create_eaten_animation(game_assets, "textures/ghost/eaten_left.png")),
            ("eaten_right", create_eaten_animation(game_assets, "textures/ghost/eaten_right.png")),
            ("frightened", create_frightened_animation(game_assets.get_asset("textures/ghost/frightened", sprite_sheets))),
            ("frightened_blinking", create_frightened_blinking_animation(game_assets.get_asset("textures/ghost/frightened_blinking", sprite_sheets))),
        ],
        "normal_left")
}

fn create_normal_animation(sprite_sheet: &SpriteSheet) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        sprite_sheet.images_at(0..2),
    )
}

fn create_eaten_animation(game_assets: &LoadedAssets, key: &'static str) -> Animation {
    Animation::from_texture(game_assets.get_handle(key))
}

fn create_frightened_animation(sprite_sheet: &SpriteSheet) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        sprite_sheet.images_at(0..2)
    )
}

fn create_frightened_blinking_animation(sprite_sheet: &SpriteSheet) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        sprite_sheet.images_at(0..4)
    )
}

/// The ghosts start with stopped animations. Restart them here
pub(in crate::ghosts) fn start_animation(
    mut query: Query<&mut Animations, With<Ghost>>
) {
    for mut anim in &mut query {
        anim.resume()
    }
}