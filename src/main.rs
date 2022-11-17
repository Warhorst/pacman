use bevy::prelude::*;

use crate::music::MusicPlugin;
use crate::camera::CameraPlugin;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug::DebugPlugin;
use crate::game::GamePlugin;
use crate::game_assets::GameAssetsPlugin;
use crate::game_state::GameStatePlugin;

use crate::ui::UIPlugin;

mod camera;
mod constants;
mod game_assets;
mod music;
mod debug;
mod ui;
mod game;
mod game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                    title: "PacMan".to_string(),
                    resizable: false,
                    ..Default::default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(GamePlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MusicPlugin)
        .run()
}
