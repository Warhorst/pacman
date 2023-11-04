use std::env;
use bevy::prelude::*;

use crate::music::MusicPlugin;
use crate::camera::CameraPlugin;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::game_assets::GameAssetsPlugin;
use crate::game_state::GameStatePlugin;
use crate::map_creator::MapCreator;
use crate::sound_effect::SoundEffectPlugin;
use crate::system_sets::SystemSetsPlugin;

use crate::ui::UIPlugin;

mod camera;
mod constants;
mod game_assets;
mod music;
mod sound_effect;
mod debug;
mod ui;
mod game;
mod game_state;
pub mod system_sets;
mod map_creator;

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
            GameStatePlugin,
            SystemSetsPlugin,
            GameAssetsPlugin,
            CameraPlugin,
            DebugPlugin,
            UIPlugin,
            MusicPlugin,
            SoundEffectPlugin
        ));


    let args: Vec<String> = env::args().collect();

    if args.contains(&"create_map".to_string()) {
        let mut map_creator = MapCreator::new(app);
        map_creator.create_map();
        map_creator.store_as_scene();
    } else {
        app.run()
    }
}
