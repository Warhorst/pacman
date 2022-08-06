use bevy::prelude::*;
use crate::animation::{Animation, Animations};

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;

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
) -> Animations {
    Animations::new(
        [
            ("eating_left", create_eating_animation(asset_server, Left)),
            ("eating_right", create_eating_animation(asset_server, Right)),
            ("eating_up", create_eating_animation(asset_server, Up)),
            ("eating_down", create_eating_animation(asset_server, Down)),
            ("dying", create_dying_animation(asset_server))
        ],
        "eating_up",
    )
}

fn create_eating_animation(
    asset_server: &AssetServer,
    direction: Direction,
) -> Animation {
    let direction = direction.to_string();
    Animation::from_sprite_sheet(
        0.2,
        true,
        4,
        asset_server.load(&format!("textures/pacman/pacman_walking_{direction}.sheet.png"))
    )
}

fn create_dying_animation(
    asset_server: &AssetServer,
) -> Animation {
    Animation::from_sprite_sheet(
        2.0,
        false,
        12,
        asset_server.load("textures/pacman/pacman_dying.sheet.png"),
    )
}