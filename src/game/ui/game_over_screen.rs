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
        Node {
            position_type: PositionType::Absolute,
            left: Percent(42.5),
            top: Percent(55.0),
            ..default()
        },
        Text::new("GAME OVER"),
        TextFont {
            font: asset_server.load(FONT),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("RestartGameScreen"),
        RestartGameScreen,
        Node {
            position_type: PositionType::Absolute,
            left: Percent(37.5),
            top: Percent(96.0),
            ..default()
        },
        Text::new("Press R to restart"),
        TextFont {
            font: asset_server.load(FONT),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
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