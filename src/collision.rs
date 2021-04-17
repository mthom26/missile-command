use bevy::prelude::*;

use crate::{
    explosion::{Explosion, SpawnExplosion},
    missile::Missile,
    state::GameState,
    team::{EnemyTeam, PlayerTeam, Team},
};

pub struct CircleCollider(pub f32);

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(update_collisions.system())
                .with_system(missile_collisions.system()),
        );
    }
}

fn update_collisions(
    mut commands: Commands,
    player_explosions: Query<(&Explosion, &PlayerTeam, &CircleCollider, &Transform)>,
    enemy_missiles: Query<(Entity, &Missile, &EnemyTeam, &Transform)>,
) {
    for (_, _, collider, transform) in player_explosions.iter() {
        for (entity, _, _, missile_transform) in enemy_missiles.iter() {
            let d = transform
                .translation
                .distance_squared(missile_transform.translation);
            if d < collider.0.powi(2) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn missile_collisions(
    mut commands: Commands,
    player_missiles: Query<(Entity, &Missile, &PlayerTeam, &Transform)>,
    enemy_missiles: Query<(Entity, &Missile, &EnemyTeam, &Transform)>,
    mut events: EventWriter<SpawnExplosion>,
) {
    for (player_entity, _, _, player_transform) in player_missiles.iter() {
        for (enemy_entity, _, _, enemy_transform) in enemy_missiles.iter() {
            let d = player_transform
                .translation
                .distance_squared(enemy_transform.translation);

            // TODO - Give missiles a customisable collision radius instead of hardcoding it?
            if d < 7.0f32.powi(2) {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
                events.send(SpawnExplosion {
                    position: player_transform.translation,
                    team: Team::Player,
                });
            }
        }
    }
}
