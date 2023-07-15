use bevy::asset::{Asset, HandleId, LoadState};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game_assets::sprite_sheet::aseprite_data::AsepriteData;
use crate::game_assets::sprite_sheet::{split_image_by_rectangles, SpriteSheet};

type ImagePath = String;
type JSONPath = String;

/// Container for assets that are already loaded. Used to load all assets at the app start
/// and use them instantly. Primarily reduces pop-ins of assets.
#[derive(Resource)]
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
                .map(|handle| (asset_server.get_handle_path(handle.id()).unwrap(), handle))
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
        self.path_handle_map.values().map(|handle| handle.id())
    }

    pub fn add_sprite_sheets(&mut self, sheets: &mut Assets<SpriteSheet>, images: &mut Assets<Image>, aseprite_data: &mut Assets<AsepriteData>) {
        let images_and_jsons = self.get_image_json_paths();

        for (image_path, json_path) in images_and_jsons {
            self.add_sprite_sheet(image_path, json_path, sheets, images, aseprite_data)
        }
    }

    /// Collect all json paths and related images
    fn get_image_json_paths(&self) -> Vec<(ImagePath, JSONPath)> {
        self.path_handle_map
            .keys()
            .filter(|path| path.ends_with(".aseprite.json"))
            .map(Clone::clone)
            .map(|json_path| {
                let image_path = self.get_matching_image_path_for_json_path(&json_path);
                (image_path, json_path)
            })
            .collect()
    }

    fn get_matching_image_path_for_json_path(&self, json_path: &JSONPath) -> String {
        let ident = json_path.replace(".aseprite.json", "");
        self.path_handle_map
            .keys()
            .filter(|path| !path.ends_with(".aseprite.json"))
            .filter(|path| path.split(".").collect::<Vec<_>>()[0] == ident)
            .next()
            .unwrap()
            .clone()
    }

    /// Create a sprite sheet by retrieving the image and data for the given paths. When the sheet was created, it will be stored in the sprite sheets assets
    /// and its handle will be moved to the path handle map
    fn add_sprite_sheet(&mut self, image_path: ImagePath, json_path: JSONPath, sheets: &mut Assets<SpriteSheet>, images: &mut Assets<Image>, aseprite_data: &mut Assets<AsepriteData>) {
        let ident = json_path.replace(".aseprite.json", "");
        let (image_handle, data_handle) = self.remove_image_and_json_handles(image_path, json_path);
        let (image, data) = self.remove_image_and_json_from_assets(image_handle, data_handle, images, aseprite_data);

        let sheet = SpriteSheet::new(
            split_image_by_rectangles(&image, data.rect_iter())
                .into_iter()
                .map(|image| images.add(image))
        );

        self.path_handle_map.insert(ident, sheets.add(sheet).clone_untyped());
    }

    /// Remove the handles for the provided image and json from the map.
    fn remove_image_and_json_handles(&mut self, image_path: ImagePath, json_path: JSONPath) -> (Handle<Image>, Handle<AsepriteData>) {
        (
            self.path_handle_map.remove(&image_path).unwrap().typed::<Image>(),
            self.path_handle_map.remove(&json_path).unwrap().typed::<AsepriteData>(),
        )
    }

    /// Remove and return the image and JSON asset from the assets resources, as they are no longer needed.
    fn remove_image_and_json_from_assets(&self, image_handle: Handle<Image>, data_handle: Handle<AsepriteData>, images: &mut Assets<Image>, aseprite_data: &mut Assets<AsepriteData>) -> (Image, AsepriteData) {
        (
            images.remove(&image_handle).unwrap(),
            aseprite_data.remove(&data_handle).unwrap()
        )
    }
}