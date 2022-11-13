use bevy::prelude::*;
use crate::ui::bottom::BottomUIPlugin;

use crate::ui::score::ScoreUIPlugin;

mod score;
mod bottom;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ScoreUIPlugin)
            .add_plugin(BottomUIPlugin)
        ;
    }
}

