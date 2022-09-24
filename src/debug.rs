use bevy::diagnostic::{Diagnostics, DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::ecs::query::{QuerySingleError, ROQueryItem};
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use crate::board_dimensions::BoardDimensions;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::edibles::dots::EatenDots;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::level::Level;
use crate::life_cycle::LifeCycle::Loading;
use crate::common::Direction;
use crate::edibles::Edible;
use crate::edibles::energizer::EnergizerTimer;
use crate::edibles::fruit::FruitDespawnTimer;
use crate::ghosts::{Blinky, Clyde, Inky, Pinky};
use crate::pacman::Pacman;
use crate::ghosts::state::State;
use crate::life_cycle::LifeCycle;

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
const PACMAN_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
const BLINKY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const PINKY_COLOR: Color = Color::rgb(1.0, 156.0 / 255.0, 206.0 / 255.0);
const INKY_COLOR: Color = Color::rgb(49.0 / 255.0, 1.0, 1.0);
const CLYDE_COLOR: Color = Color::rgb(1.0, 206.0 / 255.0, 49.0 / 255.0);

const UI_HEIGHT: f32 = 15.0;
const FPS_COUNTER: &'static str = "fps_counter";
const LIFE_CYCLE: &'static str = "life_cycle";
const DOTS_EATEN: &'static str = "dots_eaten";
const LEVEL: &'static str = "level";
const PACMAN: &'static str = "pacman";
const BLINKY: &'static str = "blinky";
const PINKY: &'static str = "pinky";
const INKY: &'static str = "inky";
const CLYDE: &'static str = "clyde";
const ENERGIZER_TIMER: &'static str = "energizer_timer";
const FRUIT_DESPAWN_TIMER: &'static str = "fruit_despawn_timer";

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system_set(
                SystemSet::on_exit(Loading).with_system(spawn_debug_uis)
            )
            .add_system(update_fps_counter)
            .add_system(update_lifecycle_ui)
            .add_system(update_dots_eaten_remaining)
            .add_system(update_level_ui)
            .add_system(update_pacman_ui)
            .add_system(update_blinky_ui)
            .add_system(update_pinky_ui)
            .add_system(update_inky_ui)
            .add_system(update_clyde_ui)
            .add_system(update_energizer_timer_ui)
            .add_system(update_fruit_despawn_timer_ui)
            .add_system(toggle_debug_ui_visibility)
            .add_system(despawn_all_edibles_on_key_press)
        ;
    }
}

fn spawn_debug_uis(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
) {
    spawn_uis(
        [
            (FPS_COUNTER, WHITE),
            (LIFE_CYCLE, WHITE),
            (DOTS_EATEN, WHITE),
            (LEVEL, WHITE),
            (PACMAN, PACMAN_COLOR),
            (BLINKY, BLINKY_COLOR),
            (PINKY, PINKY_COLOR),
            (INKY, INKY_COLOR),
            (CLYDE, CLYDE_COLOR),
            (ENERGIZER_TIMER, WHITE),
            (FRUIT_DESPAWN_TIMER, WHITE)
        ],
    &mut commands, &game_asset_handles)
}

fn spawn_uis(
    names_colors: impl IntoIterator<Item=(&'static str, Color)>,
    commands: &mut Commands,
    game_asset_handles: &LoadedAssets,
) {
    let font = game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf");
    names_colors
        .into_iter()
        .enumerate()
        .for_each(|(i, (name, color))| spawn_ui(commands, font.clone(), name, WINDOW_HEIGHT - UI_HEIGHT * (i as f32), color))
}

fn spawn_ui(commands: &mut Commands, font: Handle<Font>, name: &'static str, y: f32, color: Color) {
    let size = Vec2::new(WINDOW_WIDTH, UI_HEIGHT);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            String::new(),
            TextStyle {
                font,
                font_size: 20.0,
                color,
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            }
        ),
        transform: Transform::from_translation(Vec3::new(0.0, y, 0.0)),
        visibility: Visibility { is_visible: false },
        text_2d_bounds: Text2dBounds { size },
        ..Default::default()
    }).insert(DebugUI(name));
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
                text.sections[0].value = format!("FPS: {:.0}", frame_count)
            }
        }
    }
}

fn update_lifecycle_ui(
    life_cycle: Res<bevy::prelude::State<LifeCycle>>,
    mut query: Query<(&mut Text, &DebugUI)>,
) {
    if !life_cycle.is_changed() {
        return;
    }

    for (mut text, ui) in &mut query {
        if **ui == LIFE_CYCLE {
            text.sections[0].value = format!("LifeCycle: {:?}", life_cycle.current())
        }
    }
}

fn update_dots_eaten_remaining(
    eaten_dots: Option<Res<EatenDots>>,
    mut query: Query<(&mut Text, &DebugUI)>,
) {
    for (mut text, ui) in &mut query {
        if **ui == DOTS_EATEN {
            let value = match &eaten_dots {
                Some(eaten_dots) => format!("{} / {}", eaten_dots.get_eaten(), eaten_dots.get_max()),
                None => String::new()
            };

            text.sections[0].value = format!("Dots: {}", value)
        }
    }
}

