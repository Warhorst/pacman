use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

use crate::pacman::PacmanPlugin;
use crate::animation::AnimationPlugin;
use crate::camera::CameraPlugin;
use crate::edibles::EdiblePlugin;
use crate::game_assets::GameAssetsPlugin;

use crate::game_over_screen::GameOverScreenPlugin;
use crate::life_cycle::GameStatePlugin;
use crate::ghost_corners::GhostCornersPlugin;
use crate::ghost_house::GhostHousePlugin;
use crate::ghost_house_gate::GhostHouseGatePlugin;
use crate::ghosts::GhostPlugin;
use crate::interactions::InteractionsPlugin;
use crate::level::LevelPlugin;
use crate::lives::LivesPlugin;
use crate::map::MapPlugin;
use crate::random::RandomPlugin;
use crate::ready_screen::ReadyScreenPlugin;
use crate::score::ScorePlugin;
use crate::speed::SpeedPlugin;
use crate::spritesheet::SpriteSheetPlugin;
use crate::tunnels::TunnelPlugin;
use crate::walls::WallsPlugin;

mod camera;
mod constants;
mod common;
mod pacman;
mod score;
mod ghosts;
mod tunnels;
mod random;
mod lives;
mod level;
mod speed;
mod map;
mod walls;
mod ghost_house;
mod ghost_corners;
mod ghost_house_gate;
mod animation;
mod helper;
mod spritesheet;
mod life_cycle;
mod ready_screen;
mod game_over_screen;
mod edibles;
mod interactions;
mod game_assets;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1200.0,
            height: 900.0,
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
        .add_plugin(WallsPlugin)
        .add_plugin(EdiblePlugin)
        .add_plugin(GhostHousePlugin)
        .add_plugin(GhostCornersPlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(TunnelPlugin)
        .add_plugin(RandomPlugin)
        .add_plugin(LivesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(InteractionsPlugin)
        .add_plugin(GhostHouseGatePlugin)
        .add_plugin(SpriteSheetPlugin)
        .add_plugin(ReadyScreenPlugin)
        .add_plugin(GameOverScreenPlugin)
        .run()
}
