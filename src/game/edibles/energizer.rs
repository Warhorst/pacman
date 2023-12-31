use bevy::prelude::*;
use crate::core::prelude::*;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Start)),
                spawn_energizer,
            )
            .add_systems(
                Update,
                (
                    start_energizer_timer_when_energizer_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    update_energizer_timer
                        .after(start_energizer_timer_when_energizer_eaten)
                )
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                OnExit(Game(LevelTransition)),
                spawn_energizer,
            )
            .add_systems(
                OnEnter(Game(PacmanHit)),
                despawn_energizer_timer,
            )
            .add_systems(
                OnEnter(Game(LevelTransition)),
                despawn_energizer_timer,
            )
            .add_systems(
                OnExit(Game(GameOver)),
                (
                    despawn_energizers,
                    despawn_energizer_timer
                ),
            )
            .add_systems(
                Update,
                animate_energizers
                    .run_if(in_game),
            )
        ;
    }
}

fn spawn_energizer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_query: Query<&EnergizerSpawn>,
) {
    let energizers = commands.spawn((
        Name::new("Energizers"),
        Energizers,
        SpatialBundle::default()
    )).id();

    for spawn in &spawn_query {
        commands.entity(energizers).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/energizer.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(ENERGIZER_DIMENSION)),
                        ..default()
                    },
                    transform: Transform::from_translation(**spawn),
                    ..Default::default()
                },
                Energizer,
                Edible,
                Name::new("Energizer")
            ));
        });
    }
}

fn start_energizer_timer_when_energizer_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EnergizerWasEaten>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    for _ in event_reader.read() {
        let spec = specs_per_level.get_for(&level);
        commands.insert_resource(EnergizerTimer::start(spec.frightened_time));
    }
}

fn update_energizer_timer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerOver>,
    energizer_timer: Option<ResMut<EnergizerTimer>>,
    time: Res<Time>,
) {
    if let Some(mut timer) = energizer_timer {
        timer.tick(time.delta());

        if timer.is_finished() {
            commands.remove_resource::<EnergizerTimer>();
            event_writer.send(EnergizerOver);
        }
    }
}

fn despawn_energizer_timer(
    mut commands: Commands,
) {
    commands.remove_resource::<EnergizerTimer>();
}

fn despawn_energizers(
    mut commands: Commands,
    query: Query<Entity, With<Energizers>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Deref, DerefMut)]
struct EnergizerAnimationTimer(Timer);

impl Default for EnergizerAnimationTimer {
    fn default() -> Self {
        EnergizerAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
    }
}

/// Let energizers blink like in the real game
fn animate_energizers(
    time: Res<Time>,
    mut timer: Local<EnergizerAnimationTimer>,
    mut query: Query<&mut Visibility, With<Energizers>>,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        for mut vis in &mut query {
            *vis = match *vis {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible
            }
        }
    }
}