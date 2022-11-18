use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::constants::FONT;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game_state::GameState::GameOver;

pub(in crate::ui) struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EGameRestarted>()
            .add_system_set(
                SystemSet::on_enter(GameOver).with_system(spawn_screens)
            )
            .add_system_set(
                SystemSet::on_update(GameOver).with_system(send_restart_event_when_key_pressed)
            )
            .add_system_set(
                SystemSet::on_exit(GameOver).with_system(despawn_screens)
            )
        ;
    }
}

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct RestartGameScreen;

/// Event this is sent when the player decides to restart
pub struct EGameRestarted;

fn spawn_screens(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
) {
    commands.spawn((
        Name::new("GameOverScreen"),
        GameOverScreen,
        TextBundle::from_section(
            "GAME OVER",
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 20.0,
                color: Color::rgb(1.0, 0.0, 0.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Percent(42.5),
                top: Percent(55.0),
                ..default()
            },
            ..default()
        }),
    ));

    commands.spawn((
        Name::new("RestartGameScreen"),
        RestartGameScreen,
        TextBundle::from_section(
            "Press R to restart",
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 20.0,
                color: Color::rgb(1.0, 0.0, 0.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Percent(37.5),
                top: Percent(96.0),
                ..default()
            },
            ..default()
        }),
    ));
}

fn send_restart_event_when_key_pressed(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<EGameRestarted>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        event_writer.send(EGameRestarted);
    }
}

fn despawn_screens(
    mut commands: Commands,
    query: Query<Entity, Or<(With<GameOverScreen>, With<RestartGameScreen>)>>
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}