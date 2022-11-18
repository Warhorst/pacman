use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::game_state::GameState::Ready;
use crate::constants::FONT;
use crate::game_assets::loaded_assets::LoadedAssets;

pub (in crate::ui) struct ReadyScreenPlugin;

impl Plugin for ReadyScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_screen)
            )
            .add_system_set(
                SystemSet::on_exit(Ready).with_system(despawn_screen)
            )
        ;
    }
}

#[derive(Component)]
struct ReadyScreen;

fn spawn_screen(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
) {
    commands.spawn((
        Name::new("ReadyScreen"),
        ReadyScreen,
        TextBundle::from_section(
            "Ready!",
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 0.0),
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Percent(45.0),
                top: Percent(55.0),
                ..default()
            },
            ..default()
        }),
    ));
}

fn despawn_screen(
    mut commands: Commands,
    query: Query<Entity, With<ReadyScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}