use bevy::prelude::*;
use rand::prelude::*;

pub(crate) struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Random)
        ;
    }
}

/// Provides randomly chosen numbers to whoever needs them.
#[derive(Resource)]
pub struct Random;

impl Random {
    pub fn zero_to(&self, n: usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..n)
    }
}