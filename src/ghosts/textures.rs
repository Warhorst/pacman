use std::any::TypeId;
use bevy::prelude::*;
use crate::animation::{Animation, Animations};
use crate::common::Direction;
use crate::edibles::energizer::EnergizerTimer;
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::*;
use crate::ghosts::{Blinky, GhostType, Inky, Pinky};
use crate::ghosts::state::State;

pub(in crate::ghosts) fn update_ghost_appearance<G: 'static + Component + GhostType>(
    energizer_timer: Res<EnergizerTimer>,
    mut query: Query<(&Direction, &State, &mut Animations), With<G>>,
) {
    for (direction, state, mut animations) in query.iter_mut() {
        match state {
            State::Frightened => match energizer_timer.remaining() {
                // animate a frightened ghost differently if the energizer timer is almost ending
                Some(secs) if secs < 2.0 => animations.change_animation_to("frightened_blinking"),
                _ => animations.change_animation_to("frightened"),
            },
            State::Eaten => {
                animations.change_animation_to(format!("eaten_{}", direction.to_string()))
            }
            _ => animations.change_animation_to(format!("normal_{}", direction.to_string()))
        }
    }
}

pub(in crate::ghosts) fn create_animations_for_ghost<G: GhostType + 'static>(game_assets: &GameAssetHandles) -> Animations {
    match TypeId::of::<G>() {
        id if id == TypeId::of::<Blinky>() => create_animations_for(game_assets, [BLINKY_UP, BLINKY_DOWN, BLINKY_LEFT, BLINKY_RIGHT]),
        id if id == TypeId::of::<Pinky>() => create_animations_for(game_assets, [PINKY_UP, PINKY_DOWN, PINKY_LEFT, PINKY_RIGHT]),
        id if id == TypeId::of::<Inky>() => create_animations_for(game_assets, [INKY_UP, INKY_DOWN, INKY_LEFT, INKY_RIGHT]),
        _ => create_animations_for(game_assets, [CLYDE_UP, CLYDE_DOWN, CLYDE_LEFT, CLYDE_RIGHT]),
    }
}

fn create_animations_for(game_assets: &GameAssetHandles, normal_animation_keys: [&'static str; 4]) -> Animations {
    Animations::new(
        [
            ("normal_up", create_normal_animation(game_assets, normal_animation_keys[0])),
            ("normal_down", create_normal_animation(game_assets, normal_animation_keys[1])),
            ("normal_left", create_normal_animation(game_assets, normal_animation_keys[2])),
            ("normal_right", create_normal_animation(game_assets, normal_animation_keys[3])),
            ("eaten_up", create_eaten_animation(game_assets, EATEN_UP)),
            ("eaten_down", create_eaten_animation(game_assets, EATEN_DOWN)),
            ("eaten_left", create_eaten_animation(game_assets, EATEN_LEFT)),
            ("eaten_right", create_eaten_animation(game_assets, EATEN_RIGHT)),
            ("frightened", create_frightened_animation(game_assets)),
            ("frightened_blinking", create_frightened_blinking_animation(game_assets)),
        ],
        "normal_left")
}

fn create_normal_animation(game_assets: &GameAssetHandles, key: &'static str) -> Animation {
    Animation::from_sprite_sheet(
        0.5,
        true,
        2,
        game_assets.get_handle(key),
    )
}

fn create_eaten_animation(game_assets: &GameAssetHandles, key: &'static str) -> Animation {
    Animation::from_texture(game_assets.get_handle(key))
}

fn create_frightened_animation(game_assets: &GameAssetHandles) -> Animation {
    Animation::from_sprite_sheet(
        0.5,
        true,
        2,
        game_assets.get_handle(FRIGHTENED),
    )
}

fn create_frightened_blinking_animation(game_assets: &GameAssetHandles) -> Animation {
    Animation::from_sprite_sheet(
        0.5,
        true,
        4,
        game_assets.get_handle(FRIGHTENED_BLINKING),
    )
}