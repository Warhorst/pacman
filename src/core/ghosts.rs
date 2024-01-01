use bevy::prelude::*;

pub(super) struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Ghost>()
            .register_type::<CurrentlyEatenGhost>()
        ;
    }
}

#[derive(Component, Reflect, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum Ghost {
    #[default]
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

/// Resource that holds the entity id of the ghost that is currently eaten by pacman
/// The currently eaten ghost be known to ste him invisible while the ghost eaten pause is active
#[derive(Resource, Reflect, Deref)]
pub struct CurrentlyEatenGhost(pub Entity);