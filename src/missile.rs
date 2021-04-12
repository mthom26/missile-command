use bevy::prelude::*;

use crate::{AssetHandles, Velocity, MISSILE_VELOCITY};

struct Missile;

// Position the missile should explode at if it doesn't hit anything
struct Target(Vec3);

// Spawn missile event
pub struct SpawnMissile {
    pub position: Vec3,
    pub target: Vec3,
}

pub fn spawn_missiles(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnMissile>,
) {
    for e in events.iter() {
        let a = Vec2::new(0.0, 1.0);
        let b = e.target - e.position;
        let b = Vec2::new(b.x, b.y);
        let angle = a.angle_between(b);

        let velocity = b.normalize() * MISSILE_VELOCITY;

        commands
            .spawn_bundle(SpriteBundle {
                material: asset_handles.missile_green.clone(),
                transform: Transform {
                    translation: e.position,
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Velocity(velocity))
            .insert(Target(e.target))
            .insert(Missile);
    }
}
