use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;

pub(in crate::game) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Level>()
            .add_plugins(ResourceInspectorPlugin::<Level>::default())
            .insert_resource(Level(1))
            .add_systems(OnExit(Game(LevelTransition)), increase_level)
            .add_systems(OnExit(Game(GameOver)), reset_level)
        ;
    }
}

#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Hash, Reflect, Default, Resource)]
pub struct Level(pub usize);

impl Level {
    fn increase(&mut self) {
        **self += 1
    }
}

fn increase_level(
    mut level: ResMut<Level>,
) {
    level.increase();
}

fn reset_level(
    mut level: ResMut<Level>
) {
    level.0 = 1;
}