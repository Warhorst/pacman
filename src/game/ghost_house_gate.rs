use bevy::prelude::*;

use crate::core::prelude::*;

pub(in crate::game) struct GhostHouseGatePlugin;

impl Plugin for GhostHouseGatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Start)),
                create_gate
            )
            .add_systems(
                Update,
                (
                    update_ghost_house_gate,
                    increment_counter_when_dot_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    switch_to_global_counter_when_pacman_got_killed
                        .in_set(ProcessIntersectionsWithPacman)
                )
                    .run_if(in_state(Game(Running))))
        ;
    }
}

fn create_gate(
    mut commands: Commands,
    level: Res<Level>,
) {
    commands.insert_resource(GhostHouseGate::new(&level));
}

fn update_ghost_house_gate(
    time: Res<Time>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    ghost_house_gate.update(time.delta())
}

fn increment_counter_when_dot_eaten(
    mut event_reader: EventReader<DotWasEaten>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    for _ in event_reader.read() {
        ghost_house_gate.increment_counter()
    }
}

fn switch_to_global_counter_when_pacman_got_killed(
    mut event_reader: EventReader<PacmanWasHit>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    for _ in event_reader.read() {
        ghost_house_gate.switch_to_global_counter()
    }
}

