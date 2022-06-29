extern crate core;

use bevy::prelude::*;

use pacman::PacmanPlugin;
use crate::camera::CameraPlugin;

use crate::dots::DotPlugin;
use crate::energizer::EnergizerPlugin;
use crate::ghost_corners::GhostCornersPlugin;
use crate::ghost_house::GhostHousePlugin;
use crate::ghost_house_gate::GhostHouseGatePlugin;
use crate::ghosts::GhostPlugin;
use crate::level::LevelPlugin;
use crate::lives::LivesPlugin;
use crate::map::MapPlugin;
use crate::random::RandomPlugin;
use crate::score::ScorePlugin;
use crate::speed::SpeedPlugin;
use crate::tunnels::TunnelPlugin;
use crate::walls::WallsPlugin;

mod camera;
mod constants;
mod common;
mod pacman;
mod dots;
mod score;
mod ghosts;
mod tunnels;
mod energizer;
mod random;
mod lives;
mod level;
mod speed;
mod map;
mod walls;
mod ghost_house;
mod ghost_corners;
mod ghost_house_gate;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.0,
            height: 700.0,
            title: "PacMan".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(GhostHousePlugin)
        .add_plugin(GhostCornersPlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(DotPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(TunnelPlugin)
        .add_plugin(EnergizerPlugin)
        .add_plugin(RandomPlugin)
        .add_plugin(LivesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(GhostHouseGatePlugin)
        .run()
}