fn update_level_ui(
    level: Option<Res<Level>>,
    mut query: Query<(&mut Text, &DebugUI)>,
) {
    for (mut text, ui) in &mut query {
        if **ui == LEVEL {
            let value = match &level {
                Some(level) => format!("{}", ***level),
                None => "/".to_string()
            };

            text.sections[0].value = format!("Level: {}", value)
        }
    }
}

fn update_pacman_ui(
    dimensions: Option<Res<BoardDimensions>>,
    pacman_query: Query<(&Transform, &Direction), With<Pacman>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    let dimensions = match dimensions {
        Some(d) => d,
        None => return
    };
    let comps = pacman_query.get_single();

    for (mut text, ui) in &mut ui_query {
        if **ui == PACMAN {
            text.sections[0].value = match comps {
                Ok((transform, direction)) => {
                    let coordinates = transform.translation;
                    let position = dimensions.vec_to_pos(&coordinates);
                    format!("{}, {}, {}", format_coordinates(coordinates), position , direction)
                },
                _ => format!("-")
            };
        }
    }
}

fn update_blinky_ui(
    dimensions: Option<Res<BoardDimensions>>,
    blinky_query: Query<(&Transform, &Direction, &State), With<Blinky>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    let dimensions = match dimensions {
        Some(d) => d,
        None => return
    };
    let comps = blinky_query.get_single();

    for (mut text, ui) in &mut ui_query {
        if **ui == BLINKY {
            text.sections[0].value = create_ghost_debug_text(&comps, &dimensions);
        }
    }
}

fn update_pinky_ui(
    dimensions: Option<Res<BoardDimensions>>,
    pinky_query: Query<(&Transform, &Direction, &State), With<Pinky>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    let dimensions = match dimensions {
        Some(d) => d,
        None => return
    };
    let comps = pinky_query.get_single();

    for (mut text, ui) in &mut ui_query {
        if **ui == PINKY {
            text.sections[0].value = create_ghost_debug_text(&comps, &dimensions);
        }
    }
}

fn update_inky_ui(
    dimensions: Option<Res<BoardDimensions>>,
    inky_query: Query<(&Transform, &Direction, &State), With<Inky>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    let dimensions = match dimensions {
        Some(d) => d,
        None => return
    };
    let comps = inky_query.get_single();

    for (mut text, ui) in &mut ui_query {
        if **ui == INKY {
            text.sections[0].value = create_ghost_debug_text(&comps, &dimensions);
        }
    }
}

fn update_clyde_ui(
    dimensions: Option<Res<BoardDimensions>>,
    clyde_query: Query<(&Transform, &Direction, &State), With<Clyde>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    let dimensions = match dimensions {
        Some(d) => d,
        None => return
    };
    let comps = clyde_query.get_single();

    for (mut text, ui) in &mut ui_query {
        if **ui == CLYDE {
            text.sections[0].value = create_ghost_debug_text(&comps, &dimensions);
        }
    }
}

fn create_ghost_debug_text(comps: &Result<ROQueryItem<'_, (&Transform, &Direction, &State)>, QuerySingleError>, dimensions: &BoardDimensions) -> String {
    match comps {
        Ok((transform, direction, state)) => {
            let coordinates = transform.translation;
            let position = dimensions.vec_to_pos(&coordinates);
            format!("{}, {}, {}, {}", format_coordinates(coordinates), position , direction, state)
        },
        _ => format!("-")
    }
}

fn format_coordinates(coordinates: Vec3) -> String {
    format!("({:.2}, {:.2}, {:.2})", coordinates.x, coordinates.y, coordinates.z)
}

fn update_energizer_timer_ui(
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    for (mut text, ui) in &mut ui_query {
        if **ui == ENERGIZER_TIMER {
            text.sections[0].value = format!("EnergizerTimer: {}", match energizer_timer {
                Some(ref timer) => timer.remaining().to_string(),
                None => "-".to_string()
            })
        }
    }
}

fn update_fruit_despawn_timer_ui(
    fruit_despawn_timer: Option<Res<FruitDespawnTimer>>,
    mut ui_query: Query<(&mut Text, &DebugUI)>,
) {
    for (mut text, ui) in &mut ui_query {
        if **ui == FRUIT_DESPAWN_TIMER {
            text.sections[0].value = format!("FruitTimer: {}", match fruit_despawn_timer {
                Some(ref timer) => (timer.duration().as_secs_f32() - timer.elapsed_secs()).to_string(),
                None => "-".to_string()
            })
        }
    }
}

fn toggle_debug_ui_visibility(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<DebugUI>>,
) {
    if !keyboard_input.just_pressed(KeyCode::B) {
        return;
    }

    for mut vis in &mut query {
        vis.is_visible = !vis.is_visible
    }
}

/// Despawn all dots when '1' was pressed.
fn despawn_all_edibles_on_key_press(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Edible>>
) {
    if !keyboard_input.just_pressed(KeyCode::Key1) {
        return;
    }

    for e in &query {
        commands.entity(e).despawn();
    }
}

#[derive(Component, Deref)]
struct DebugUI(&'static str);