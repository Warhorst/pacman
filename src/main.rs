use bevy::prelude::*;

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
