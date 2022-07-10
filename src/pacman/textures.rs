use bevy::prelude::*;

use std::collections::HashMap;
use std::time::Duration;
use crate::pacman::Pacman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::map::Rotation;

pub (in crate::pacman) fn update_pacman_appearance(
    animation: Res<Animation>,
    pacman_textures: Res<PacmanTextures>,
    mut query: Query<(&Direction, &mut Transform, &mut Handle<Image>), With<Pacman>>
) {
    for (direction, mut transform, mut texture) in query.iter_mut() {
        *texture = pacman_textures.get_texture_for_phase(animation.current_phase);

        match direction {
            Up => transform.rotation = Rotation::D90.quat_z(),
            Down => transform.rotation = Rotation::D270.quat_z(),
            Left => transform.rotation = Rotation::D0.quat_z(),
            Right => transform.rotation = Rotation::D180.quat_z(),
        }
    }
}

pub (in crate::pacman) fn update_animation(
    time: Res<Time>,
    mut animation: ResMut<Animation>
) {
    animation.update(time.delta());
}

/// Resource that holds every texture for pacman.
pub (in crate::pacman) struct PacmanTextures {
    textures: HashMap<Phase, Handle<Image>>
}

impl PacmanTextures {
    pub  fn new(asset_server: &AssetServer) -> Self {
        let mut textures = HashMap::with_capacity(3);
        textures.insert(Phase::Closed, asset_server.load("textures/pacman/pacman_closed.png"));
        textures.insert(Phase::Opening, asset_server.load("textures/pacman/pacman_opening.png"));
        textures.insert(Phase::Open, asset_server.load("textures/pacman/pacman_open.png"));

        PacmanTextures {
            textures
        }
    }

    pub fn get_texture_for_phase(&self, phase: Phase) -> Handle<Image> {
        self.textures.get(&phase).expect("for every animation phase of pacman should be a texture registered").clone()
    }
}

/// Resource that determines in which animation phase pacman currently is.
pub (in crate::pacman) struct Animation {
    timer: Timer,
    current_phase: Phase
}

impl Animation {
    const ANIMATION_DURATION_SECS: f32 = 0.2;

    pub fn new() -> Self {
        Animation {
            timer: Timer::new(Duration::from_secs_f32(Self::ANIMATION_DURATION_SECS), true),
            current_phase: Phase::Closed
        }
    }

    fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        let elapsed = self.timer.elapsed_secs();

        self.current_phase = if elapsed < Self::ANIMATION_DURATION_SECS  * 0.25 {
            Phase::Closed
        } else if elapsed < Self::ANIMATION_DURATION_SECS  * 0.5 {
            Phase::Opening
        } else if elapsed < Self::ANIMATION_DURATION_SECS  * 0.75 {
            Phase::Open
        } else {
            Phase::Opening
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub (in crate::pacman) enum Phase {
    Open,
    Opening,
    Closed
}