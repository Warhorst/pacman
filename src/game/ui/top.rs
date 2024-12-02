use std::time::Duration;
use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::core::prelude::*;

pub(super) struct TopUIPlugin;

impl Plugin for TopUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Start)),
                spawn_top_ui,
            )
            .add_systems(
                Update,
                (
                    update_scoreboard,
                    update_high_score_board,
                    blink_1_up_label
                ).run_if(in_game))
            .add_systems(
                OnExit(Game(GameOver)),
                despawn_top_ui,
            )
        ;
    }
}

/// Parent of all top UI elements
#[derive(Component)]
struct TopUI;

/// Shows the score of the current game
#[derive(Component)]
struct ScoreBoard;

/// Shows the current high score
#[derive(Component)]
struct HighScoreBoard;

/// Shows the "1UP" in the top left corner of the screen
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
        Node {
            width: Percent(40.0),
            height: Percent(10.0),
            justify_content: JustifyContent::SpaceBetween,
            left: Percent(30.0),
            position_type: PositionType::Absolute,
            ..default()
        },
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
        Node {
            position_type: PositionType::Absolute,
            top: Percent(50.0),
            ..default()
        },
        Text::new("0"),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    ));
}

fn spawn_high_score_board(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("HighScoreBoard"),
        HighScoreBoard,
        Node {
            position_type: PositionType::Absolute,
            left: Percent(50.0),
            top: Percent(50.0),
            ..default()
        },
        Text::new("0"),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    ));
}

fn spawn_high_score_label(
    font: Handle<Font>,
    parent: &mut ChildBuilder,
) {
    parent.spawn((
        Name::new("HighScoreBoardLabel"),
        Node {
            position_type: PositionType::Absolute,
            top: Percent(10.0),
            left: Percent(30.0),
            ..default()
        },
        Text::new("HIGH SCORE"),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
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
        Node {
            position_type: PositionType::Absolute,
            top: Percent(10.0),
            ..default()
        },
        Text::new("1UP"),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    ));
}

fn update_scoreboard(
    mut writer: TextUiWriter,
    score: Res<Score>,
    query: Query<Entity, With<ScoreBoard>>,
) {
    if !score.is_changed() {
        return;
    }

    for entity in query.iter() {
        *writer.text(entity, 0) = format!("{}", **score)
    }
}

fn update_high_score_board(
    mut writer: TextUiWriter,
    high_score: Res<HighScore>,
    query: Query<Entity, With<HighScoreBoard>>,
) {
    if !high_score.is_changed() {
        return;
    }

    for entity in query.iter() {
        *writer.text(entity, 0) = format!("{}", high_score.score)
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