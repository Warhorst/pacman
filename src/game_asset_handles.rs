use std::collections::HashMap;
use bevy::asset::{Asset, HandleId, LoadState};
use bevy::prelude::*;
use crate::life_cycle::LifeCycle::Loading;
use keys::*;

pub mod keys {
    pub const FONT: &'static str = "fonts/FiraSans-Bold.ttf";

    pub const PACMAN_WALKING_UP: &'static str = "textures/pacman/pacman_walking_up.sheet.png";
    pub const PACMAN_WALKING_DOWN: &'static str = "textures/pacman/pacman_walking_down.sheet.png";
    pub const PACMAN_WALKING_LEFT: &'static str = "textures/pacman/pacman_walking_left.sheet.png";
    pub const PACMAN_WALKING_RIGHT: &'static str = "textures/pacman/pacman_walking_right.sheet.png";
    pub const PACMAN_DYING: &'static str = "textures/pacman/pacman_dying.sheet.png";

    pub const BLINKY_UP: &'static str = "textures/ghost/blinky_up.sheet.png";
    pub const BLINKY_DOWN: &'static str = "textures/ghost/blinky_down.sheet.png";
    pub const BLINKY_LEFT: &'static str = "textures/ghost/blinky_left.sheet.png";
    pub const BLINKY_RIGHT: &'static str = "textures/ghost/blinky_right.sheet.png";

    pub const PINKY_UP: &'static str = "textures/ghost/pinky_up.sheet.png";
    pub const PINKY_DOWN: &'static str = "textures/ghost/pinky_down.sheet.png";
    pub const PINKY_LEFT: &'static str = "textures/ghost/pinky_left.sheet.png";
    pub const PINKY_RIGHT: &'static str = "textures/ghost/pinky_right.sheet.png";

    pub const INKY_UP: &'static str = "textures/ghost/inky_up.sheet.png";
    pub const INKY_DOWN: &'static str = "textures/ghost/inky_down.sheet.png";
    pub const INKY_LEFT: &'static str = "textures/ghost/inky_left.sheet.png";
    pub const INKY_RIGHT: &'static str = "textures/ghost/inky_right.sheet.png";

    pub const CLYDE_UP: &'static str = "textures/ghost/clyde_up.sheet.png";
    pub const CLYDE_DOWN: &'static str = "textures/ghost/clyde_down.sheet.png";
    pub const CLYDE_LEFT: &'static str = "textures/ghost/clyde_left.sheet.png";
    pub const CLYDE_RIGHT: &'static str = "textures/ghost/clyde_right.sheet.png";

    pub const FRIGHTENED: &'static str = "textures/ghost/frightened.sheet.png";
    pub const FRIGHTENED_BLINKING: &'static str = "textures/ghost/frightened_blinking.sheet.png";

    pub const EATEN_UP: &'static str = "textures/ghost/eaten_up.png";
    pub const EATEN_DOWN: &'static str = "textures/ghost/eaten_down.png";
    pub const EATEN_LEFT: &'static str = "textures/ghost/eaten_left.png";
    pub const EATEN_RIGHT: &'static str = "textures/ghost/eaten_right.png";

    pub const OUTER_WALL_CORNER: &'static str = "textures/walls/outer_wall_corner.png";
    pub const OUTER_WALL_CORNER_BLINKING: &'static str = "textures/walls/outer_wall_corner_blinking.sheet.png";
    pub const OUTER_WALL: &'static str = "textures/walls/outer_wall.png";
    pub const OUTER_WALL_BLINKING: &'static str = "textures/walls/outer_wall_blinking.sheet.png";
    pub const INNER_WALL_CORNER: &'static str = "textures/walls/inner_wall_corner.png";
    pub const INNER_WALL_CORNER_BLINKING: &'static str = "textures/walls/inner_wall_corner_blinking.sheet.png";
    pub const INNER_WALL: &'static str = "textures/walls/inner_wall.png";
    pub const INNER_WALL_BLINKING: &'static str = "textures/walls/inner_wall_blinking.sheet.png";
    pub const GHOST_WALL_CORNER: &'static str = "textures/walls/ghost_house_wall_corner.png";
    pub const GHOST_WALL_CORNER_BLINKING: &'static str = "textures/walls/ghost_house_wall_corner_blinking.sheet.png";
    pub const GHOST_WALL: &'static str = "textures/walls/ghost_house_wall.png";
    pub const GHOST_WALL_BLINKING: &'static str = "textures/walls/ghost_house_wall_blinking.sheet.png";
}

pub struct GameAssetHandlesPlugin;

impl Plugin for GameAssetHandlesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EAllAssetsLoaded>()
            .add_system_set(
                SystemSet::on_enter(Loading).with_system(create_game_asset_handles)
            )
            .add_system_set(
                SystemSet::on_update(Loading).with_system(notify_when_all_assets_loaded)
            )
        ;
    }
}

