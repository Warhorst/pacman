mod enhance_maze;

use bevy::prelude::*;
use crate::core::prelude::*;
use crate::spawn::enhance_map::enhance_maze::EnhanceMazePlugin;

pub(super) struct EnhanceMapPlugin;

impl Plugin for EnhanceMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnhanceMazePlugin)
            .add_systems(
                OnEnter(Spawn(EnhanceMap)),
                add_spatial_bundle_to_map
            )
        ;
    }
}

fn add_spatial_bundle_to_map(
    mut commands: Commands,
    maps: Query<Entity, With<Map>>
) {
    commands.entity(maps.single()).insert(SpatialBundle::default());
}