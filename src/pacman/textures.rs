use bevy::prelude::*;
use crate::animation::{Animation, Animations};

use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::map::Rotation;

pub(in crate::pacman) fn update_pacman_appearance(
    mut query: Query<(&Direction, &mut Transform), With<Pacman>>
) {
    for (direction, mut transform) in query.iter_mut() {
        match direction {
            Up => transform.rotation = Rotation::D90.quat_z(),
            Down => transform.rotation = Rotation::D270.quat_z(),
            Left => transform.rotation = Rotation::D0.quat_z(),
            Right => transform.rotation = Rotation::D180.quat_z(),
        }
    }
}

pub(in crate::pacman) fn create_pacman_animations(asset_server: &AssetServer) -> Animations {
    Animations::new(
        [
            ("eating", create_eating_animation(asset_server)),
            ("dying", create_dying_animation(asset_server))
        ],
        "eating",
    )
}

fn create_eating_animation(asset_server: &AssetServer) -> Animation {
    Animation::new(
        0.2,
        true,
        [
            asset_server.load("textures/pacman/pacman_closed.png"),
            asset_server.load("textures/pacman/pacman_opening.png"),
            asset_server.load("textures/pacman/pacman_open.png"),
            asset_server.load("textures/pacman/pacman_opening.png"),
        ],
    )
}

fn create_dying_animation(asset_server: &AssetServer) -> Animation {
    Animation::new(
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