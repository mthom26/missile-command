use bevy::prelude::*;

use crate::{
    explosion::{Explosion, SpawnExplosion},
    missile::Missile,
    state::GameState,
    team::{EnemyTeam, PlayerTeam, Team},
    Building, Silo,
};

pub struct CircleCollider(pub f32);

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(update_collisions.system())
                .with_system(missile_collisions.system())
                .with_system(enemy_missile_collisions.system())
                .with_system(missile_ground_collisions.system()),
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

// Detect collisions between enemy missiles and player buildings/silos
// TODO - Stop hardcoding values like 'half_width, etc...'. Should probably
//        just make a collision component that stores them
fn enemy_missile_collisions(
    mut commands: Commands,
    enemy_missiles: Query<(Entity, &Missile, &EnemyTeam, &Transform)>,
    player_structures: Query<
        (Entity, &Transform, Option<&Building>),
        Or<(With<Building>, With<Silo>)>,
    >,
    mut events: EventWriter<SpawnExplosion>,
) {
    for (missile, _, _, missile_transform) in enemy_missiles.iter() {
        let m_x = missile_transform.translation.x;
        let m_y = missile_transform.translation.y;

        for (_, structure_transform, b) in player_structures.iter() {
            let s_x = structure_transform.translation.x;
            let s_y = structure_transform.translation.y;

            if b.is_some() {
                // Hit a building
                let half_width = 16.0;
                let half_height = 20.0;
                let y_offset = -12.0;

                if check_collision(m_x, m_y, s_x, s_y, half_width, half_height, y_offset) {
                    commands.entity(missile).despawn();
                    events.send(SpawnExplosion {
                        position: missile_transform.translation,
                        team: Team::Enemy,
                    });
                }
            } else {
                // Hit a silo
                let half_width = 32.0;
                let half_height = 16.0;
                let y_offset = 0.0;

                if check_collision(m_x, m_y, s_x, s_y, half_width, half_height, y_offset) {
                    commands.entity(missile).despawn();
                    events.send(SpawnExplosion {
                        position: missile_transform.translation,
                        team: Team::Enemy,
                    });
                }
            }
        }
    }
}

fn missile_ground_collisions(
    mut commands: Commands,
    missiles: Query<(Entity, &Missile, &Transform, &Team)>,
    mut events: EventWriter<SpawnExplosion>,
) {
    for (entity, _, transform, team) in missiles.iter() {
        if transform.translation.y < -296.0 {
            commands.entity(entity).despawn();
            events.send(SpawnExplosion {
                position: transform.translation,
                team: *team,
            })
        }
    }
}

fn check_collision(
    missile_x: f32,
    missile_y: f32,
    structure_x: f32,
    structure_y: f32,
    half_width: f32,
    half_height: f32,
    y_offset: f32,
) -> bool {
    missile_x > structure_x - half_width
        && missile_x < structure_x + half_width
        && missile_y > structure_y + y_offset - half_height
        && missile_y < structure_y + y_offset + half_height
}
