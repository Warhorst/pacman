use std::time::Duration;
use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::constants::FONT;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::score::Score;
use crate::game_state::in_game;

pub(in crate::ui) struct TopUIPlugin;

impl Plugin for TopUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Game(Start)), spawn_top_ui)
            .add_systems(Update, (
                update_scoreboard,
                blink_1_up_label
            ).run_if(in_game))
            .add_systems(OnExit(Game(GameOver)), despawn_top_ui)
        ;
    }
}

#[derive(Component)]
struct TopUI;

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct HighScoreBoard;

#[derive(Component)]
struct OneUpLabel;

fn spawn_top_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(FONT);

    commands.spawn((
        Name::new("TopUI"),
        TopUI,
        NodeBundle {
            style: Style {
                width: Percent(40.0),
                height: Percent(10.0),
                justify_content: JustifyContent::SpaceBetween,
                left: Percent(30.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        }
    ))
        .with_children(|parent| spawn_score_board(font.clone(), parent))
        .with_children(|parent| spawn_high_score_board(font.clone(), parent))
        .with_children(|parent| spawn_high_score_label(font.clone(), parent))
        .with_children(|parent| spawn_1up_label(font.clone(), parent))
    ;
}

fn spawn_score_board(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("ScoreBoard"),
        ScoreBoard,
        TextBundle::from_section(
            "0",
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Percent(50.0),
            ..default()
        })
    ));
}

fn spawn_high_score_board(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("HighScoreBoard"),
        HighScoreBoard,
        TextBundle::from_section(
            "0",
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            left: Percent(50.0),
            top: Percent(50.0),
            ..default()
        })
    ));
}

fn spawn_high_score_label(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("HighScoreBoardLabel"),
        TextBundle::from_section(
            "HIGH SCORE",
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Percent(10.0),
            left: Percent(30.0),
            ..default()
        })
    ));
}

/// Spawn the "1UP" in the top left of the screen.
///
/// To be honest, I didn't find out what this exactly does. It's not a Super Mario 1UP (aka extra live), but probably a pinball 1UP
/// (if 1UP is displayed then you see the score of player one; when he is done, it says 2UP and its player twos turn and score).
fn spawn_1up_label(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("1UPLabel"),
        OneUpLabel,
        TextBundle::from_section(
            "1UP",
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Percent(10.0),
            ..default()
        })
    ));
}

fn update_scoreboard(
    score: Res<Score>,
    mut query: Query<&mut Text, Or<(With<ScoreBoard>, With<HighScoreBoard>)>>,
) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", **score)
    }
}

#[derive(Deref, DerefMut)]
struct OneUpBlinkTimer(Timer);

impl Default for OneUpBlinkTimer {
    fn default() -> Self {
        OneUpBlinkTimer(Timer::new(Duration::from_secs_f32(0.2), TimerMode::Repeating))
    }
}

/// Let the "1UP" blink, like in the original arcade game.
fn blink_1_up_label(
    mut timer: Local<OneUpBlinkTimer>,
    time: Res<Time>,
    mut query: Query<&mut Visibility, With<OneUpLabel>>,
) {
    timer.tick(time.delta());

    let timer_finished = timer.just_finished();

    for mut vis in &mut query {
        if timer_finished {
            *vis = match *vis {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                _ => *vis
            };
        }
    }
}

fn despawn_top_ui(
    mut commands: Commands,
    query: Query<Entity, With<TopUI>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive()
    }
}