use bevy::prelude::*;
use bevy_asset_preload::AssetPreloadPlugin;
use bevy_sprite_sheet::SpriteSheetPlugin;

use core::prelude::*;
use crate::camera::CameraPlugin;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::game_state::GameStatePlugin;

use crate::map_creator::create_map;
use crate::music::MusicPlugin;
use crate::sound_effect::SoundEffectPlugin;
use crate::spawn::SpawnPlugin;

use crate::ui::UIPlugin;

mod camera;
mod music;
mod sound_effect;
mod debug;
mod ui;
mod game;
mod game_state;
mod map_creator;
mod spawn;
mod core;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    title: "PacMan".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((
            CorePlugin,
            GamePlugin,
            SpawnPlugin,
            AssetPreloadPlugin::new(Setup(PreloadAssets), Setup(CreateSpriteSheets)),
            SpriteSheetPlugin::new(Setup(CreateSpriteSheets), Spawn(SpawnMapScene)),
            GameStatePlugin,
            SystemSetsPlugin,
            CameraPlugin,
            DebugPlugin,
            UIPlugin,
            MusicPlugin,
            SoundEffectPlugin
        ))
    ;

    if should_create_map() {
        create_map(&mut app);
    } else {
        app.run();
    }
}

fn should_create_map() -> bool {
    std::env::args().into_iter().any(|arg| arg.contains("create_map"))
}
