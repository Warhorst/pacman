use std::collections::HashMap;
use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::*;

pub fn start_base_assets_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BaseAssetsToLoad::from_keys_and_handles([
        load(FONT, &asset_server),
        load(MAP, &asset_server),
        load(EATEN_UP, &asset_server),
        load(EATEN_DOWN, &asset_server),
        load(EATEN_LEFT, &asset_server),
        load(EATEN_RIGHT, &asset_server),
    ]))
}

pub fn move_assets_to_the_game_asset_handles_when_loaded(
    mut game_asset_handles: ResMut<GameAssetHandles>,
    mut base_assets_to_load: ResMut<BaseAssetsToLoad>,
    asset_server: Res<AssetServer>,
) {
    let all_loaded = match asset_server.get_group_load_state(base_assets_to_load.handle_id_iter()) {
        LoadState::Failed => panic!("some assets failed loading, abort"),
        LoadState::Loaded => true,
        _ => false
    };

    if all_loaded {
        for (k, h) in base_assets_to_load.flush_keys_and_handles() {
            game_asset_handles.add_handle(k, h)
        }
    }
}

pub fn notify_when_all_base_assets_loaded(
    mut event_writer: EventWriter<EAllBaseAssetsLoaded>,
    base_assets_to_load: ResMut<BaseAssetsToLoad>,
) {
    if base_assets_to_load.is_loaded() {
        event_writer.send(EAllBaseAssetsLoaded);
    }
}

fn load<S: ToString>(key: S, asset_server: &AssetServer) -> (S, HandleUntyped) {
    let handle = asset_server.load_untyped(&key.to_string());
    (key, handle)
}

/// Fired when all base assets are loaded
pub struct EAllBaseAssetsLoaded;

/// Some asset handles with keys that are currently being loaded.
/// These are base assets because nothing special must be done when they are loaded. They just get moved into the game asset handles.
pub struct BaseAssetsToLoad {
    keys_and_handles: HashMap<String, HandleUntyped>,
}

impl BaseAssetsToLoad {
    pub fn from_keys_and_handles<S: ToString, I: IntoIterator<Item=(S, HandleUntyped)>>(keys_and_handles: I) -> Self {
        BaseAssetsToLoad {
            keys_and_handles: keys_and_handles.into_iter()
                .map(|(k, h)| (k.to_string(), h))
                .collect()
        }
    }

    pub fn handle_id_iter<'a>(&'a self) -> impl IntoIterator<Item=HandleId> + 'a {
        self.keys_and_handles.values().map(|h| h.id)
    }

    pub fn flush_keys_and_handles(&mut self) -> Vec<(String, HandleUntyped)> {
        let keys = self.keys_and_handles.keys().map(Clone::clone).collect::<Vec<_>>();

        keys.into_iter()
            .map(|k| {
                let value = self.keys_and_handles.remove(&k).unwrap();
                (k, value)
            })
            .collect()
    }

    /// All assets are loaded if no handles remain
    pub fn is_loaded(&self) -> bool {
        self.keys_and_handles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::HandleId;
    use bevy::prelude::*;

    use crate::game_assets::base_assets::BaseAssetsToLoad;
    use crate::sprite_sheet::SpriteSheet;

    #[test]
    fn it_can_be_created_from_an_iterator_of_keys_and_handles() {
        let base_assets_to_load = BaseAssetsToLoad::from_keys_and_handles([
            ("my_image", HandleUntyped::weak(HandleId::random::<Image>())),
            ("my_font", HandleUntyped::weak(HandleId::random::<Font>())),
            ("my_sheet", HandleUntyped::weak(HandleId::random::<SpriteSheet>()))
        ]);

        assert_eq!(base_assets_to_load.keys_and_handles.len(), 3)
    }

    #[test]
    fn it_can_return_all_handle_ids_as_an_iterator() {
        let image = HandleUntyped::weak(HandleId::random::<Image>());
        let font = HandleUntyped::weak(HandleId::random::<Font>());
        let sheet = HandleUntyped::weak(HandleId::random::<SpriteSheet>());

        let base_assets_to_load = BaseAssetsToLoad::from_keys_and_handles([
            ("my_image", image.clone()),
            ("my_font", font.clone()),
            ("my_sheet", sheet.clone())
        ]);

        let ids = base_assets_to_load.handle_id_iter().into_iter().collect::<Vec<_>>();

        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&image.id));
        assert!(ids.contains(&font.id));
        assert!(ids.contains(&sheet.id));
    }

    #[test]
    fn it_can_flush_all_keys_and_handles() {
        let image = HandleUntyped::weak(HandleId::random::<Image>());
        let font = HandleUntyped::weak(HandleId::random::<Font>());
        let mut base_assets_to_load = BaseAssetsToLoad::from_keys_and_handles([
            ("my_image", image.clone()),
            ("my_font", font.clone()),
        ]);

        assert_eq!(base_assets_to_load.keys_and_handles.len(), 2);

        let keys_and_handles = base_assets_to_load.flush_keys_and_handles();
        assert!(base_assets_to_load.keys_and_handles.is_empty());
        assert_eq!(keys_and_handles.len(), 2);
        assert!(keys_and_handles.contains(&("my_image".to_string(), image)));
        assert!(keys_and_handles.contains(&("my_font".to_string(), font)));
    }

    #[test]
    fn it_can_tell_if_everything_is_loaded() {
        let not_fully_loaded = BaseAssetsToLoad::from_keys_and_handles([
            ("my_image", HandleUntyped::weak(HandleId::random::<Image>())),
        ]);

        let fully_loaded = BaseAssetsToLoad::from_keys_and_handles::<String, _>([]);

        assert_eq!(not_fully_loaded.is_loaded(), false);
        assert_eq!(fully_loaded.is_loaded(), true);
    }
}