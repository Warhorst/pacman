use bevy::prelude::*;

pub(super) struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Speed>()
        ;
    }
}

/// The current speed of a moving entity
#[derive(Copy, Clone, Default, Component, Deref, DerefMut, Reflect)]
pub struct Speed(pub f32);