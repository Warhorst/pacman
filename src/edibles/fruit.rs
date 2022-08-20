use std::time::Duration;
use bevy::prelude::*;
use crate::level::Level;
use Fruit::*;
use crate::constants::{FRUIT_Z, PACMAN_DIMENSION};
use crate::edibles::dots::EatenDots;
use crate::edibles::Edible;
use crate::is;
use crate::life_cycle::LifeCycle::Running;
use crate::map::{Element, Map};
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
                    .with_system(reset_fruit_despawn_timer_when_level_changed)
            )
            .add_system_set(
                SystemSet::on_exit(Running).with_system(despawn_fruit_and_timer)
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

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>
) {
    let num_eaten_dots = eaten_dots.get_eaten();

    if num_eaten_dots == 70 || num_eaten_dots == 170 {
        let mut coordinates = map.coordinates_between_positions_matching(is!(Element::FruitSpawn));
        coordinates.z = FRUIT_Z;
        let dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
        let fruit = specs_per_level.get_for(&level).fruit_to_spawn;

        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: get_texture_for_fruit(&fruit, &asset_server),
                sprite: Sprite {
                    custom_size: Some(dimension),
                    ..default()
                },
                transform: Transform::from_translation(coordinates),
                ..Default::default()
            })
            .insert(fruit)
            .insert(Edible)
        ;
        commands.insert_resource(FruitDespawnTimer::new());
    }
}

fn get_texture_for_fruit(fruit: &Fruit, asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(&format!("textures/fruits/{}.png", match fruit {
        Cherry => "cherry",
        Strawberry => "strawberry",
        Peach => "peach",
        Apple => "cherry",
        Grapes => "grapes",
        Galaxian => "galaxian",
        Bell => "bell",
        Key => "key"
    }))
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
    query: Query<Entity, With<Fruit>>
) {
    commands.remove_resource::<FruitDespawnTimer>();

    for e in &query {
        commands.entity(e).despawn()
    }
}