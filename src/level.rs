use bevy::prelude::*;
use crate::constants::FIELD_DIMENSION;
use crate::game_assets::handles::GameAssetHandles;
use crate::game_assets::keys::FONT;
use crate::life_cycle::LifeCycle::{LevelTransition, Start};
use crate::map::board::Board;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level(1))
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_level_ui)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(increase_level)
            )
        ;
    }
}

#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq)]
pub struct Level(pub usize);

impl Level {
    fn increase(&mut self) {
        **self += 1
    }
}

#[derive(Component)]
pub struct LevelUi;

fn spawn_level_ui(
    mut commands: Commands,
    game_asset_handles: Res<GameAssetHandles>,
    level: Res<Level>,
    board: Res<Board>,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            format!("Level: {}", **level),
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 40.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        ),
        transform: Transform::from_xyz(FIELD_DIMENSION * (board.width as f32 / 2.0), FIELD_DIMENSION * board.height as f32, 0.0),
        ..Default::default()
    })
        .insert(LevelUi);
}

fn increase_level(
    mut level: ResMut<Level>,
    mut query: Query<&mut Text, With<LevelUi>>,
) {
    level.increase();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Level: {}", **level)
    }
}