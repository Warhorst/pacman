use bevy::asset::{Asset, HandleId, LoadState};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::sprite_sheet::aseprite_data::AsepriteData;
use crate::sprite_sheet::{split_image_by_rectangles, SpriteSheet};

/// Container for assets that are already loaded. Used to load all assets at the app start
/// and use the instantly. Primarily reduces pop-ins of assets.
pub struct LoadedAssets {
    path_handle_map: HashMap<String, HandleUntyped>,
}

impl LoadedAssets {
    /// Start loading the assets when on pc.
    ///
    /// All assets are loaded using the asset server. The asset paths are retrieved afterwards.
    #[cfg(not(target_family = "wasm"))]
    pub fn start_load(asset_server: &AssetServer) -> Self {
        let mut asset_folder = bevy::asset::FileAssetIo::get_base_path();
        asset_folder.push("assets");
        let handles = asset_server.load_folder(asset_folder).unwrap();

        LoadedAssets {
            path_handle_map: handles.into_iter()
                .map(|handle| (asset_server.get_handle_path(handle.id).unwrap(), handle))
                .map(|(asset_path, handle)| (asset_path.path().to_str().unwrap().replace("\\", "/"), handle))
                .collect()
        }
    }

    /// Start loading the assets when on WASM.
    ///
    /// In WASM, a folder cannot be loaded from the asset server. The workaround for this issue
    /// is to use a build script, which scans the assets folder and puts every file path relative to "assets"
    /// into an array. The asset server then loads the asset for every path.
    #[cfg(target_family = "wasm")]
    pub fn start_load(asset_server: &AssetServer) -> Self {
        let asset_paths = include!(concat!(env!("OUT_DIR"), "/asset_paths.rs"));

        LoadedAssets {
            path_handle_map: asset_paths.into_iter().map(|path| (path.to_string(), asset_server.load_untyped(path))).collect()
        }
    }

    pub fn get_handle<S: ToString, T: Asset>(&self, key: S) -> Handle<T> {
        self.path_handle_map.get(&key.to_string()).expect("the requested handle should be registered").clone().typed()
    }

    pub fn get_asset<'a, T: Asset>(&'a self, key: impl ToString, assets: &'a Assets<T>) -> &T {
        let handle = self.get_handle::<_, T>(key);
        assets.get(&handle).unwrap()
    }

    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        match asset_server.get_group_load_state(self.handle_ids()) {
            LoadState::Failed => panic!("some assets failed loading, abort"),
            LoadState::Loaded => true,
            _ => false
        }
    }

    fn handle_ids<'a>(&'a self) -> impl IntoIterator<Item=HandleId> + 'a {
        self.path_handle_map.values().map(|handle| handle.id)
    }

    pub fn add_sprite_sheets(&mut self, sheets: &mut Assets<SpriteSheet>, images: &mut Assets<Image>, aseprite_data: &Assets<AsepriteData>) {
        let sheet_json_paths = self.path_handle_map
            .keys()
            .filter(|path| path.ends_with(".aseprite.json"))
            .map(Clone::clone)
            .collect::<Vec<_>>();

        let images_and_jsons = sheet_json_paths
            .into_iter()
            .map(|json_path| {
                let ident = json_path.replace(".aseprite.json", "");
                let image_path = self.path_handle_map
                    .keys()
                    .filter(|path| !path.ends_with(".aseprite.json"))
                    .filter(|path| path.split(".").collect::<Vec<_>>()[0] == ident)
                    .next()
                    .unwrap()
                    .clone();
                (image_path, json_path)
            })
            .collect::<Vec<_>>();

        for (image_path, json_path) in images_and_jsons {
            let ident = json_path.replace(".aseprite.json", "");
            let (image_handle, data_handle) = (
                self.path_handle_map.remove(&image_path).unwrap().typed::<Image>(),
                self.path_handle_map.remove(&json_path).unwrap().typed::<AsepriteData>(),
            );

            let (image, data) = (
                images.remove(&image_handle).unwrap(),
                aseprite_data.get(&data_handle).unwrap()
            );

            let sheet = SpriteSheet::new(
                split_image_by_rectangles(&image, data.rect_iter())
                    .into_iter()
                    .map(|image| images.add(image))
            );

            self.path_handle_map.insert(ident, sheets.add(sheet).clone_untyped());
        }
    }
}