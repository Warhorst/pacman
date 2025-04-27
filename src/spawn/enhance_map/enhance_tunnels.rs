use bevy::prelude::*;

use crate::core::prelude::*;

pub(super) struct EnhanceTunnelPlugin;

impl Plugin for EnhanceTunnelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(SpawnMaze(EnhanceMap)),
                enhance_tunnels,
            )
        ;
    }
}

fn enhance_tunnels(
    mut commands: Commands,
    maps: Query<Entity, With<Map>>,
    tunnels: Query<(Entity, &Tunnel, &Tiles)>,
) -> Result {
    let map = maps.single()?;

    for (entity, tunnel, tiles) in &tunnels {
        let tunnel_transform = Transform::from_translation(tiles.to_vec3(TUNNEL_Z));
        let tunnel_entrance_transform = Transform::from_translation(tiles.to_pos().neighbour_in_direction(tunnel.direction.opposite()).to_vec3(TUNNEL_Z));

        commands
            .entity(entity)
            .insert((
                Sprite {
                    color: Color::srgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(TUNNEL_DIMENSION)),
                    ..default()
                },
                tunnel_transform
            ));

        let entrance = commands.spawn((
            Name::new("TunnelEntrance"),
            Sprite {
                color: Color::srgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(TUNNEL_DIMENSION)),
                ..default()
            },
            tunnel_entrance_transform,
        )).id();

        commands.entity(map).add_children(&[entrance]);
    }
    
    Ok(())
}