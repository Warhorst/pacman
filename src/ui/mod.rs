use bevy::prelude::*;

use crate::ui::score::ScoreUIPlugin;

mod score;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ScoreUIPlugin);
    }
}

