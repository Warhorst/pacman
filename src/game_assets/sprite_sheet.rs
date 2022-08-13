use bevy::prelude::*;
use std::collections::HashMap;
use bevy::asset::{HandleId, LoadState};
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::sprite_sheets::*;
use crate::sprite_sheet::aseprite_data::AsepriteData;
use crate::sprite_sheet::{split_image_by_rectangles, SpriteSheet};

pub fn start_sprite_sheet_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SheetsToLoad::from_keys_and_handles(
        load_sheets(
            &asset_server,
            [
                PACMAN_WALKING_UP,
                PACMAN_WALKING_DOWN,
                PACMAN_WALKING_LEFT,
                PACMAN_WALKING_RIGHT,
                PACMAN_DYING,
                OUTER_WALL_CORNER_BLINKING,
                OUTER_WALL_BLINKING,
                INNER_WALL_CORNER_BLINKING,
                INNER_WALL_BLINKING,
                GHOST_WALL_CORNER_BLINKING,
                GHOST_WALL_BLINKING,
                BLINKY_UP,
                BLINKY_DOWN,
                BLINKY_LEFT,
                BLINKY_RIGHT,
                PINKY_UP,
                PINKY_DOWN,
                PINKY_LEFT,
                PINKY_RIGHT,
                INKY_UP,
                INKY_DOWN,
                INKY_LEFT,
                INKY_RIGHT,
                CLYDE_UP,
                CLYDE_DOWN,
                CLYDE_LEFT,
                CLYDE_RIGHT,
                FRIGHTENED,
                FRIGHTENED_BLINKING,
            ],
        )));
}

