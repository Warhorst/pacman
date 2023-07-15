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
            .add_systems(OnEnter(GameOver), spawn_screens)
            .add_systems(Update, send_restart_event_when_key_pressed.run_if(in_state(GameOver)))
            .add_systems(OnExit(GameOver), despawn_screens)
        ;
    }
}

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct RestartGameScreen;

/// Event this is sent when the player decides to restart
#[derive(Event)]
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
            left: Percent(42.5),
            top: Percent(55.0),
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
            left: Percent(37.5),
            top: Percent(96.0),
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
    query: Query<Entity, Or<(With<GameOverScreen>, With<RestartGameScreen>)>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}