use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, toggle_time)
        ;
    }
}

fn toggle_time(
    mut time: ResMut<Time<Virtual>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if time.relative_speed() == 1.0 {
            time.set_relative_speed(0.0)
        } else {
            time.set_relative_speed(1.0)
        }
    }
}