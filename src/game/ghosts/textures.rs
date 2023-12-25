use bevy::prelude::*;
use bevy_sprite_sheet::{SpriteSheet, SpriteSheets};
use crate::animation::{Animation, Animations};
use crate::game::direction::MovementDirection;
use crate::game::edibles::energizer::EnergizerTimer;
use crate::game::ghosts::Ghost;
use crate::game::ghosts::Ghost::*;
use crate::game::state::State;
use pad::Direction::*;

pub(crate) fn update_ghost_appearance(
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut query: Query<(&MovementDirection, &State, &mut Animations)>,
) {
    for (direction, state, mut animations) in query.iter_mut() {
        match state {
            State::Frightened => match energizer_timer {
                // animate a frightened ghost differently if the energizer timer is almost ending
                Some(ref timer) if timer.remaining() < 2.0 => animations.change_animation_to("frightened_blinking"),
                _ => animations.change_animation_to("frightened"),
            },
            State::Eaten => {
                animations.change_animation_to(match **direction {
                    XP => "eaten_right",
                    XM => "eaten_left",
                    YP => "eaten_up",
                    YM => "eaten_down",
                    _ => panic!("invalid direction")
                })
            }
            _ => animations.change_animation_to(match **direction {
                XP => "normal_right",
                XM => "normal_left",
                YP => "normal_up",
                YM => "normal_down",
                _ => panic!("invalid direction")
            })
        }
    }
}

pub(crate) fn create_animations_for_ghost(ghost: &Ghost, asset_server: &AssetServer, sprite_sheets: &SpriteSheets) -> Animations {
    match *ghost {
        Blinky => create_animations_for(asset_server, sprite_sheets, ["textures/ghost/blinky_up", "textures/ghost/blinky_down", "textures/ghost/blinky_left", "textures/ghost/blinky_right"]),
        Pinky => create_animations_for(asset_server, sprite_sheets, ["textures/ghost/pinky_up", "textures/ghost/pinky_down", "textures/ghost/pinky_left", "textures/ghost/pinky_right"]),
        Inky => create_animations_for(asset_server, sprite_sheets, ["textures/ghost/inky_up", "textures/ghost/inky_down", "textures/ghost/inky_left", "textures/ghost/inky_right"]),
        Clyde => create_animations_for(asset_server, sprite_sheets, ["textures/ghost/clyde_up", "textures/ghost/clyde_down", "textures/ghost/clyde_left", "textures/ghost/clyde_right"]),
    }
}

fn create_animations_for(
    asset_server: &AssetServer,
    sprite_sheets: &SpriteSheets,
    normal_animation_keys: [&'static str; 4]
) -> Animations {
    Animations::new(
        [
            ("normal_up", create_normal_animation(sprite_sheets.get_sheet(normal_animation_keys[0]))),
            ("normal_down", create_normal_animation(sprite_sheets.get_sheet(normal_animation_keys[1]))),
            ("normal_left", create_normal_animation(sprite_sheets.get_sheet(normal_animation_keys[2]))),
            ("normal_right", create_normal_animation(sprite_sheets.get_sheet(normal_animation_keys[3]))),
            ("eaten_up", create_eaten_animation(asset_server, "textures/ghost/eaten_up.png")),
            ("eaten_down", create_eaten_animation(asset_server, "textures/ghost/eaten_down.png")),
            ("eaten_left", create_eaten_animation(asset_server, "textures/ghost/eaten_left.png")),
            ("eaten_right", create_eaten_animation(asset_server, "textures/ghost/eaten_right.png")),
            ("frightened", create_frightened_animation(sprite_sheets.get_sheet("textures/ghost/frightened"))),
            ("frightened_blinking", create_frightened_blinking_animation(sprite_sheets.get_sheet("textures/ghost/frightened_blinking"))),
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

fn create_eaten_animation(asset_server: &AssetServer, key: &'static str) -> Animation {
    Animation::from_texture(asset_server.load(key))
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
pub(crate) fn start_animation(
    mut query: Query<&mut Animations, With<Ghost>>
) {
    for mut anim in &mut query {
        anim.resume()
    }
}