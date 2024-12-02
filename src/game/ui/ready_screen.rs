use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use crate::core::prelude::*;

pub(super) struct ReadyScreenPlugin;

impl Plugin for ReadyScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Ready)),
                spawn_screen
            )
            .add_systems(
                OnExit(Game(Ready)),
                despawn_screen
            )
        ;
    }
}

/// Identifies the yellow text at the start of the game which just states "Ready". Hypes the player up for the game.
#[derive(Component)]
struct ReadyScreen;

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("ReadyScreen"),
        ReadyScreen,
        Node {
            position_type: PositionType::Absolute,
            left: Percent(45.0),
            top: Percent(55.0),
            ..default()
        },
        Text::new("Ready!"),
        TextFont {
            font: asset_server.load(FONT),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
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