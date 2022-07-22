use bevy::prelude::*;
use crate::animation::{Animation, Animations};

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::helper::load_textures;

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

pub(in crate::pacman) fn create_pacman_animations(asset_server: &AssetServer) -> Animations {
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

fn create_eating_animation(asset_server: &AssetServer, direction: Direction) -> Animation {
    let direction = direction.to_string();
    Animation::from_textures(
        0.2,
        true,
        load_textures(asset_server, &[
            "textures/pacman/pacman_closed.png".to_string(),
            format!("textures/pacman/pacman_opening_{direction}.png"),
            format!("textures/pacman/pacman_open_{direction}.png"),
            format!("textures/pacman/pacman_opening_{direction}.png")
        ]),
    )
}

fn create_dying_animation(asset_server: &AssetServer) -> Animation {
    Animation::from_textures(
        2.0,
        true,
        [
            asset_server.load("textures/pacman/pacman_dying_a.png"),
            asset_server.load("textures/pacman/pacman_dying_b.png"),
            asset_server.load("textures/pacman/pacman_dying_c.png"),
            asset_server.load("textures/pacman/pacman_dying_d.png"),
            asset_server.load("textures/pacman/pacman_dying_e.png"),
            asset_server.load("textures/pacman/pacman_dying_f.png"),
            asset_server.load("textures/pacman/pacman_dying_g.png"),
            asset_server.load("textures/pacman/pacman_dying_h.png"),
            asset_server.load("textures/pacman/pacman_dying_i.png"),
            asset_server.load("textures/pacman/pacman_dying_j.png"),
            asset_server.load("textures/pacman/pacman_dying_k.png"),
            asset_server.load("textures/pacman/pacman_dying_l.png"),
        ],
    )
}