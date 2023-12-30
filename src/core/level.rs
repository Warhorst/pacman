use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Level>()
            .add_plugins(ResourceInspectorPlugin::<Level>::default())
        ;
    }
}

/// The current level which defines the difficulty, the fruit to spawn and more.
#[derive(Resource, Reflect, Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Level(pub usize);

impl Level {
    pub fn increase(&mut self) {
        **self += 1
    }
}