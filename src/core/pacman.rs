use bevy::prelude::*;

pub(super) struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Pacman>()
        ;
    }
}

/// Marker component for a pacman entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Pacman;