use bevy::prelude::*;
use LifeCycle::Ready;
use crate::board_dimensions::BoardDimensions;
use crate::constants::TEXT_Z;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::is;
use crate::life_cycle::LifeCycle;
use crate::map::Map;
use crate::map::Element;

pub struct ReadyScreenPlugin;

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
    dimensions: Res<BoardDimensions>,
    map: Res<Map>,
) {
    let transform = dimensions.positions_to_trans(map.get_positions_matching(is!(Element::FruitSpawn)), TEXT_Z);
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Ready!".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(1.0, 1.0, 0.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        ),
        transform,
        ..Default::default()
    })
        .insert(ReadyScreen);
}

fn despawn_screen(
    mut commands: Commands,
    query: Query<Entity, With<ReadyScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}