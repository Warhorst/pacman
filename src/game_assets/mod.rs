use bevy::prelude::*;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::life_cycle::LifeCycle::Loading;
use crate::sprite_sheet::aseprite_data::AsepriteData;
use crate::sprite_sheet::SpriteSheet;

pub mod keys;
pub mod loaded_assets;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EAllAssetsLoaded>()
            .add_system_set(
                SystemSet::on_enter(Loading).with_system(start_asset_load)
            )
            .add_system_set(
                SystemSet::on_update(Loading).with_system(create_sprite_sheets_and_send_event_when_all_loaded)
            )
        ;
    }
}

fn start_asset_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(LoadedAssets::start_load(&asset_server));
}

fn create_sprite_sheets_and_send_event_when_all_loaded(
    asset_server: Res<AssetServer>,
    mut loaded_assets: ResMut<LoadedAssets>,
    mut images: ResMut<Assets<Image>>,
    sheet_data: Res<Assets<AsepriteData>>,
    mut sprite_sheets: ResMut<Assets<SpriteSheet>>,
    mut event_writer: EventWriter<EAllAssetsLoaded>,
) {
    if loaded_assets.all_loaded(&asset_server) {
        loaded_assets.add_sprite_sheets(&mut sprite_sheets, &mut images, &sheet_data);
        event_writer.send(EAllAssetsLoaded)
    }
}

/// Fired when all assets were successfully loaded
pub struct EAllAssetsLoaded;