/// Load the image and data sheet for every given key.
///
/// The key is the identifier and path of the sprite sheet. To load its
/// data and image, it is used to generate the correct path.
///
/// Following conventions apply:
/// - the sheet key is the path relative to the assets directory without a file ending
/// - the image must be in the same path with a png ending
/// - the data must be in the same path with an aseprite.json ending
fn load_sheets<'a>(
    asset_server: &'a AssetServer,
    keys: impl IntoIterator<Item=&'static str> + 'a,
) -> impl IntoIterator<Item=(&'static str, (Handle<Image>, Handle<AsepriteData>))> + 'a {
    keys.into_iter().map(|k|
        (k,
         (
             asset_server.load(&format!("{k}.png")),
             asset_server.load(&format!("{k}.aseprite.json"))
         )
        )
    )
}

pub fn create_sprite_sheets_when_texture_and_data_are_loaded(
    mut game_asset_handles: ResMut<GameAssetHandles>,
    mut images: ResMut<Assets<Image>>,
    sheet_data: Res<Assets<AsepriteData>>,
    mut sprite_sheets: ResMut<Assets<SpriteSheet>>,
    mut sheets_to_load: ResMut<SheetsToLoad>,
    asset_server: Res<AssetServer>,
) {
    let mut loaded_assets_keys = vec![];

    for (key, handle_ids) in sheets_to_load.key_and_handle_ids_iter() {
        match asset_server.get_group_load_state(handle_ids) {
            LoadState::Failed => panic!("some assets failed loading, abort"),
            LoadState::Loaded => loaded_assets_keys.push(key.clone()),
            _ => ()
        }
    }

    for key in loaded_assets_keys {
        let (image_handle, data_handle) = sheets_to_load.remove_handles(&key);
        let image = images.remove(image_handle).expect("image should be loaded");
        let data = sheet_data.get(&data_handle).expect("data should be loaded");

        let sheet = SpriteSheet::new(
            split_image_by_rectangles(&image, data.rect_iter())
                .into_iter()
                .map(|image| images.add(image))
        );

        let sheet_handle = sprite_sheets.add(sheet);
        game_asset_handles.add_handle(key, sheet_handle.clone_untyped())
    }
}

pub fn notify_when_all_sprite_sheets_loaded(
    sheets_to_load: Res<SheetsToLoad>,
    mut event_writer: EventWriter<EAllSheetsLoaded>,
) {
    if sheets_to_load.is_loaded() {
        event_writer.send(EAllSheetsLoaded)
    }
}

/// Fired when all sprite sheets are loaded.
pub struct EAllSheetsLoaded;

pub struct SheetsToLoad {
    key_and_handles: HashMap<String, (Handle<Image>, Handle<AsepriteData>)>,
}

impl SheetsToLoad {
    pub fn from_keys_and_handles<S: ToString, I: IntoIterator<Item=(S, (Handle<Image>, Handle<AsepriteData>))>>(keys_and_handles: I) -> Self {
        SheetsToLoad {
            key_and_handles: keys_and_handles.into_iter()
                .map(|(key, h)| (key.to_string(), h))
                .collect()
        }
    }

    pub fn key_and_handle_ids_iter<'a>(&'a self) -> impl IntoIterator<Item=(&'a String, [HandleId; 2])> + 'a {
        self.key_and_handles
            .iter()
            .map(|(k, hs)| (k, [hs.0.id, hs.1.id]))
    }

    pub fn remove_handles(&mut self, key: &impl ToString) -> (Handle<Image>, Handle<AsepriteData>) {
        self.key_and_handles.remove(&key.to_string()).expect("the given key should exist")
    }

    /// All sheets are loaded if no handles remain
    pub fn is_loaded(&self) -> bool {
        self.key_and_handles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::HandleId;
    use bevy::prelude::*;
    use crate::game_assets::sprite_sheet::SheetsToLoad;
    use crate::sprite_sheet::aseprite_data::AsepriteData;

    #[test]
    fn it_can_be_created_from_an_iterator_of_keys_to_handles() {
        let first_handles = (
            Handle::weak(HandleId::random::<Image>()),
            Handle::weak(HandleId::random::<AsepriteData>())
        );
        let second_handles = (
            Handle::weak(HandleId::random::<Image>()),
            Handle::weak(HandleId::random::<AsepriteData>())
        );

        let sheets_to_load = SheetsToLoad::from_keys_and_handles([
            ("foo", first_handles.clone()),
            ("bar", second_handles.clone())
        ]);

        assert_eq!(sheets_to_load.key_and_handles.len(), 2);
        assert_eq!(sheets_to_load.key_and_handles.get(&"foo".to_string()), Some(&first_handles));
        assert_eq!(sheets_to_load.key_and_handles.get(&"bar".to_string()), Some(&second_handles));
    }

    #[test]
    fn it_can_return_an_iterator_of_key_and_handle_ids() {
        let handles = (
            Handle::weak(HandleId::random::<Image>()),
            Handle::weak(HandleId::random::<AsepriteData>())
        );

        let sheets_to_load = SheetsToLoad::from_keys_and_handles([
            ("foo", handles.clone()),
        ]);

        let mut key_and_handle_ids_iter = sheets_to_load.key_and_handle_ids_iter().into_iter();

        let next_opt = key_and_handle_ids_iter.next();
        assert!(next_opt.is_some());
        let next = next_opt.unwrap();
        assert_eq!(&"foo".to_string(), next.0);
        assert_eq!([handles.0.id, handles.1.id], next.1);
    }

    #[test]
    fn it_can_remove_handles_by_a_given_key_and_return_the_value() {
        let handles = (
            Handle::weak(HandleId::random::<Image>()),
            Handle::weak(HandleId::random::<AsepriteData>())
        );

        let mut sheets_to_load = SheetsToLoad::from_keys_and_handles([
            ("foo", handles.clone()),
        ]);

        let extracted = sheets_to_load.remove_handles(&"foo");

        assert_eq!(sheets_to_load.key_and_handles.len(), 0);
        assert_eq!(handles, extracted)
    }

    #[test]
    fn it_can_tell_if_everything_is_loaded() {
        let handles = (
            Handle::weak(HandleId::random::<Image>()),
            Handle::weak(HandleId::random::<AsepriteData>())
        );

        let not_fully_loaded = SheetsToLoad::from_keys_and_handles([
            ("foo", handles.clone()),
        ]);

        let fully_loaded = SheetsToLoad::from_keys_and_handles::<String, _>([]);

        assert_eq!(not_fully_loaded.is_loaded(), false);
        assert_eq!(fully_loaded.is_loaded(), true);
    }
}