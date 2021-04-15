use bevy::prelude::*;

use crate::{explosion::SpawnExplosion, AssetHandles, Velocity, MISSILE_VELOCITY};

struct Missile;

// Position the missile should explode at if it doesn't hit anything
struct Target(Vec3);

// Spawn missile event
pub struct SpawnMissile {
    pub position: Vec3,
    pub target: Vec3,
}

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnMissile>()
            .add_system(spawn_missiles.system())
            .add_system(check_target_reached.system());
    }
}

fn spawn_missiles(
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

fn check_target_reached(
    mut commands: Commands,
    mut events: EventWriter<SpawnExplosion>,
    query: Query<(Entity, &Transform, &Target)>,
) {
    for (entity, transform, target) in query.iter() {
        if transform.translation.distance_squared(target.0) < 10.0 {
            commands.entity(entity).despawn();
            events.send(SpawnExplosion {
                position: transform.translation,
            });
        }
    }
}
