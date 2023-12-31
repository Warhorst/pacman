mod enhance_maze;
mod enhance_ghost_house;

use bevy::prelude::*;
use crate::core::prelude::*;
use crate::spawn::enhance_map::enhance_ghost_house::EnhanceGhostHousePlugin;
use crate::spawn::enhance_map::enhance_maze::EnhanceMazePlugin;

pub(super) struct EnhanceMapPlugin;

impl Plugin for EnhanceMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EnhanceMazePlugin,
                EnhanceGhostHousePlugin
            ))
            .add_systems(
                OnEnter(Spawn(EnhanceMap)),
                add_spatial_bundle_to_map,
            )
        ;
    }
}

fn add_spatial_bundle_to_map(
    mut commands: Commands,
    maps: Query<Entity, With<Map>>,
) {
    commands.entity(maps.single()).insert(SpatialBundle::default());
}