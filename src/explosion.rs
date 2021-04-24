use bevy::prelude::*;

use crate::{
    collision::CircleCollider,
    consts::EXPLOSION_SIZE,
    state::GameState,
    team::{EnemyTeam, PlayerTeam, Team},
    AssetHandles,
};

pub struct Explosion;

struct Size(f32);

// Event to spawn explosion
pub struct SpawnExplosion {
    pub position: Vec3,
    pub team: Team,
}

pub struct ExplosionPlugin;
impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnExplosion>().add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(spawn_explosions.system())
                .with_system(update_explosions.system()),
        );
    }
}

fn spawn_explosions(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnExplosion>,
) {
    for e in events.iter() {
        let explosion_material = match e.team {
            Team::Player => asset_handles.explosion_green.clone(),
            Team::Enemy => asset_handles.explosion_red.clone(),
        };

        if e.team == Team::Player {
            commands
                .spawn_bundle(SpriteBundle {
                    material: explosion_material,
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
                .insert(PlayerTeam)
                .insert(Size(1.0))
                .insert(CircleCollider(EXPLOSION_SIZE))
                .insert(Explosion);
        } else {
            commands
                .spawn_bundle(SpriteBundle {
                    material: explosion_material,
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
                .insert(EnemyTeam)
                .insert(Size(1.0))
                .insert(CircleCollider(EXPLOSION_SIZE))
                .insert(Explosion);
        }
    }
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Explosion,
        &mut Size,
        &mut Transform,
        &mut CircleCollider,
    )>,
) {
    for (entity, _, mut size, mut transform, mut collider) in query.iter_mut() {
        size.0 -= time.delta_seconds();
        transform.scale.x = size.0;
        transform.scale.y = size.0;
        collider.0 = EXPLOSION_SIZE * size.0;

        // Despawn explosion when it shrinks to zero size
        // TODO - Add easing so the explosion grows quickly then smoothly shrinks?
        if size.0 < 0.01 {
            commands.entity(entity).despawn();
        }
    }
}
