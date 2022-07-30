use bevy::prelude::*;
use crate::constants::{FIELD_DIMENSION, PACMAN_DIMENSION};
use crate::life_cycle::LifeCycle;
use crate::life_cycle::LifeCycle::Start;
use crate::map::board::Board;
use crate::pacman::EPacmanHit;
use crate::score::Score;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PointsRequiredForExtraLife::new())
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_lives)
            )
            .add_system_set(
                SystemSet::on_update(LifeCycle::Running)
                    .with_system(remove_life_when_pacman_dies)
                    .with_system(add_life_if_player_reaches_specific_score)
            )
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
    asset_server: Res<AssetServer>,
    board: Res<Board>
) {
    for i in 0..LIVES {
        spawn_life(&mut commands, &asset_server, &board, i)
    }
}

fn spawn_life(commands: &mut Commands, asset_server: &AssetServer, board: &Board, life_index: usize) {
    let life_x = FIELD_DIMENSION * board.width as f32 + (life_index as f32) * (FIELD_DIMENSION) * 2.0;

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/pacman/pacman_life.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(life_x, FIELD_DIMENSION * board.height as f32, 0.0)),
            ..default()
        })
        .insert(Life(life_index));
}

fn remove_life_when_pacman_dies(
    mut commands: Commands,
    mut event_reader: EventReader<EPacmanHit>,
    query: Query<(Entity, &Life)>,
) {
    for _ in event_reader.iter() {
        let life_to_remove = query.iter()
            .max_by(|(_, i0), (_, i1)| i0.cmp(i1));

        if let Some((e, _)) = life_to_remove {
            commands.entity(e).despawn()
        }
    }
}

fn add_life_if_player_reaches_specific_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    mut points_required_for_extra_life: ResMut<PointsRequiredForExtraLife>,
    board: Res<Board>,
    query: Query<&Life>
) {
    if **score >= **points_required_for_extra_life {
        let index = query.iter().count();
        spawn_life(&mut commands, &asset_server, &board, index);
        points_required_for_extra_life.increase_limit();
    }
}

