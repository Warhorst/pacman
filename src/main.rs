use bevy::prelude::*;

use pacman::PacmanPlugin;

pub mod pacman;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(PacmanPlugin)
        .run()
}
