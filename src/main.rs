use bevy::prelude::*;
use bevy_asset_preload::{AssetPreloadPlugin, load_assets};
use bevy_sprite_sheet::SpriteSheetPlugin;

use core::prelude::*;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;

use crate::map_creator::create_map;
use crate::spawn::SpawnPlugin;

mod debug;
mod game;
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
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins((
            CorePlugin,
            GamePlugin,
            SpawnPlugin,
            AssetPreloadPlugin::load_given_paths(Setup(PreloadAssets), Setup(CreateSpriteSheets), load_assets!()),
            SpriteSheetPlugin::new(Setup(CreateSpriteSheets), Spawn(SpawnMapScene)),
            DebugPlugin,
        ))
    ;

    if should_create_map() {
        create_map(&mut app);
    } else {
        app.run();
    }
}

fn should_create_map() -> bool {
    std::env::args().any(|arg| arg.contains("create_map"))
}
