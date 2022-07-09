use std::any::TypeId;
use std::collections::HashMap;
use std::time::Duration;
use bevy::prelude::*;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};
use crate::ghosts::state::State;

type NormalTextures = HashMap<(TypeId, Direction, Phase), Handle<Image>>;

pub (in crate::ghosts) fn update_ghost_appearance<G: 'static + Component + GhostType>(
    ghost_textures: Res<GhostTextures>,
    animation: Res<Animation>,
    mut query: Query<(&Direction, &State, &mut Handle<Image>), With<G>>
) {
    for (direction, state, mut texture) in query.iter_mut() {
        match state {
            State::Frightened => *texture = ghost_textures.get_frightened_texture_for(animation.current_phase),
            State::Eaten => *texture = ghost_textures.get_eaten_texture(&direction),
            _ => *texture = ghost_textures.get_normal_texture_for::<G>(&direction, animation.current_phase)
        }
    }
}

pub (in crate::ghosts) fn update_animation(
    time: Res<Time>,
    mut animation: ResMut<Animation>
) {
    animation.update(time.delta())
}

/// Resource that holds all handles to ghost textures.
pub struct GhostTextures {
    normal_ghost_textures: NormalTextures,
    frightened_texture: HashMap<Phase, Handle<Image>>,
    eaten_textures_by_direction: HashMap<Direction, Handle<Image>>
}

impl GhostTextures {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut normal_ghost_textures = HashMap::with_capacity(4 * 4 * 2); // #ghost * #directions * #phases
        Self::load_movement_textures(&mut normal_ghost_textures, ("blinky", TypeId::of::<Blinky>()), asset_server);
        Self::load_movement_textures(&mut normal_ghost_textures, ("pinky", TypeId::of::<Pinky>()), asset_server);
        Self::load_movement_textures(&mut normal_ghost_textures, ("inky", TypeId::of::<Inky>()), asset_server);
        Self::load_movement_textures(&mut normal_ghost_textures, ("clyde", TypeId::of::<Clyde>()), asset_server);

        let frightened_texture = Self::load_frightened_textures(asset_server);
        let eaten_textures_by_direction = Self::load_eaten_textures(asset_server);

        GhostTextures {
            normal_ghost_textures,
            frightened_texture,
            eaten_textures_by_direction
        }
    }

    fn load_movement_textures(textures: &mut NormalTextures, ghost_name_id: (&str, TypeId), asset_server: &AssetServer) {
        for dir in [Up, Down, Left, Right] {
            Self::load_phase_textures(textures, ghost_name_id, dir, asset_server)
        }
    }

    fn load_phase_textures(textures: &mut NormalTextures, ghost_name_id: (&str, TypeId), dir: Direction, asset_server: &AssetServer) {
        let t_id = ghost_name_id.1;
        let ghost_name = ghost_name_id.0;
        let direction_str = match dir {
            Up => "up",
            Down => "down",
            Left => "left",
            Right => "right",
        };

        textures.insert((t_id, dir, Phase::A), asset_server.load(&format!("textures/ghost/{}_{}_a.png", ghost_name, direction_str)));
        textures.insert((t_id, dir, Phase::B), asset_server.load(&format!("textures/ghost/{}_{}_b.png", ghost_name, direction_str)));
    }

    fn load_frightened_textures(asset_server: &AssetServer) -> HashMap<Phase, Handle<Image>> {
        let mut textures = HashMap::with_capacity(2);
        textures.insert(Phase::A, asset_server.load("textures/ghost/frightened_a.png"));
        textures.insert(Phase::B, asset_server.load("textures/ghost/frightened_b.png"));
        textures
    }

    fn load_eaten_textures(asset_server: &AssetServer) -> HashMap<Direction, Handle<Image>> {
        let mut map = HashMap::new();
        map.insert(Up, asset_server.load("textures/ghost/eyes_up.png"));
        map.insert(Down, asset_server.load("textures/ghost/eyes_down.png"));
        map.insert(Left, asset_server.load("textures/ghost/eyes_left.png"));
        map.insert(Right, asset_server.load("textures/ghost/eyes_right.png"));
        map
    }

    // TODO is cloning the right way?
    pub fn get_normal_texture_for<G: 'static + GhostType>(&self, direction: &Direction, phase: Phase) -> Handle<Image> {
        self.normal_ghost_textures
            .get(&(TypeId::of::<G>(), *direction, phase))
            .expect("for every ghost, direction and animation phase should be a texture registered")
            .clone()
    }

    pub fn get_frightened_texture_for(&self, phase: Phase) -> Handle<Image> {
        self.frightened_texture.get(&phase).expect("for every phase should be a frightened texture registered").clone()
    }

    pub fn get_eaten_texture(&self, direction: &Direction) -> Handle<Image> {
        self.eaten_textures_by_direction
            .get(direction)
            .expect("every direction should have a texture for eaten ghosts")
            .clone()
    }
}

/// Resource that controls the animation of ghost. This means: it determines which texture variant should
/// be displayed (a or b).
pub (in crate::ghosts) struct Animation {
    timer: Timer,
    current_phase: Phase
}

impl Animation {
    const ANIMATION_DURATION_SECS: f32 = 0.5;

    pub (in crate::ghosts) fn new() -> Self {
        Animation {
            timer: Timer::new(Duration::from_secs_f32(Self::ANIMATION_DURATION_SECS), true),
            current_phase: Phase::A
        }
    }

    fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);

        self.current_phase = if self.timer.elapsed_secs() < Self::ANIMATION_DURATION_SECS / 2.0 {
            Phase::A
        } else {
            Phase::B
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Phase {
    A,
    B
}