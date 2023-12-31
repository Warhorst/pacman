use bevy::prelude::*;
use crate::spawn::enhance_map::EnhanceMapPlugin;

use crate::spawn::spawn_map_scene::SpawnMapScenePlugin;

mod spawn_map_scene;
mod enhance_map;

pub(super) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                SpawnMapScenePlugin,
                EnhanceMapPlugin
            ))
        ;
    }
}

