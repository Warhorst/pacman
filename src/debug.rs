use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use crate::core::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[cfg(debug_assertions)]
    fn build(&self, app: &mut App) {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(
                Update,
                (
                    toggle_time,
                    despawn_all_edibles
                )
            )
        ;
    }

    #[cfg(not(debug_assertions))]
    fn build(&self, _app: &mut App) {

    }
}

#[cfg(debug_assertions)]
fn toggle_time(
    mut time: ResMut<Time<Virtual>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if time.relative_speed() == 1.0 {
            time.set_relative_speed(0.0)
        } else {
            time.set_relative_speed(1.0)
        }
    }
}

#[cfg(debug_assertions)]
fn despawn_all_edibles(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Edible>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        for e in &query {
            commands.entity(e).despawn();
        }
    }
}