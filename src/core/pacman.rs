use bevy::prelude::*;
use crate::core::prelude::*;

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
#[require(
    Name::new("Pacman"),
    Dir::Up,
    Speed,
    Sprite,
    Animations
)]
pub struct Pacman;