/// Load all required game assets here and store their handles.
///
/// TODO: reading the whole asset folder might be easier, but the related method from the asset server (load_folder) does not return the paths of the
///  loaded assets. And loading from a directory directly does not work in WASM.
fn create_game_asset_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(GameAssetHandles::from_handles([
        load(FONT, &asset_server),
        load(PACMAN_WALKING_UP, &asset_server),
        load(PACMAN_WALKING_DOWN, &asset_server),
        load(PACMAN_WALKING_LEFT, &asset_server),
        load(PACMAN_WALKING_RIGHT, &asset_server),
        load(PACMAN_DYING, &asset_server),
        load(BLINKY_UP, &asset_server),
        load(BLINKY_DOWN, &asset_server),
        load(BLINKY_LEFT, &asset_server),
        load(BLINKY_RIGHT, &asset_server),
        load(PINKY_UP, &asset_server),
        load(PINKY_DOWN, &asset_server),
        load(PINKY_LEFT, &asset_server),
        load(PINKY_RIGHT, &asset_server),
        load(INKY_UP, &asset_server),
        load(INKY_DOWN, &asset_server),
        load(INKY_LEFT, &asset_server),
        load(INKY_RIGHT, &asset_server),
        load(CLYDE_UP, &asset_server),
        load(CLYDE_DOWN, &asset_server),
        load(CLYDE_LEFT, &asset_server),
        load(CLYDE_RIGHT, &asset_server),
        load(FRIGHTENED, &asset_server),
        load(FRIGHTENED_BLINKING, &asset_server),
        load(EATEN_UP, &asset_server),
        load(EATEN_DOWN, &asset_server),
        load(EATEN_LEFT, &asset_server),
        load(EATEN_RIGHT, &asset_server),
        load(OUTER_WALL_CORNER, &asset_server),
        load(OUTER_WALL_CORNER_BLINKING, &asset_server),
        load(OUTER_WALL, &asset_server),
        load(OUTER_WALL_BLINKING, &asset_server),
        load(INNER_WALL_CORNER, &asset_server),
        load(INNER_WALL_CORNER_BLINKING, &asset_server),
        load(INNER_WALL, &asset_server),
        load(INNER_WALL_BLINKING, &asset_server),
        load(GHOST_WALL_CORNER, &asset_server),
        load(GHOST_WALL_CORNER_BLINKING, &asset_server),
        load(GHOST_WALL, &asset_server),
        load(GHOST_WALL_BLINKING, &asset_server),
    ]))
}

fn load<S: ToString>(key: S, asset_server: &AssetServer) -> (S, HandleUntyped) {
    let handle = asset_server.load_untyped(&key.to_string());
    (key, handle)
}

fn notify_when_all_assets_loaded(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssetHandles>,
    mut event_writer: EventWriter<EAllAssetsLoaded>
) {
    match asset_server.get_group_load_state(game_assets.id_iter()) {
        LoadState::Failed => panic!("some assets failed loading, abort"),
        LoadState::Loaded => event_writer.send(EAllAssetsLoaded),
        _ => ()
    }
}

/// Fired when all assets were successfully loaded
pub struct EAllAssetsLoaded;

/// Resource that holds handles for all assets used in the game.
pub struct GameAssetHandles {
    handles: HashMap<String, HandleUntyped>,
}

impl GameAssetHandles {
    pub fn from_handles<H, S>(handles: H) -> Self
        where H: IntoIterator<Item=(S, HandleUntyped)>, S: ToString {
        GameAssetHandles {
            handles: handles.into_iter().map(|(key, h)| (key.to_string(), h)).collect()
        }
    }

    pub fn get_handle<S: ToString, T: Asset>(&self, key: S) -> Handle<T> {
        self.handles.get(&key.to_string()).expect("the requested handle should be registered").clone().typed()
    }

    pub fn id_iter<'a>(&'a self) -> impl IntoIterator<Item=HandleId> + 'a {
        self.handles.values().map(|handle| handle.id)
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::HandleId;
    use bevy::prelude::*;
    use crate::game_asset_handles::GameAssetHandles;
    use crate::spritesheet::SpriteSheet;

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
    pub fn all_handle_ids_can_be_retrieved_as_an_iterator() {
        let handle_ids = [
            HandleId::random::<Image>(),
            HandleId::random::<Font>(),
            HandleId::random::<SpriteSheet>(),
        ];

        let handles = [
            ("my_image", HandleUntyped::weak(handle_ids[0])),
            ("my_font", HandleUntyped::weak(handle_ids[1])),
            ("my_sheet", HandleUntyped::weak(handle_ids[2])),
        ];

        let assets = GameAssetHandles::from_handles(handles);

        let retrieved_ids = assets.id_iter().into_iter().collect::<Vec<_>>();

        assert_eq!(handle_ids.len(), retrieved_ids.len());
        retrieved_ids.into_iter().for_each(|id| {
            assert!(handle_ids.contains(&id))
        })
    }
}