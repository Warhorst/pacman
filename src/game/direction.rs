use bevy::prelude::*;
use pad::Direction;

/// The direction some entity is currently moving to
#[derive(Component, Reflect, Deref, DerefMut, Copy, Clone, Debug, Eq, PartialEq)]
pub struct MovementDirection(pub Direction);