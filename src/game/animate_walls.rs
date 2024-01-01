use bevy::prelude::*;

use crate::core::prelude::*;

pub(super) struct AnimateWallsPlugin;

impl Plugin for AnimateWallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(LevelTransition)),
                set_animation_to_blinking
            )
            .add_systems(
                OnExit(Game(LevelTransition)),
                set_animation_to_idle
            )
        ;
    }
}

fn set_animation_to_blinking(
    mut query: Query<&mut Animations, With<Wall>>
) {
    for mut animations in &mut query {
        animations.change_animation_to("blinking")
    }
}

fn set_animation_to_idle(
    mut query: Query<&mut Animations, With<Wall>>
) {
    for mut animations in &mut query {
        animations.change_animation_to("idle")
    }
}