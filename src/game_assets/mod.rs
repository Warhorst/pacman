use bevy::prelude::*;

use crate::game_assets::animation::AnimationPlugin;

// pub mod loaded_assets;
pub mod animation;
// pub mod sprite_sheet;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AnimationPlugin,
                // SpriteSheetPlugin
            ))
            // .add_event::<EAllAssetsLoaded>()
            // .add_systems(OnEnter(Loading), start_asset_load)
            // .add_systems(Update, create_sprite_sheets_and_send_event_when_all_loaded.run_if(in_state(Loading)))
        ;
    }
}

// fn start_asset_load(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.insert_resource(LoadedAssets::start_load(&asset_server));
// }
//
// fn create_sprite_sheets_and_send_event_when_all_loaded(
//     asset_server: Res<AssetServer>,
//     mut loaded_assets: ResMut<LoadedAssets>,
//     mut images: ResMut<Assets<Image>>,
//     mut sheet_data: ResMut<Assets<AsepriteData>>,
//     mut sprite_sheets: ResMut<Assets<SpriteSheet>>,
//     mut event_writer: EventWriter<EAllAssetsLoaded>,
// ) {
//     if loaded_assets.all_loaded(&asset_server) {
//         loaded_assets.add_sprite_sheets(&mut sprite_sheets, &mut images, &mut sheet_data);
//         event_writer.send(EAllAssetsLoaded)
//     }
// }
//
// /// Fired when all assets were successfully loaded
// #[derive(Event)]
// pub struct EAllAssetsLoaded;
