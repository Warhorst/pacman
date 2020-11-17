use bevy::prelude::*;

const POINTS_PER_POINT: usize = 10;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Score::new())
            .add_startup_system(create_scoreboard.system())
            .add_system(update_scoreboard.system());
    }
}

pub struct Score {
    points: usize
}

impl Score {
    fn new() -> Self {
        Score {
            points: 0
        }
    }

    pub fn increment(&mut self) {
        self.points += POINTS_PER_POINT
    }

    pub fn get_points(&self) -> usize {
        self.points
    }
}

fn create_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextComponents {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            value: "Score:".to_string(),
            style: TextStyle {
                color: Color::rgb(1.0, 1.0, 1.0),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.value = format!("Score: {}", score.get_points())
    }
}
