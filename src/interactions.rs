use bevy::prelude::*;

use crate::common::Position;
use crate::pacman::Pacman;
use crate::points::Point;

pub struct InteractionsPlugin;

/// Plugin that controls interactions between actors from different categories.
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(pacman_eat_points.system());
    }
}

fn pacman_eat_points(mut commands: Commands, pacman_component: Query<(&Pacman, &Position)>, point_components: Query<(Entity, &Point, &Position)>) {
    for (_pacman, pacman_pos) in pacman_component.iter() {
        for (entity, _point, point_pos) in point_components.iter() {
            if pacman_pos == point_pos {
                commands.despawn(entity);
            }
        }
    }
}