use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

use crate::pacman::PacmanPlugin;
use crate::animation::AnimationPlugin;
use crate::background_noise::BackgroundNoisePlugin;
use crate::camera::CameraPlugin;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug::DebugPlugin;
use crate::edibles::EdiblePlugin;
use crate::game_assets::GameAssetsPlugin;

use crate::game_over_screen::GameOverScreenPlugin;
use crate::life_cycle::GameStatePlugin;
use crate::ghost_house_gate::GhostHouseGatePlugin;
use crate::ghosts::GhostPlugin;
use crate::interactions::InteractionsPlugin;
use crate::level::LevelPlugin;
use crate::lives::LivesPlugin;
use crate::map::MapPlugin;
use crate::random::RandomPlugin;
use crate::ready_screen::ReadyScreenPlugin;
use crate::score::ScorePlugin;
use crate::specs_per_level::SpecsPerLevelPlugin;
use crate::speed::SpeedPlugin;
use crate::sprite_sheet::SpriteSheetPlugin;
use crate::ui::UIPlugin;

mod camera;
mod constants;
mod common;
mod pacman;
mod score;
mod ghosts;
mod random;
mod lives;
mod level;
mod speed;
mod map;
mod ghost_house_gate;
mod animation;
mod sprite_sheet;
mod life_cycle;
mod ready_screen;
mod game_over_screen;
mod edibles;
mod interactions;
mod game_assets;
mod specs_per_level;
mod background_noise;
mod debug;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "PacMan".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ImageSettings::default_nearest())
        .add_plugin(GameStatePlugin)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(EdiblePlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(RandomPlugin)
        .add_plugin(LivesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(InteractionsPlugin)
        .add_plugin(GhostHouseGatePlugin)
        .add_plugin(SpriteSheetPlugin)
        .add_plugin(ReadyScreenPlugin)
        .add_plugin(GameOverScreenPlugin)
        .add_plugin(SpecsPerLevelPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(BackgroundNoisePlugin)
        .run()
}
