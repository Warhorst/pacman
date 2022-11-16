use bevy::prelude::*;
use crate::ui::bottom::BottomUIPlugin;
use crate::ui::game_over_screen::GameOverScreenPlugin;
use crate::ui::ready_screen::ReadyScreenPlugin;

use crate::ui::top::TopUIPlugin;

mod top;
mod bottom;
pub mod game_over_screen;
pub mod ready_screen;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TopUIPlugin)
            .add_plugin(BottomUIPlugin)
            .add_plugin(ReadyScreenPlugin)
            .add_plugin(GameOverScreenPlugin)
        ;
    }
}

