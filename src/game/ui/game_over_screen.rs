use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::core::prelude::*;

pub(super) struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(GameOver)),
                spawn_screens
            )
            .add_systems(
                OnExit(Game(GameOver)),
                despawn_screens
            )
        ;
    }
}

/// Shows a big red "GAME OVER"
#[derive(Component)]
struct GameOverScreen;

/// Shows a prompt which indicates you can restart the game
#[derive(Component)]
struct RestartGameScreen;

fn spawn_screens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("GameOverScreen"),
        GameOverScreen,
        TextBundle::from_section(
            "GAME OVER",
            TextStyle {
                font: asset_server.load(FONT),
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
                font: asset_server.load(FONT),
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

fn despawn_screens(
    mut commands: Commands,
    query: Query<Entity, Or<(With<GameOverScreen>, With<RestartGameScreen>)>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}