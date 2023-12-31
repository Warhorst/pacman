use bevy::prelude::*;
use crate::core::prelude::*;

pub (in crate::game) struct SpecsPerLevelPlugin;

impl Plugin for SpecsPerLevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpecsPerLevel::default())
        ;
    }
}