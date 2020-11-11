use bevy::prelude::*;

use crate::common::Position;
use crate::pacman::Pacman;
use crate::points::Point;
use crate::score::Score;

pub struct InteractionsPlugin;

/// Plugin that controls interactions between actors from different categories.
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(pacman_eat_points.system());
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