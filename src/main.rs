use bevy::prelude::*;
use bevy::render::pass::ClearColor;

use interactions::InteractionsPlugin;
use map::MapPlugin;
use pacman::PacmanPlugin;

use crate::dots::DotPlugin;
use crate::events::EventPlugin;
use crate::ghosts::GhostPlugin;
use crate::score::ScorePlugin;

mod constants;
mod common;
mod pacman;
mod map;
mod dots;
mod interactions;
mod score;
mod ghosts;
mod events;

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
        .add_plugin(MapPlugin)
        .add_plugin(PacmanPlugin)
        .add_plugin(DotPlugin)
        .add_plugin(InteractionsPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(EventPlugin)
        .add_startup_system(init.system())
        .run()
}

fn init(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}
