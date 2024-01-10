use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub(super) struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Lives>()
        ;

        #[cfg(debug_assertions)]
        app
            .add_plugins(ResourceInspectorPlugin::<Lives>::default())
        ;
    }
}

/// Resource that tells how many lives pacman currently has.
#[derive(Deref, DerefMut, Reflect, Default, Resource)]
pub struct Lives(pub usize);

/// Keeps track how many points the player needs to get a new life for pacman.
#[derive(Deref, DerefMut, Resource)]
pub struct PointsRequiredForExtraLife(usize);

impl PointsRequiredForExtraLife {
    pub fn new() -> Self {
        PointsRequiredForExtraLife(10000)
    }

    pub fn increase_limit(&mut self) {
        **self += 10000
    }
}