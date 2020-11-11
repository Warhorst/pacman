use bevy::prelude::*;
use bevy::render::pass::ClearColor;

use interactions::InteractionsPlugin;
use map::MapPlugin;
use pacman::PacmanPlugin;

use crate::points::PointPlugin;

mod common;
mod pacman;
mod map;
mod points;
mod interactions;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 1000,
            height: 700,
            title: "PacMan".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        .add_plugin(MapPlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(PointPlugin)
        .add_plugin(InteractionsPlugin)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
