use std::time::Duration;
use bevy::prelude::*;
use crate::level::Level;
use Fruit::*;
use crate::constants::{FIELD_DIMENSION, FRUIT_DIMENSION, FRUIT_Z};
use crate::edibles::dots::EatenDots;
use crate::edibles::Edible;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::interactions::{EDotEaten, EFruitEaten};
use crate::life_cycle::LifeCycle::{LevelTransition, PacmanHit, Ready, Running};
use crate::map::{FruitSpawn, Map};
use crate::specs_per_level::SpecsPerLevel;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(spawn_fruit_when_dot_limit_reached)
                    .with_system(update_despawn_timer)
                    .with_system(despawn_fruit_if_timer_exceeded)
                    .with_system(play_fruit_eaten_sound_when_fruit_was_eaten)
                    .with_system(reset_fruit_despawn_timer_when_level_changed)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanHit).with_system(despawn_fruit_and_timer)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(despawn_fruit_and_timer)
            )
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(spawn_fruits_to_display)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(despawn_displayed_fruits)
            )
        ;
    }
}

#[derive(Copy, Clone, Component, Debug, Default, Eq, PartialEq)]
pub enum Fruit {
    #[default]
    Cherry,
    Strawberry,
    Peach,
    Apple,
    Grapes,
    Galaxian,
    Bell,
    Key,
}

#[derive(Deref, DerefMut)]
pub struct FruitDespawnTimer(Timer);

impl FruitDespawnTimer {
    fn new() -> Self {
        FruitDespawnTimer(Timer::new(Duration::from_secs_f32(9.5), false))
    }
}

#[derive(Component)]
struct DisplayedFruit;

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>,
    mut event_reader: EventReader<EDotEaten>,
    spawn_query: Query<&FruitSpawn>,
) {
    let num_eaten_dots = eaten_dots.get_eaten();

    for _ in event_reader.iter() {
        if let 70 | 170 = num_eaten_dots {
            for spawn in &spawn_query {
                let fruit = specs_per_level.get_for(&level).fruit_to_spawn;
                commands.spawn()
                    .insert(Name::new("Fruit"))
                    .insert_bundle(SpriteBundle {
                        texture: get_texture_for_fruit(&fruit, &loaded_assets),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(FRUIT_DIMENSION)),
                            ..default()
                        },
                        transform: Transform::from_translation(**spawn),
                        ..Default::default()
                    })
                    .insert(fruit)
                    .insert(Edible);
            }
            commands.insert_resource(FruitDespawnTimer::new());
        }
    }
}

/// Update the despawn timer with delta time.
fn update_despawn_timer(
    time: Res<Time>,
    mut despawn_timer_opt: Option<ResMut<FruitDespawnTimer>>,
) {
    if let Some(ref mut despawn_timer) = despawn_timer_opt {
        despawn_timer.tick(time.delta());
    }
}

/// When the fruit despawn timer exceeds, remove the fruit and the timer resource.
fn despawn_fruit_if_timer_exceeded(
    mut commands: Commands,
    despawn_timer_opt: Option<Res<FruitDespawnTimer>>,
    query: Query<Entity, With<Fruit>>,
) {
    if let Some(ref despawn_timer) = despawn_timer_opt {
        if despawn_timer.finished() {
            for entity in query.iter() {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>()
            }
        }
    }
}

/// If the level changed, remove the timer and reset the dot counter.
fn reset_fruit_despawn_timer_when_level_changed(
    mut commands: Commands,
    level: Res<Level>,
) {
    if level.is_changed() {
        commands.remove_resource::<FruitDespawnTimer>();
    }
}

fn despawn_fruit_and_timer(
    mut commands: Commands,
    query: Query<Entity, With<Fruit>>,
) {
    commands.remove_resource::<FruitDespawnTimer>();

    for e in &query {
        commands.entity(e).despawn()
    }
}

fn play_fruit_eaten_sound_when_fruit_was_eaten(
    loaded_assets: Res<LoadedAssets>,
    audio: Res<Audio>,
    mut event_reader: EventReader<EFruitEaten>,
) {
    for _ in event_reader.iter() {
        audio.play(loaded_assets.get_handle("sounds/fruit_eaten.ogg"));
    }
}

fn spawn_fruits_to_display(
    mut commands: Commands,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    loaded_assets: Res<LoadedAssets>,
    map_query: Query<&Map>
) {
    let map = map_query.single();
    let fruits_to_display = get_fruits_to_display(&level, &specs_per_level);
    let len = fruits_to_display.len();

    for (i, fruit) in fruits_to_display.into_iter().enumerate() {
        let transform = Transform::from_translation(Vec3::new(
            map.width as f32 * FIELD_DIMENSION - (len - i) as f32 * FRUIT_DIMENSION,
            -FRUIT_DIMENSION,
            FRUIT_Z,
        ));

        commands.spawn().insert_bundle(SpriteBundle {
            texture: get_texture_for_fruit(&fruit, &loaded_assets),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(FRUIT_DIMENSION)),
                ..default()
            },
            transform,
            ..Default::default()
        }).insert(DisplayedFruit);
    }
}

fn get_fruits_to_display(
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) -> Vec<Fruit> {
    let border = level.checked_sub(7).unwrap_or(1).max(1);

    (border..=**level).rev()
        .into_iter()
        .map(|i| specs_per_level.get_for(&Level(i)).fruit_to_spawn)
        .collect()
}

fn despawn_displayed_fruits(
    mut commands: Commands,
    query: Query<Entity, With<DisplayedFruit>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn get_texture_for_fruit(fruit: &Fruit, asset_handles: &LoadedAssets) -> Handle<Image> {
    asset_handles.get_handle(&format!("textures/fruits/{}.png", match fruit {
        Cherry => "cherry",
        Strawberry => "strawberry",
        Peach => "peach",
        Apple => "apple",
        Grapes => "grapes",
        Galaxian => "galaxian",
        Bell => "bell",
        Key => "key"
    }))
}