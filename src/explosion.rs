use bevy::prelude::*;

use crate::AssetHandles;

struct Explosion;

// Event to spawn explosion
pub struct SpawnExplosion {
    pub position: Vec3,
}

pub struct ExplosionPlugin;
impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnExplosion>()
            .add_system(spawn_explosions.system())
            .add_system(update_explosions.system());
    }
}

fn spawn_explosions(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnExplosion>,
) {
    for e in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: asset_handles.explosion_green.clone(),
                transform: Transform {
                    translation: e.position,
                    ..Default::default()
                },
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                ..Default::default()
            })
            .insert(Explosion);
    }
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Explosion, &mut Transform)>,
) {
    for (entity, _, mut transform) in query.iter_mut() {
        transform.scale.x -= time.delta_seconds();
        transform.scale.y -= time.delta_seconds();

        // Despawn explosion when it shrinks to zero size
        // TODO - Add easing so the explosion grows quickly then smoothly shrinks?
        if transform.scale.x < 0.01 {
            commands.entity(entity).despawn();
        }
    }
}
