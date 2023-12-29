use bevy::prelude::*;
use bevy_asset_preload::AssetPreloadPlugin;
use bevy_sprite_sheet::SpriteSheetPlugin;

use crate::prelude::*;
use crate::animation::AnimationPlugin;

use crate::map_creator::create_map;

use crate::ui::UIPlugin;

mod camera;
mod constants;
mod animation;
mod music;
mod sound_effect;
mod debug;
mod ui;
mod game;
mod game_state;
pub mod system_sets;
mod map_creator;
mod prelude;

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
            GamePlugin,
            AssetPreloadPlugin::new(Setup(PreloadAssets), Setup(CreateSpriteSheets)),
            SpriteSheetPlugin::new(Setup(CreateSpriteSheets), Game(Start)),
            GameStatePlugin,
            SystemSetsPlugin,
            AnimationPlugin,
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
