use bevy::prelude::*;

use crate::common::Position;
use crate::ghosts::Ghost;
use crate::pacman::Pacman;
use crate::points::Point;
use crate::score::Score;

pub struct InteractionsPlugin;

/// Plugin that controls interactions between actors from different categories.
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(pacman_eat_points.system())
            .add_system(ghost_hits_pacman.system());
    }
}

fn pacman_eat_points(mut commands: Commands, mut score: ResMut<Score>, pacman_component: Query<With<Pacman, &Position>>, point_components: Query<With<Point, (Entity, &Position)>>) {
    for pacman_pos in pacman_component.iter() {
        for (entity, point_pos) in point_components.iter() {
            if pacman_pos == point_pos {
                score.increment();
                commands.despawn(entity);
            }
        }
    }
}

fn ghost_hits_pacman(mut commands: Commands, pacman_query: Query<With<Pacman, (Entity, &Position)>>, ghost_query: Query<With<Ghost, &Position>>) {
    for (pacman_entity, pacman_position) in pacman_query.iter() {
        for ghost_position in ghost_query.iter() {
            if pacman_position == ghost_position {
                commands.despawn(pacman_entity);
            }
        }
    }
}