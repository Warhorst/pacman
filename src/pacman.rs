use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::board::Board;
use crate::common::{Direction, Position};

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(set_direction.system())
            .add_system(move_pacman.system());
    }
}

struct Pacman {
    movement: Movement
}

enum Movement {
    Idle,
    Moving(Direction),
}

fn spawn_pacman(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, board: Res<Board>) {
    let start_position = Position::new(1, 2);
    let pacman_dimension = Vec2::new(25.0, 25.0);
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(board.window_coordinates(&start_position, &pacman_dimension)),
            sprite: Sprite::new(pacman_dimension),
            ..Default::default()
        })
        .with(Pacman { movement: Movement::Idle })
        .with(start_position);
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Pacman>) {
    for mut pacman in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            pacman.movement = Movement::Moving(Direction::Left)
        }

        if keyboard_input.pressed(KeyCode::Right) {
            pacman.movement = Movement::Moving(Direction::Right)
        }

        if keyboard_input.pressed(KeyCode::Up) {
            pacman.movement = Movement::Moving(Direction::Up)
        }

        if keyboard_input.pressed(KeyCode::Down) {
            pacman.movement = Movement::Moving(Direction::Down)
        }
    }
}

fn move_pacman(time: Res<Time>, board: Res<Board>, mut query: Query<(&Pacman, &mut Position, &Sprite, &mut Transform)>) {
    for (pacman, mut position, sprite, mut transform) in &mut query.iter() {
        let direction = match &pacman.movement {
            Movement::Idle => return,
            Movement::Moving(dir) => dir
        };

        let (x, y) = match direction {
            Direction::Up => (0.0, 1.0),
            Direction::Down => (0.0, -1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0)
        };

        let translation = &mut transform.translation_mut();
        *translation.x_mut() += time.delta_seconds * x * 250.0;
        *translation.y_mut() += time.delta_seconds * y * 250.0;
        *position = board.calculate_position(&Vec3::new(translation.x(), translation.y(), 0.0), &sprite.size);

        if board.collides_with_obstacle(&position, &direction) {
            let pacman_coordinates = board.window_coordinates(&position, &sprite.size);
            println!("New: {:?}", Vec3::new(translation.x(), translation.y(), 0.0));
            println!("Bounds: {:?}", pacman_coordinates);
            match direction {
                Direction::Up => *translation.y_mut() = translation.y().min(pacman_coordinates.y()),
                Direction::Down => *translation.y_mut() = translation.y().max(pacman_coordinates.y()),
                Direction::Left => *translation.x_mut() = translation.x().max(pacman_coordinates.x()),
                Direction::Right => *translation.x_mut() = translation.x().min(pacman_coordinates.x())
            };
            println!("New After: {:?}", Vec3::new(translation.x(), translation.y(), 0.0));
        }
    }
}