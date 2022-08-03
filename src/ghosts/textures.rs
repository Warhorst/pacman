use std::any::TypeId;
use bevy::prelude::*;
use crate::animation::{Animation, Animations};
use crate::common::Direction;
use crate::edibles::energizer::EnergizerTimer;
use crate::ghosts::{Blinky, GhostType, Inky, Pinky};
use crate::ghosts::state::State;

pub (in crate::ghosts) fn update_ghost_appearance<G: 'static + Component + GhostType>(
    energizer_timer: Res<EnergizerTimer>,
    mut query: Query<(&Direction, &State, &mut Animations), With<G>>
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
            },
            _ => animations.change_animation_to(format!("normal_{}", direction.to_string()))
        }
    }
}

pub (in crate::ghosts) fn create_animations_for_ghost<G: GhostType + 'static>(asset_server: &AssetServer) -> Animations {
    match TypeId::of::<G>() {
        id if id == TypeId::of::<Blinky>() => create_animations_for(asset_server, "blinky"),
        id if id == TypeId::of::<Pinky>() => create_animations_for(asset_server, "pinky"),
        id if id == TypeId::of::<Inky>() => create_animations_for(asset_server, "inky"),
        _ => create_animations_for(asset_server, "clyde"),
    }
}

fn create_animations_for(asset_server: &AssetServer, ghost_name: &'static str) -> Animations {
    Animations::new(
        [
            ("normal_up", create_normal_animation(asset_server, ghost_name, "up")),
            ("normal_down", create_normal_animation(asset_server, ghost_name, "down")),
            ("normal_left", create_normal_animation(asset_server, ghost_name, "left")),
            ("normal_right", create_normal_animation(asset_server, ghost_name, "right")),
            ("eaten_up", create_eaten_animation(asset_server, "up")),
            ("eaten_down", create_eaten_animation(asset_server, "down")),
            ("eaten_left", create_eaten_animation(asset_server, "left")),
            ("eaten_right", create_eaten_animation(asset_server, "right")),
            ("frightened", create_frightened_animation(asset_server)),
            ("frightened_blinking", create_frightened_blinking_animation(asset_server)),
        ],
        "normal_left")
}

fn create_normal_animation(asset_server: &AssetServer, ghost_name: &'static str, direction: &'static str) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        [
            asset_server.load(&format!("textures/ghost/{ghost_name}_{direction}_a.png")),
            asset_server.load(&format!("textures/ghost/{ghost_name}_{direction}_b.png")),
        ]
    )
}

fn create_eaten_animation(asset_server: &AssetServer, direction: &'static str) -> Animation {
    Animation::from_texture(asset_server.load(&format!("textures/ghost/eaten_{direction}.png")))
}

fn create_frightened_animation(asset_server: &AssetServer) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        [
            asset_server.load("textures/ghost/frightened_a.png"),
            asset_server.load("textures/ghost/frightened_b.png"),
        ]
    )
}

fn create_frightened_blinking_animation(asset_server: &AssetServer) -> Animation {
    Animation::from_textures(
        0.5,
        true,
        [
            asset_server.load("textures/ghost/frightened_a.png"),
            asset_server.load("textures/ghost/frightened_blinking_a.png"),
            asset_server.load("textures/ghost/frightened_b.png"),
            asset_server.load("textures/ghost/frightened_blinking_b.png"),
        ]
    )
}