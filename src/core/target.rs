use bevy::prelude::*;

pub(super) struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Target>()
        ;
    }
}

#[derive(Component, Reflect)]
pub struct Target {
    coordinates: Option<Vec3>,
}

impl Target {
    pub fn new() -> Self {
        Target { coordinates: None }
    }

    pub fn is_set(&self) -> bool {
        self.coordinates.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    /// Return the coordinates without checking if they are present.
    /// The check should happen somewhere else anyway.
    pub fn get(&self) -> Vec3 {
        self.coordinates.unwrap()
    }

    pub fn set(&mut self, coordinates: Vec3) {
        self.coordinates = Some(coordinates)
    }

    pub fn clear(&mut self) {
        self.coordinates = None
    }
}