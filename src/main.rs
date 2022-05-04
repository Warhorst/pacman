extern crate core;

use bevy::prelude::*;

use map::MapPlugin;
use pacman::PacmanPlugin;

use crate::debug::DebugPlugin;
use crate::dots::DotPlugin;
use crate::energizer::EnergizerPlugin;
use crate::ghosts::GhostPlugin;
use crate::random::RandomPlugin;
use crate::score::ScorePlugin;
use crate::tunnels::TunnelPlugin;

mod constants;
mod common;
mod pacman;
mod map;
mod dots;
mod score;
mod ghosts;
mod tunnels;
mod energizer;
mod debug;
mod random;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(DotPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(TunnelPlugin)
        .add_plugin(EnergizerPlugin)
        .add_plugin(RandomPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(init)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
