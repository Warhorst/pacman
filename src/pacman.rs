use bevy::prelude::*;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(set_direction.system())
            .add_system(move_pacman.system());
    }
}

struct Pacman(Direction);

enum Direction {
    Idle,
    Up,
    Down,
    Left,
    Right,
}

fn spawn_pacman(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Pacman(Direction::Idle));
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Pacman>) {
    for mut pacman in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            pacman.0 = Direction::Left
        }

        if keyboard_input.pressed(KeyCode::Right) {
            pacman.0 = Direction::Right
        }

        if keyboard_input.pressed(KeyCode::Up) {
            pacman.0 = Direction::Up
        }

        if keyboard_input.pressed(KeyCode::Down) {
            pacman.0 = Direction::Down
        }
    }
}

fn move_pacman(time: Res<Time>, mut query: Query<(&Pacman, &mut Transform)>) {
    for (pacman, mut transform) in &mut query.iter() {
        let translation = &mut transform.translation_mut();

        let (x, y) = match &pacman.0 {
            Direction::Up => (0.0, 1.0),
            Direction::Down => (0.0, -1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
            Direction::Idle => (0.0, 0.0)
        };

        *translation.x_mut() += time.delta_seconds * x * 500.0;
        *translation.x_mut() = translation.x().min(400.0).max(-400.0);
        *translation.y_mut() += time.delta_seconds * y * 500.0;
        *translation.y_mut() = translation.y().min(200.0).max(-200.0);
    }
}