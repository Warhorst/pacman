use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use crate::core::prelude::*;

pub(super) struct SpawnMapScenePlugin;

impl Plugin for SpawnMapScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(SpawnMaze(SpawnMapScene)),
                spawn_map_scene,
            )
            .add_systems(
                Startup,
                switch_state_when_map_spawned,
            )
        ;
    }
}

/// The currently loading map scene entity
#[derive(Resource, Deref)]
struct LoadingMap(Entity);

fn spawn_map_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let entity = commands.spawn(DynamicSceneRoot(asset_server.load(MAP_SCENE_PATH))).id();

    commands.insert_resource(LoadingMap(entity));
}

fn switch_state_when_map_spawned(
    mut commands: Commands,
) {
    commands.add_observer(|_: Trigger<SceneInstanceReady>, mut next_state: ResMut<NextState<GameState>>| {
        // todo in the current bevy version at the time of writing (0.15.0), I don't really understand how to check if a
        //  specific scene was spawned, only that some scene was spawned. But as I only have one, this should be fine
        next_state.set(SpawnMaze(EnhanceMap));
    });
}