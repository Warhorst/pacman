use bevy::prelude::*;
use crate::common::has_no_events;
use crate::constants::PACMAN_DIMENSION;
use crate::pacman::PacmanKilled;
use crate::score::Score;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PointsRequiredForExtraLife::new())
            .add_startup_system(spawn_lives)
            .add_system(remove_life_when_pacman_dies)
            .add_system(add_life_if_player_reaches_specific_score)
        ;
    }
}

const LIVES: usize = 3;

/// Represents a life of pacman. A life gets removed if pacman dies or gets added
/// if the player reaches a specific score
#[derive(Component, Ord, PartialOrd, Eq, PartialEq)]
pub struct Life(usize);

/// Keeps track how many points the player needs to get a new life for pacman.
#[derive(Deref, DerefMut)]
pub struct PointsRequiredForExtraLife(usize);

impl PointsRequiredForExtraLife {
    pub fn new() -> Self {
        PointsRequiredForExtraLife(10000)
    }

    pub fn increase_limit(&mut self) {
        **self += 10000
    }
}

fn spawn_lives(
    mut commands: Commands,
) {
    for i in 0..LIVES {
        spawn_life(&mut commands, i)
    }
}

fn spawn_life(commands: &mut Commands, life_index: usize) {
    let life_x = 480.0 + (life_index as f32) * (PACMAN_DIMENSION) * 2.0;

    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("FFEE00").unwrap(),
                custom_size: Some(Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(life_x, 500.0, 0.0)),
            ..default()
        })
        .insert(Life(life_index));
}

fn remove_life_when_pacman_dies(
    mut commands: Commands,
    event_reader: EventReader<PacmanKilled>,
    query: Query<(Entity, &Life)>,
) {
    if has_no_events(event_reader) { return; }

    let life_to_remove = query.iter()
        .max_by(|(_, i0), (_, i1)| i0.cmp(i1));

    if let Some((e, _)) = life_to_remove {
        commands.entity(e).despawn()
    }
}

fn add_life_if_player_reaches_specific_score(
    mut commands: Commands,
    score: Res<Score>,
    mut points_required_for_extra_life: ResMut<PointsRequiredForExtraLife>,
    query: Query<&Life>
) {
    if **score >= **points_required_for_extra_life {
        let index = query.iter().count();
        spawn_life(&mut commands, index);
        points_required_for_extra_life.increase_limit();
    }
}

