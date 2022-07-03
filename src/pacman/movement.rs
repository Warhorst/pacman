use std::time::Duration;
use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;

use crate::common::Direction;
use crate::common::Direction::*;
use crate::common::Position;
use crate::constants::{PACMAN_DIMENSION, WALL_DIMENSION};
use crate::dots::DotEaten;
use crate::energizer::EnergizerEaten;
use crate::is;
use crate::map::board::Board;
use crate::map::Element;
use crate::pacman::Pacman;
use crate::speed::Speed;

pub struct PacmanMovementPlugin;

impl Plugin for PacmanMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PacmanStopTimer::new())
            .add_system(stop_pacman_when_a_dot_was_eaten.label("pacman_stop"))
            .add_system(stop_pacman_when_energizer_was_eaten.label("pacman_stop"))
            .add_system(move_pacman.after("pacman_stop"))
            .add_system(update_stop_timer.after(move_pacman))
        ;
    }
}

/// Timer that tells how long pacman will be unable to move.
struct PacmanStopTimer {
    timer: Option<Timer>,
}

impl PacmanStopTimer {
    pub fn new() -> Self {
        PacmanStopTimer {
            timer: None
        }
    }

    /// Stop pacman for 1/60 second.
    ///
    /// On the arcade machine (which was locked to 60 FPS), pacman just stopped for one frame. This does not have
    /// the desired effect when playing on 144 or 30 FPS.
    pub fn start_for_dot(&mut self) {
        self.timer = Some(Timer::from_seconds(1.0 / 60.0, false))
    }

    /// Stop pacman for 3/60 second.
    ///
    /// On the arcade machine (which was locked to 60 FPS), pacman just stopped for three frames. This does not have
    /// the desired effect when playing on 144 or 30 FPS.
    pub fn start_for_energizer(&mut self) {
        self.timer = Some(Timer::from_seconds(3.0 / 60.0, false))
    }

