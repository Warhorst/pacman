use bevy::prelude::*;
use crate::prelude::*;

pub(in crate::game) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level(1))
            .add_systems(
                OnExit(Game(LevelTransition)),
                increase_level
            )
            .add_systems(
                OnExit(Game(GameOver)),
                reset_level
            )
        ;
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