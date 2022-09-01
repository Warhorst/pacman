use bevy::diagnostic::{Diagnostics, DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::constants::WINDOW_HEIGHT;
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::FONT;
use crate::life_cycle::LifeCycle::Loading;

const FPS_COUNTER: &'static str = "fps_counter";

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system_set(
                SystemSet::on_exit(Loading)
                    .with_system(spawn_fps_counter)
            )
            .add_system(update_fps_counter)
            .add_system(toggle_debug_ui_visibility)
        ;
    }
}

fn spawn_fps_counter(
    mut commands: Commands,
    game_asset_handles: Res<GameAssetHandles>,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            String::new(),
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            }
        ),
        transform: Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT, 0.0)),
        visibility: Visibility {is_visible: false},
        ..Default::default()
    }).insert(DebugUI(FPS_COUNTER));
}

fn update_fps_counter(
    diagnostics_opt: Option<Res<Diagnostics>>,
    mut query: Query<(&mut Text, &DebugUI)>,
) {
    if let Some(diagnostics) = diagnostics_opt {
        let frame_count = match diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            Some(diag) => match diag.value() {
                Some(val) => val,
                None => return
            },
            None => return
        };

        for (mut text, ui) in &mut query {
            if **ui == FPS_COUNTER {
                text.sections[0].value = format!("{:.0}", frame_count)
            }
        }
    }
}

fn toggle_debug_ui_visibility(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<DebugUI>>
) {
    if !keyboard_input.just_pressed(KeyCode::B) {
        return;
    }

    for mut vis in &mut query {
        vis.is_visible = !vis.is_visible
    }
}

#[derive(Component, Deref)]
struct DebugUI(&'static str);