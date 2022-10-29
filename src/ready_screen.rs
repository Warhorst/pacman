use bevy::prelude::*;
use LifeCycle::Ready;
use crate::constants::FONT;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::life_cycle::LifeCycle;
use crate::map::FruitSpawn;

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
    fruit_spawn_query: Query<&FruitSpawn>,
) {
    let transform = Transform::from_translation(fruit_spawn_query.single().0);
    commands.spawn()
        .insert(Name::new("ReadyScreen"))
        .insert_bundle(Text2dBundle {
            text: Text::from_section(
                "Ready!".to_string(),
                TextStyle {
                    font: game_asset_handles.get_handle(FONT),
                    font_size: 20.0,
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