    pub fn tick(&mut self, delta: Duration) {
        if let Some(ref mut timer) = self.timer {
            timer.tick(delta);
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.timer {
            None => true,
            Some(ref t) => t.finished()
        }
    }

    pub fn is_active(&self) -> bool {
        !self.is_finished()
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct MoveComponents<'a> {
    direction: &'a Direction,
    transform: &'a mut Transform,
    speed: &'a Speed,
}

fn move_pacman(
    board: Res<Board>,
    time: Res<Time>,
    pacman_stop_timer: Res<PacmanStopTimer>,
    mut query: Query<MoveComponents, With<Pacman>>,
) {
    if pacman_stop_timer.is_active() { return; }

    let delta_seconds = time.delta_seconds();

    for mut move_components in query.iter_mut() {
        let mut new_coordinates = calculate_new_coordinates(&mut move_components, delta_seconds);
        let new_position = new_coordinates.into();

        if is_going_to_collide_with_obstacle(&board, &move_components.direction, &new_position, &new_coordinates) {
            process_collision(&move_components.direction, &new_position, &mut new_coordinates)
        } else {
            center_position(&move_components.direction, &new_position, &mut new_coordinates)
        }

        move_components.transform.translation = new_coordinates;
    }
}

/// Calculate pacmans new coordinates on the window based on his speed and the time.
fn calculate_new_coordinates(move_components: &mut MoveComponentsItem, delta_seconds: f32) -> Vec3 {
    let (x, y) = get_modifiers_for_direction(move_components.direction);
    let mut new_coordinates = move_components.transform.translation;
    new_coordinates.x += delta_seconds * x * **move_components.speed;
    new_coordinates.y += delta_seconds * y * **move_components.speed;
    new_coordinates
}

fn get_modifiers_for_direction(direction: &Direction) -> (f32, f32) {
    match direction {
        Up => (0.0, 1.0),
        Down => (0.0, -1.0),
        Left => (-1.0, 0.0),
        Right => (1.0, 0.0)
    }
}

/// Determine if pacman will collide with an obstacle if he is going further in his current direction.
fn is_going_to_collide_with_obstacle(board: &Board, direction: &Direction, new_position: &Position, new_coordinates: &Vec3) -> bool {
    let pos_in_direction = new_position.neighbour_position(direction);

    if position_is_obstacle(board, &pos_in_direction) {
        true
    } else {
        !are_coordinates_in_field_center(direction, &pos_in_direction, new_coordinates, PACMAN_DIMENSION)
    }
}

/// Determines if pacmans current coordinates are in the center of his current position. The center of the position is
/// its middle point with the width/height of the accumulated distance between pacman and the walls.
/// Assumes pacman is larger than a wall.
pub fn are_coordinates_in_field_center(direction: &Direction, position: &Position, coordinates: &Vec3, entity_dimension: f32) -> bool {
    let position_coordinates = Vec3::from(position);
    let entity_wall_distance = match entity_dimension > WALL_DIMENSION {
        true => entity_dimension - WALL_DIMENSION,
        false => 0.0
    };
    match direction {
        Left | Right => {
            let y_start = position_coordinates.y - entity_wall_distance;
            let y_end = position_coordinates.y + entity_wall_distance;
            coordinates.y >= y_start && coordinates.y <= y_end
        }
        Up | Down => {
            let x_start = position_coordinates.x - entity_wall_distance;
            let x_end = position_coordinates.x + entity_wall_distance;
            coordinates.x >= x_start && coordinates.x <= x_end
        }
    }
}

/// Tells if the given position is an obstacle for pacman.
fn position_is_obstacle(board: &Board, position: &Position) -> bool {
    board.position_matches_filter(position, is!(Element::Wall {..} | Element::GhostHouseEntrance {..} | Element::InvisibleWall))
}

/// Limit pacmans movement if he reached an obstacle and stop him.
fn process_collision(direction: &Direction, new_position: &Position, new_coordinates: &mut Vec3) {
    let field_coordinates = new_position.into();
    limit_movement(direction, &field_coordinates, new_coordinates);
}

/// Because the next field is an obstacle, pacman can not go beyond his current field.
fn limit_movement(direction: &Direction, field_coordinates: &Vec3, new_coordinates: &mut Vec3) {
    match direction {
        Up => new_coordinates.y = new_coordinates.y.min(field_coordinates.y),
        Down => new_coordinates.y = new_coordinates.y.max(field_coordinates.y),
        Left => new_coordinates.x = new_coordinates.x.max(field_coordinates.x),
        Right => new_coordinates.x = new_coordinates.x.min(field_coordinates.x)
    }
}

/// Center pacmans current position in the middle of his current field.
/// The purpose of this method is to keep equally sized gaps to the hallway pacman is currently passing.
fn center_position(direction: &Direction, new_position: &Position, new_coordinates: &mut Vec3) {
    let position_coordinates = Vec3::from(new_position);
    match direction {
        Up | Down => new_coordinates.x = position_coordinates.x,
        Left | Right => new_coordinates.y = position_coordinates.y
    }
}

/// When pacman eats a dot, he will stop for a moment. This allows
/// the ghost to catch up on him if he continues to eat dots.
fn stop_pacman_when_a_dot_was_eaten(
    mut event_reader: EventReader<DotEaten>,
    mut pacman_stop_timer: ResMut<PacmanStopTimer>,
) {
    for _ in event_reader.iter() {
        pacman_stop_timer.start_for_dot();
    }
}

fn stop_pacman_when_energizer_was_eaten(
    mut event_reader: EventReader<EnergizerEaten>,
    mut pacman_stop_timer: ResMut<PacmanStopTimer>,
) {
    for _ in event_reader.iter() {
        pacman_stop_timer.start_for_energizer();
    }
}

fn update_stop_timer(
    time: Res<Time>,
    mut pacman_stop_timer: ResMut<PacmanStopTimer>,
) {
    pacman_stop_timer.tick(time.delta());
}