use bevy::prelude::*;
use crate::game_assets::base_assets::{EAllBaseAssetsLoaded, start_base_assets_load, move_assets_to_the_game_asset_handles_when_loaded, notify_when_all_base_assets_loaded, BaseAssetsToLoad};
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::sprite_sheet::{create_sprite_sheets_when_texture_and_data_are_loaded, EAllSheetsLoaded, notify_when_all_sprite_sheets_loaded, SheetsToLoad, start_sprite_sheet_load};
use crate::life_cycle::LifeCycle::Loading;

pub mod handles;
mod sprite_sheet;
pub mod keys;
mod base_assets;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssetHandles>()
            .init_resource::<LoadState>()
            .add_event::<EAllBaseAssetsLoaded>()
            .add_event::<EAllSheetsLoaded>()
            .add_event::<EAllAssetsLoaded>()
            .add_system_set(
                SystemSet::on_enter(Loading)
                    .with_system(start_base_assets_load)
                    .with_system(start_sprite_sheet_load)
            )
            .add_system_set(
                SystemSet::on_update(Loading)
                    .with_system(move_assets_to_the_game_asset_handles_when_loaded)
                    .with_system(notify_when_all_base_assets_loaded)
                    .with_system(update_load_state_when_base_assets_loaded)
                    .with_system(notify_when_all_sprite_sheets_loaded)
                    .with_system(create_sprite_sheets_when_texture_and_data_are_loaded)
                    .with_system(update_load_state_when_sheets_loaded)
                    .with_system(send_event_when_everything_is_loaded)
            )
            .add_system_set(
                SystemSet::on_exit(Loading).with_system(remove_loader_resources)
            )
        ;
    }
}

fn update_load_state_when_base_assets_loaded(
    mut load_state: ResMut<LoadState>,
    mut event_reader: EventReader<EAllBaseAssetsLoaded>
) {
    for _ in event_reader.iter() {
        load_state.basic_assets_loaded = true
    }
}

fn update_load_state_when_sheets_loaded(
    mut load_state: ResMut<LoadState>,
    mut event_reader: EventReader<EAllSheetsLoaded>
) {
    for _ in event_reader.iter() {
        load_state.sheets_loaded = true
    }
}

fn send_event_when_everything_is_loaded(
    load_state: Res<LoadState>,
    mut event_writer: EventWriter<EAllAssetsLoaded>
) {
    if load_state.all_loaded() {
        event_writer.send(EAllAssetsLoaded)
    }
}

fn remove_loader_resources(
    mut commands: Commands,
) {
    commands.remove_resource::<BaseAssetsToLoad>();
    commands.remove_resource::<SheetsToLoad>();
}

/// Fired when all assets were successfully loaded
pub struct EAllAssetsLoaded;

#[derive(Default)]
struct LoadState {
    basic_assets_loaded: bool,
    sheets_loaded: bool
}

impl LoadState {
    fn all_loaded(&self) -> bool {
        self.basic_assets_loaded && self.sheets_loaded
    }
}