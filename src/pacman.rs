use bevy::prelude::*;

use crate::board::Board;
use crate::common::{Direction, Position};

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(set_direction.system())
            .add_system(move_pacman.system())
            .add_system(walk_through_tunnel.system());
    }
}

struct Pacman;

#[derive(Debug)]
enum Movement {
    Idle,
    Moving(Direction),
}

fn spawn_pacman(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, board: Res<Board>) {
    let start_position = Position::new(1, 1);
    let pacman_dimension = Vec2::new(20.0, 20.0);
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(board.window_coordinates(&start_position)),
            sprite: Sprite::new(pacman_dimension),
            ..Default::default()
        })
        .with(Pacman)
        .with(Movement::Idle)
        .with(start_position);
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Pacman, &mut Movement)>) {
    for (_pacman, mut movement) in query.iter_mut() {
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
}

fn move_pacman(time: Res<Time>, board: Res<Board>, mut query: Query<(&Pacman, &mut Movement, &Sprite, &mut Position, &mut Transform)>) {
    for (_pacman, mut movement, sprite, mut position, mut transform) in query.iter_mut() {
        let direction = match *movement {
            Movement::Idle => return,
            Movement::Moving(dir) => dir
        };

        let translation = &mut transform.translation;
        *position = board.calculate_position(translation);
        move_in_direction(&direction, translation, time.delta_seconds);

        if board.collides_with_obstacle(&position, &direction, translation, &sprite.size) {
            process_collision(&board, &position, &direction, translation, &mut movement)
        } else {
            center_position(&board, translation, &position, &direction)
        }
    }
}

fn move_in_direction(direction: &Direction, translation: &mut Vec3, delta_seconds: f32) {
    let speed = 250.0;
    let (x, y) = get_modifiers_for_direction(direction);
    *translation.x_mut() += delta_seconds * x * speed;
    *translation.y_mut() += delta_seconds * y * speed;
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
    let border_coordinates = board.window_coordinates(&position);
    limit_movement(direction, &border_coordinates, translation);
    stop_if_at_border(direction, &border_coordinates, translation, movement)
}

fn limit_movement(direction: &Direction, border_coordinates: &Vec3, translation: &mut Vec3) {
    match direction {
        Direction::Up => *translation.y_mut() = translation.y().min(border_coordinates.y()),
        Direction::Down => *translation.y_mut() = translation.y().max(border_coordinates.y()),
        Direction::Left => *translation.x_mut() = translation.x().max(border_coordinates.x()),
        Direction::Right => *translation.x_mut() = translation.x().min(border_coordinates.x())
    }
}

fn stop_if_at_border(direction: &Direction, border_coordinates: &Vec3, translation: &mut Vec3, movement: &mut Movement) {
    match direction {
        Direction::Up | Direction::Down => if border_coordinates.y() == translation.y() {
            *movement = Movement::Idle
        }
        ,
        Direction::Left | Direction::Right => if border_coordinates.x() == translation.x() {
            *movement = Movement::Idle
        }
    }
}

fn center_position(board: &Board, translation: &mut Vec3, position: &Position, direction: &Direction) {
    let position_coordinates = board.window_coordinates(position);
    match direction {
        Direction::Up | Direction::Down => *translation.x_mut() = position_coordinates.x(),
        Direction::Left | Direction::Right => *translation.y_mut() = position_coordinates.y()
    }
}

fn walk_through_tunnel(board: Res<Board>, mut query: Query<(&Pacman, &Movement, &mut Position, &mut Transform)>) {
    for(_pacman, movement, mut position, mut transform) in query.iter_mut() {
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
            *translation = board.window_coordinates(left_tunnel_position);
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
            *translation = board.window_coordinates(right_tunnel_position);
            *position = *right_tunnel_position;
        }
    }
}