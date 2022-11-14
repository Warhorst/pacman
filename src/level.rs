use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use crate::life_cycle::LifeCycle::LevelTransition;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InspectorPlugin::<Level>::new())
            .insert_resource(Level(1))
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(increase_level)
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