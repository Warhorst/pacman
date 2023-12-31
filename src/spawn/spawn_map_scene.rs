use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use crate::core::prelude::*;

pub(super) struct SpawnMapScenePlugin;

impl Plugin for SpawnMapScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Spawn(SpawnMapScene)),
                spawn_map_scene
            )
            .add_systems(
                Update,
                switch_state_when_map_spawned.run_if(in_state(Spawn(SpawnMapScene)))
            )
        ;
    }
}

/// The currently loading map scene entity
#[derive(Resource, Deref)]
struct LoadingMap(Entity);

fn spawn_map_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let entity = commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(MAP_SCENE_PATH),
        ..default()
    }).id();

    commands.insert_resource(LoadingMap(entity));
}

fn switch_state_when_map_spawned(
    mut events: EventReader<SceneInstanceReady>,
    loading_map: Res<LoadingMap>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        if event.parent == **loading_map {
            next_state.set(Spawn(EnhanceMap));
        }
    }
}