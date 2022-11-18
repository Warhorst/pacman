use std::time::Duration;
use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::constants::FONT;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game_state::GameState::{GameOver, InGame, Start};
use crate::game::score::Score;

pub(in crate::ui) struct TopUIPlugin;

impl Plugin for TopUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_top_ui)
            )
            .add_system_set(
                SystemSet::on_inactive_update(InGame)
                    .with_system(update_scoreboard)
                    .with_system(blink_1_up_label)
            )
            .add_system_set(
                SystemSet::on_exit(GameOver).with_system(despawn_top_ui)
            )
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
    loaded_assets: Res<LoadedAssets>,
) {
    let font = loaded_assets.get_handle(FONT);

    commands.spawn((
        Name::new("TopUI"),
        TopUI,
        NodeBundle {
            style: Style {
                size: Size::new(Percent(40.0), Percent(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                position: UiRect {
                    left: Percent(30.0),
                    ..default()
                },
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
            position: UiRect {
                top: Percent(50.0),
                ..default()
            },
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
            position: UiRect {
                left: Percent(50.0),
                top: Percent(50.0),
                ..default()
            },
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
            position: UiRect {
                top: Percent(10.0),
                left: Percent(30.0),
                ..default()
            },
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
            position: UiRect {
                top: Percent(10.0),
                ..default()
            },
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
            vis.is_visible = !vis.is_visible;
        }
    }
}

fn despawn_top_ui(
    mut commands: Commands,
    query: Query<Entity, With<TopUI>>
) {
    for e in &query {
        commands.entity(e).despawn_recursive()
    }
}