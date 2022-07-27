use bevy::prelude::*;
use crate::animation::{Animation, Animations};

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::spritesheet::SpriteSheets;

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
    asset_server: &AssetServer,
    sprite_sheets: &mut SpriteSheets
) -> Animations {
    Animations::new(
        [
            ("eating_left", create_eating_animation(asset_server, sprite_sheets, Left)),
            ("eating_right", create_eating_animation(asset_server, sprite_sheets, Right)),
            ("eating_up", create_eating_animation(asset_server, sprite_sheets, Up)),
            ("eating_down", create_eating_animation(asset_server, sprite_sheets, Down)),
            ("dying", create_dying_animation(asset_server, sprite_sheets))
        ],
        "eating_up",
    )
}

fn create_eating_animation(
    asset_server: &AssetServer,
    sprite_sheets: &mut SpriteSheets,
    direction: Direction
) -> Animation {
    let direction = direction.to_string();
    Animation::from_sprite_sheet(
        0.2,
        true,
        sprite_sheets.add_sheet(
            asset_server.load(&format!("textures/pacman/pacman_walking_{direction}.png")),
            Vec2::new(16.0, 16.0),
            4,
            1
        )
    )
}

fn create_dying_animation(
    asset_server: &AssetServer,
    sprite_sheets: &mut SpriteSheets,
) -> Animation {
    Animation::from_sprite_sheet(
        2.0,
        false,
        sprite_sheets.add_sheet(
            asset_server.load("textures/pacman/pacman_dying.png"),
            Vec2::new(16.0, 16.0),
            12,
            1
        )
    )
}