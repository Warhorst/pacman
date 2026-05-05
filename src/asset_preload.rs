use std::{fs::read_dir, path::Path};

use bevy::{asset::LoadState, prelude::*, state::state::FreelyMutableState};

/// Plugin that starts loading all assets in the asset folder for a given state and
/// automatically switches to another given state when everything is loaded.
pub struct AssetPreloadPlugin<
    LoadingState: States + FreelyMutableState,
    NextState: States + FreelyMutableState,
> {
    /// The state the plugin will start and keep loading all assets.
    loading_state: LoadingState,
    /// The state the plugin will switch to when all assets are loaded
    next_state: NextState,
    /// The path from where the paths to load the assets from originate
    path_source: PathSource,
}

impl<LoadingState: States + FreelyMutableState, NextState: States + FreelyMutableState>
    AssetPreloadPlugin<LoadingState, NextState>
{
    /// Load all assets directly from the assets folder. This requires access to the file system and will therefore
    /// not work in WASM.
    #[allow(dead_code)]
    pub fn load_from_asset_folder(
        loading_state: LoadingState,
        next_state: NextState,
    ) -> Self {
        Self {
            loading_state,
            next_state,
            path_source: PathSource::LoadFromFolder,
        }
    }

    /// Load all the given assets only. This variant can be used to preload the whole asset folder in a WASM environment. Use the
    /// load_assets macro to provide a vector of all asset paths which is created at compile time.
    pub fn load_given_paths<S: ToString>(
        loading_state: LoadingState,
        next_state: NextState,
        paths: impl IntoIterator<Item = S>,
    ) -> Self {
        Self {
            loading_state,
            next_state,
            path_source: PathSource::GivenPaths(paths.into_iter().map(|s| s.to_string()).collect()),
        }
    }
}

impl<LoadingState: States + FreelyMutableState, NextState: States + FreelyMutableState> Plugin
    for AssetPreloadPlugin<LoadingState, NextState>
{
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_message::<AssetPreloadUpdate>()
            .add_systems(
                OnEnter(self.loading_state.clone()),
                start_asset_loading(self.path_source.clone()),
            )
            .add_systems(
                Update,
                switch_state_when_all_loaded(self.next_state.clone())
                    .run_if(in_state(self.loading_state.clone())),
            );
    }
}

#[derive(Message)]
#[allow(dead_code)]
pub struct AssetPreloadUpdate {
    /// The amount of assets which are already loaded
    pub num_loaded: usize,
    /// The amount of all assets which get currently loaded or are already loaded
    pub num_loading: usize,
}

#[derive(Clone)]
enum PathSource {
    /// Load all asset paths from the asset folder.
    LoadFromFolder,
    /// Use a given list of paths to load the assets
    GivenPaths(Vec<String>),
}

impl PathSource {
    fn load_assets(
        &self,
        asset_server: &AssetServer,
    ) -> Vec<UntypedHandle> {
        match self {
            PathSource::LoadFromFolder => {
                let paths = load_asset_paths();
                paths
                    .into_iter()
                    .map(|p| asset_server.load_untyped(p).untyped())
                    .collect()
            }
            PathSource::GivenPaths(paths) => paths
                .iter()
                .map(|p| asset_server.load_untyped(p).untyped())
                .collect(),
        }
    }
}

/// Resource that holds handles to all assets in the assets folder. This only exists to ensure
/// the assets don't get unloaded because nobody is using them.
#[derive(Resource)]
struct LoadedAssets(Vec<UntypedHandle>);

impl LoadedAssets {
    fn iter(&self) -> impl Iterator<Item = &UntypedHandle> {
        self.0.iter()
    }

    fn num_loading_assets(&self) -> usize {
        self.0.len()
    }
}

fn start_asset_loading(path_source: PathSource) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands: Commands, asset_server: Res<AssetServer>| {
        let handles = path_source.load_assets(&asset_server);
        commands.insert_resource(LoadedAssets(handles));
    }
}

// TODO copied code, fix!
fn load_asset_paths() -> Vec<String> {
    load_asset_paths_recursive(Path::new("./assets")).expect("the assets folder should exist")
}

fn load_asset_paths_recursive(path: &Path) -> std::io::Result<Vec<String>> {
    let mut files = vec![];

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(load_asset_paths_recursive(&path)?);
            } else {
                let path_str = path
                    .to_str()
                    .unwrap()
                    .replace('\\', "/")
                    .replace("./assets/", "")
                    .to_string();
                files.push(path_str);
            }
        }
    }

    Ok(files)
}

#[allow(clippy::type_complexity)]
fn switch_state_when_all_loaded<S: States + FreelyMutableState>(
    followup_state: S
) -> impl Fn(Res<AssetServer>, Res<LoadedAssets>, MessageWriter<AssetPreloadUpdate>, ResMut<NextState<S>>)
{
    move |asset_server, loaded_assets, mut event_writer, mut next_state| {
        let num_loaded = loaded_assets
            .iter()
            .filter(|uh| match asset_server.load_state(uh.id()) {
                LoadState::Loaded => true,
                LoadState::Failed(_) => panic!("load failed!"),
                _ => false,
            })
            .count();

        event_writer.write(AssetPreloadUpdate {
            num_loaded,
            num_loading: loaded_assets.num_loading_assets(),
        });

        if num_loaded == loaded_assets.num_loading_assets() {
            next_state.set(followup_state.clone())
        }
    }
}
