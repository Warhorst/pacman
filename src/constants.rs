pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const POINTS_PER_DOT: usize = 10;
pub const POINTS_PER_ENERGIZER: usize = 50;
pub const POINTS_PER_GHOST: usize = 200;

pub const TUNNEL_Z: f32 = 300.0;
pub const TEXT_Z: f32 = 200.0;
pub const PACMAN_Z: f32 = 100.0;
pub const BLINKY_Z: f32 = 90.0;
pub const PINKY_Z: f32 = 80.0;
pub const INKY_Z: f32 = 700.0;
pub const CLYDE_Z: f32 = 60.0;
pub const ENERGIZER_Z: f32 = 50.0;
pub const DOT_Z: f32 = 40.0;
pub const FRUIT_Z: f32 = 30.0;

pub const FONT: &'static str = "fonts/PressStart2P-Regular.ttf";

pub const FIELD_DIMENSION: f32 = 18.5;
pub const WALL_DIMENSION: f32 = FIELD_DIMENSION;
pub const PACMAN_DIMENSION: f32 = FIELD_DIMENSION + FIELD_DIMENSION * 0.6;
pub const DOT_DIMENSION: f32 = PACMAN_DIMENSION;
pub const ENERGIZER_DIMENSION: f32 = PACMAN_DIMENSION;
pub const FRUIT_DIMENSION: f32 = PACMAN_DIMENSION;
pub const GHOST_DIMENSION: f32 = PACMAN_DIMENSION;

pub const PACMAN_BASE_SPEED: f32 = FIELD_DIMENSION * 9.0;
pub const GHOST_BASE_SPEED: f32 = PACMAN_BASE_SPEED;