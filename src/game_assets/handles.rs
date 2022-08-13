use std::collections::HashMap;
use bevy::asset::Asset;
use bevy::prelude::*;

/// Resource that holds handles for all assets used in the game.
pub struct GameAssetHandles {
    handles: HashMap<String, HandleUntyped>,
}

impl GameAssetHandles {
    #[cfg(test)]
    pub fn from_handles<H, S>(handles: H) -> Self
        where H: IntoIterator<Item=(S, HandleUntyped)>, S: ToString {
        GameAssetHandles {
            handles: handles.into_iter().map(|(key, h)| (key.to_string(), h)).collect()
        }
    }

    pub fn get_handle<S: ToString, T: Asset>(&self, key: S) -> Handle<T> {
        self.handles.get(&key.to_string()).expect("the requested handle should be registered").clone().typed()
    }

    pub fn add_handle(&mut self, key: impl ToString, handle: HandleUntyped) {
        self.handles.insert(key.to_string(), handle);
    }
}

impl Default for GameAssetHandles {
    fn default() -> Self {
        GameAssetHandles {
            handles: HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::HandleId;
    use bevy::prelude::*;
    use crate::game_assets::handles::GameAssetHandles;
    use crate::sprite_sheet::SpriteSheet;

    #[test]
    pub fn can_be_created_from_iterator_of_keys_and_untyped_handles() {
        let handles = [
            ("my_image", HandleUntyped::weak(HandleId::random::<Image>())),
            ("my_font", HandleUntyped::weak(HandleId::random::<Font>())),
            ("my_sheet", HandleUntyped::weak(HandleId::random::<SpriteSheet>())),
        ];
        let num_handles = handles.len();

        let assets = GameAssetHandles::from_handles(handles);
        assert_eq!(assets.handles.len(), num_handles)
    }

    #[test]
    pub fn a_registered_handle_can_be_retrieved() {
        let handle = Handle::weak(HandleId::random::<Image>());
        let key = "my_image";
        let assets = GameAssetHandles::from_handles(Some((key, handle.clone_untyped())));

        let image_handle: Handle<Image> = assets.get_handle(key);

        assert_eq!(handle, image_handle)
    }

    #[test]
    #[should_panic]
    pub fn retrieving_an_unregistered_handle_panics() {
        let handle: Handle<Image> = Handle::weak(HandleId::random::<Image>());
        let assets = GameAssetHandles::from_handles(Some(("my_image", handle.clone_untyped())));

        assets.get_handle::<_, Image>("not_my_image");
    }

    #[test]
    pub fn a_new_handle_can_be_added_after_creation() {
        let mut assets = GameAssetHandles::from_handles([
            ("my_image", HandleUntyped::weak(HandleId::random::<Image>()))
        ]);

        assert_eq!(assets.handles.len(), 1);

        let new_asset = HandleUntyped::weak(HandleId::random::<Image>());
        assets.add_handle("my_other_image", new_asset);

        assert_eq!(assets.handles.len(), 2);
    }
}