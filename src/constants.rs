pub const MAP_PATH: &'static str = "./maps/new_map.json";

const SCALE: f32 = 1.75;

pub const FIELD_DIMENSION: f32 = 15.0 * SCALE;
pub const PACMAN_DIMENSION: f32 = 22.5 * SCALE;
pub const PACMAN_SPEED: f32 = 125.0 * SCALE;
pub const WALL_DIMENSION: f32 = FIELD_DIMENSION;
pub const POINT_DIMENSION: f32 = 3.0 * SCALE;
pub const ENERGIZER_DIMENSION: f32 = 7.5 * SCALE;
pub const GHOST_DIMENSION: f32 = PACMAN_DIMENSION;
pub const GHOST_SPEED: f32 = PACMAN_SPEED;

pub const POINTS_PER_DOT: usize = 10;
pub const POINTS_PER_ENERGIZER: usize = 50;
pub const POINTS_PER_GHOST: usize = 200;