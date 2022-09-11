use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::interactions::EPacmanHit;
use crate::life_cycle::LifeCycle;
use crate::life_cycle::LifeCycle::Start;
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
    asset_handles: Res<LoadedAssets>,
    dimensions: Res<BoardDimensions>
) {
    for i in 0..LIVES {
        spawn_life(&mut commands, &asset_handles, i, &dimensions)
    }
}

fn spawn_life(
    commands: &mut Commands,
    asset_handles: &LoadedAssets,
    life_index: usize,
    dimensions: &BoardDimensions
) {
    let origin = dimensions.origin();
    let life_x = origin.x + (life_index as f32) * (dimensions.life()) * 2.0;

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_handles.get_handle("textures/pacman/pacman_life.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(dimensions.life(), dimensions.life())),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(life_x, origin.y - dimensions.life(), 0.0)),
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
    game_assets: Res<LoadedAssets>,
    score: Res<Score>,
    mut points_required_for_extra_life: ResMut<PointsRequiredForExtraLife>,
    dimensions: Res<BoardDimensions>,
    query: Query<&Life>
) {
    if **score >= **points_required_for_extra_life {
        let index = query.iter().count();
        spawn_life(&mut commands, &game_assets, index, &dimensions);
        points_required_for_extra_life.increase_limit();
    }
}

