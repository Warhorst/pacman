use bevy::prelude::*;

use crate::common::{Direction, Position};
use crate::constants::{PACMAN_DIMENSION, PACMAN_SPEED};
use crate::map::board::Board;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(move_pacman.system())
            .add_system(walk_through_tunnel.system());
    }
}

pub struct Pacman;

#[derive(Debug)]
enum Movement {
    Idle,
    Moving(Direction),
}

fn spawn_pacman(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let start_position = board.get_pacman_spawn_position().clone();
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(board.coordinates_of_position(&start_position)),
            sprite: Sprite::new(pacman_dimension),
            ..Default::default()
        })
        .with(Pacman)
        .with(Movement::Idle)
        .with(start_position);
}

fn move_pacman(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, board: Res<Board>, mut query: Query<With<Pacman, (&mut Movement, &mut Position, &mut Transform)>>) {
    for (mut movement, mut position, mut transform) in query.iter_mut() {
        set_direction(&keyboard_input, &mut movement);

        let direction = match *movement {
            Movement::Idle => return,
            Movement::Moving(dir) => dir
        };

        let translation = &mut transform.translation;
        *position = board.position_of_coordinates(translation);
        move_in_direction(&direction, translation, time.delta_seconds);

        if board.going_to_collide_with_obstacle(&position, &direction, translation) {
            process_collision(&board, &position, &direction, translation, &mut movement)
        } else {
            center_position(&board, translation, &position, &direction)
        }
    }
}

fn set_direction(keyboard_input: &Input<KeyCode>, movement: &mut Movement) {
    if keyboard_input.pressed(KeyCode::Left) {
        *movement = Movement::Moving(Direction::Left)
    }

    if keyboard_input.pressed(KeyCode::Right) {
        *movement = Movement::Moving(Direction::Right)
    }

    if keyboard_input.pressed(KeyCode::Up) {
        *movement = Movement::Moving(Direction::Up)
    }

    if keyboard_input.pressed(KeyCode::Down) {
        *movement = Movement::Moving(Direction::Down)
    }
}

fn move_in_direction(direction: &Direction, translation: &mut Vec3, delta_seconds: f32) {
    let (x, y) = get_modifiers_for_direction(direction);
    *translation.x_mut() += delta_seconds * x * PACMAN_SPEED;
    *translation.y_mut() += delta_seconds * y * PACMAN_SPEED;
}

fn get_modifiers_for_direction(direction: &Direction) -> (f32, f32) {
    match direction {
        Direction::Up => (0.0, 1.0),
        Direction::Down => (0.0, -1.0),
        Direction::Left => (-1.0, 0.0),
        Direction::Right => (1.0, 0.0)
    }
}

fn process_collision(board: &Board, position: &Position, direction: &Direction, translation: &mut Vec3, movement: &mut Movement) {
    let field_coordinates = board.coordinates_of_position(&position);
    limit_movement(direction, &field_coordinates, translation);
    stop_if_at_border(direction, &field_coordinates, translation, movement)
}

/// Because the next field is an obstacle, pacman can not go beyond his current field.
fn limit_movement(direction: &Direction, field_coordinates: &Vec3, translation: &mut Vec3) {
    match direction {
        Direction::Up => *translation.y_mut() = translation.y().min(field_coordinates.y()),
        Direction::Down => *translation.y_mut() = translation.y().max(field_coordinates.y()),
        Direction::Left => *translation.x_mut() = translation.x().max(field_coordinates.x()),
        Direction::Right => *translation.x_mut() = translation.x().min(field_coordinates.x())
    }
}

/// If pacman is at a border, he can not go further and stop.
fn stop_if_at_border(direction: &Direction, field_coordinates: &Vec3, translation: &mut Vec3, movement: &mut Movement) {
    match direction {
        Direction::Up | Direction::Down => if field_coordinates.y() == translation.y() {
            *movement = Movement::Idle
        }
        ,
        Direction::Left | Direction::Right => if field_coordinates.x() == translation.x() {
            *movement = Movement::Idle
        }
    }
}

/// When pacman enters a hallway, he might not be centered. This is fixed here.
fn center_position(board: &Board, translation: &mut Vec3, position: &Position, direction: &Direction) {
    let position_coordinates = board.coordinates_of_position(position);
    match direction {
        Direction::Up | Direction::Down => *translation.x_mut() = position_coordinates.x(),
        Direction::Left | Direction::Right => *translation.y_mut() = position_coordinates.y()
    }
}

fn walk_through_tunnel(board: Res<Board>, mut query: Query<With<Pacman, (&Movement, &mut Position, &mut Transform)>>) {
    for (movement, mut position, mut transform) in query.iter_mut() {
        let direction = match movement {
            Movement::Idle => return,
            Movement::Moving(dir) => dir
        };

        match direction {
            Direction::Up | Direction::Down => return,
            Direction::Right => walk_through_right_tunnel(&board, &mut position, &mut transform.translation),
            Direction::Left => walk_through_left_tunnel(&board, &mut position, &mut transform.translation)
        }
    }
}

fn walk_through_right_tunnel(board: &Board, position: &mut Position, translation: &mut Vec3) {
    let right_tunnel_position = board.get_right_tunnel_position();
    let left_tunnel_position = board.get_left_tunnel_position();
    match position == right_tunnel_position {
        false => return,
        true => {
            *translation = board.coordinates_of_position(left_tunnel_position);
            *position = *left_tunnel_position;
        }
    }
}

fn walk_through_left_tunnel(board: &Board, position: &mut Position, translation: &mut Vec3) {
    let right_tunnel_position = board.get_right_tunnel_position();
    let left_tunnel_position = board.get_left_tunnel_position();
    match position == left_tunnel_position {
        false => return,
        true => {
            *translation = board.coordinates_of_position(right_tunnel_position);
            *position = *right_tunnel_position;
        }
    }
}