use std::time::Duration;
use bevy::prelude::*;
use crate::level::Level;
use Fruit::*;
use crate::constants::PACMAN_DIMENSION;
use crate::edibles::Edible;
use crate::interactions::EDotEaten;
use crate::life_cycle::LifeCycle;
use crate::is;
use crate::map::{Element, Map};

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FruitDotCounter::new())
            .add_system_set(
                SystemSet::on_update(LifeCycle::Running)
                    .with_system(spawn_fruit_when_dot_limit_reached)
                    .with_system(update_despawn_timer)
                    .with_system(increase_dot_counter_when_dot_was_eaten)
                    .with_system(despawn_fruit_if_timer_exceeded)
                    .with_system(reset_fruit_dot_counter_and_despawn_timer_when_level_changed)
            )
        ;
    }
}

#[derive(Component)]
pub enum Fruit {
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

#[derive(Deref, DerefMut)]
struct FruitDotCounter(usize);

impl FruitDotCounter {
    fn new() -> Self {
        FruitDotCounter(0)
    }

    fn increase(&mut self) {
        self.0 += 1
    }
}

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    level: Res<Level>,
    fruit_dot_counter: Res<FruitDotCounter>,
) {
    let eaten_dots = **fruit_dot_counter;

    if eaten_dots == 70 || eaten_dots == 170 {
        let coordinates = map.coordinates_between_positions_matching(is!(Element::FruitSpawn));
        let dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
        let fruit = get_fruit_for_level(&level);

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

fn get_fruit_for_level(level: &Level) -> Fruit {
    match **level {
        1 => Cherry,
        2 => Strawberry,
        3 | 4 => Peach,
        5 | 6 => Apple,
        7 | 8 => Grapes,
        9 | 10 => Galaxian,
        11 | 12 => Bell,
        _ => Key
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

/// When a dot was eaten, increase the dot counter.
fn increase_dot_counter_when_dot_was_eaten(
    mut event_reader: EventReader<EDotEaten>,
    mut fruit_dot_counter: ResMut<FruitDotCounter>
) {
    for _ in event_reader.iter() {
        fruit_dot_counter.increase()
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
fn reset_fruit_dot_counter_and_despawn_timer_when_level_changed(
    mut commands: Commands,
    level: Res<Level>,
    mut fruit_dot_counter: ResMut<FruitDotCounter>,
) {
    if level.is_changed() {
        commands.remove_resource::<FruitDespawnTimer>();
        **fruit_dot_counter = 0
    }
}