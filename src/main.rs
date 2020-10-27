use bevy::prelude::*;

use pacman::PacmanPlugin;

mod pacman;
mod board;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(PacmanPlugin)
        .run()
}
