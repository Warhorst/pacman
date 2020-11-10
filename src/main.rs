use bevy::prelude::*;

use map::MapPlugin;
use pacman::PacmanPlugin;

mod common;
mod pacman;
mod map;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        .add_plugin(MapPlugin)
        .add_plugin(PacmanPlugin)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
