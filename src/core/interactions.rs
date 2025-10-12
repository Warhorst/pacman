use bevy::prelude::*;
use crate::core::prelude::Fruit;

pub(super) struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PacmanWasHit>()
            .register_type::<GhostWasEaten>()
            .register_type::<DotWasEaten>()
            .register_type::<EnergizerWasEaten>()
            .register_type::<FruitWasEaten>()
            .add_message::<PacmanWasHit>()
            .add_message::<GhostWasEaten>()
            .add_message::<DotWasEaten>()
            .add_message::<EnergizerWasEaten>()
            .add_message::<FruitWasEaten>()
        ;
    }
}

/// Fired when pacman was hit by a ghost.
#[derive(Message, Reflect)]
pub struct PacmanWasHit;

/// Fired when Pacman ate a ghost in frightened state.
/// Contains the eaten ghost entity and the transform to show a score on the ghosts
/// former position.
#[derive(Message, Reflect, Copy, Clone)]
pub struct GhostWasEaten(pub Entity, pub Transform);

/// Fired when pacman eats a dot.
#[derive(Message, Reflect)]
pub struct DotWasEaten;

/// Fired when pacman eats an energizer.
#[derive(Message, Reflect, Copy, Clone)]
pub struct EnergizerWasEaten;

/// Event that gets fired when pacman ate a fruit.
/// Holds the type of fruit and the transform to show a score on the fruits
/// former position.
#[derive(Message, Reflect)]
pub struct FruitWasEaten(pub Fruit, pub Transform);