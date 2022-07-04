use std::time::Duration;
use bevy::prelude::*;
use crate::level::Level;
use Fruit::*;
use crate::common::position::ToPosition;
use crate::constants::PACMAN_DIMENSION;
use crate::dots::DotEaten;
use crate::pacman::Pacman;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FruitEaten>()
            .insert_resource(FruitDotCounter::new())
            .add_system(spawn_fruit_when_dot_limit_reached)
            .add_system(update_despawn_timer)
            .add_system(increase_dot_counter_when_dot_was_eaten)
            .add_system(despawn_fruit_if_timer_exceeded)
            .add_system(reset_fruit_dot_counter_and_despawn_timer_when_level_changed)
            .add_system(eat_fruit_when_pacman_touches_it)
        ;
    }
}

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    level: Res<Level>,
    fruit_dot_counter: Res<FruitDotCounter>,
) {
    let eaten_dots = **fruit_dot_counter;

    if eaten_dots == 70 || eaten_dots == 170 {
        // TODO: get real coordinates from board
        let coordinates = Vec3::new(0.0, 0.0, 0.0);
        let dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);

        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    custom_size: Some(dimension),
                    ..default()
                },
                transform: Transform::from_translation(coordinates),
                ..Default::default()
            })
            .insert(get_fruit_for_level(&level));
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

/// When a dot was eaten, increase the dot counter.
fn increase_dot_counter_when_dot_was_eaten(
    mut event_reader: EventReader<DotEaten>,
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

/// If pacman touches the fruit, despawn it, remove the timer and send an event.
fn eat_fruit_when_pacman_touches_it(
    mut commands: Commands,
    mut event_writer: EventWriter<FruitEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    fruit_query: Query<(Entity, &Transform), With<Fruit>>
) {
    for pacman_tf in pacman_query.iter() {
        for (entity, fruit_tf) in fruit_query.iter() {
            if pacman_tf.pos() == fruit_tf.pos() {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(FruitEaten)
            }
        }
    }
}

#[derive(Component)]
enum Fruit {
    Cherry,
    Strawberry,
    Peach,
    Apple,
    Grapes,
    Galaxian,
    Bell,
    Key,
}

/// Event that gets fired when pacman ate a fruit.
pub struct FruitEaten;

#[derive(Deref, DerefMut)]
struct FruitDespawnTimer(Timer);

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