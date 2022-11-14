use bevy::prelude::*;
use crate::ui::bottom::BottomUIPlugin;

use crate::ui::top::TopUIPlugin;

mod top;
mod bottom;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TopUIPlugin)
            .add_plugin(BottomUIPlugin)
        ;
    }
}

