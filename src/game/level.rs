use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use crate::game_state::GameState::{GameOver, LevelTransition};

pub (in crate::game) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InspectorPlugin::<Level>::new())
            .insert_resource(Level(1))
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(increase_level)
            )
            .add_system_set(
                SystemSet::on_exit(GameOver).with_system(reset_level)
            )
        ;
    }
}

#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Hash, Inspectable, Default, Resource)]
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