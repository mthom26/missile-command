use bevy::prelude::*;

use crate::{
    explosion::Explosion,
    missile::Missile,
    state::GameState,
    team::{EnemyTeam, PlayerTeam},
};

pub struct CircleCollider(pub f32);

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(update_collisions.system()),
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
