mod enhance_maze;
mod enhance_ghost_house;
mod enhance_tunnels;

use bevy::prelude::*;
use crate::core::prelude::*;
use crate::spawn::enhance_map::enhance_ghost_house::EnhanceGhostHousePlugin;
use crate::spawn::enhance_map::enhance_maze::EnhanceMazePlugin;
use crate::spawn::enhance_map::enhance_tunnels::EnhanceTunnelPlugin;

pub(super) struct EnhanceMapPlugin;

impl Plugin for EnhanceMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EnhanceMazePlugin,
                EnhanceGhostHousePlugin,
                EnhanceTunnelPlugin
            ))
            .add_systems(
                OnEnter(Spawn(EnhanceMap)),
                add_spatial_bundle_to_map,
            )
            .add_systems(
                Update,
                switch_state_after_enhance.run_if(in_state(Spawn(EnhanceMap)))
            )
        ;
    }
}

fn add_spatial_bundle_to_map(
    mut commands: Commands,
    maps: Query<Entity, With<Map>>,
) {
    commands.entity(maps.single()).insert((
        Transform::default(),
        Visibility::default(),
    ));
}

fn switch_state_after_enhance(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(Game(Start))
}