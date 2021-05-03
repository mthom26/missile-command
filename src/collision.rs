use bevy::prelude::*;

use crate::{
    audio::PlayAudio,
    consts::{
        EXPLOSION_SIZE_SCALE, MISSILE_HIT_VALUE, MISSILE_SPEED_BONUS, MISSILE_VALUE,
        PLAYER_MISSILE_VELOCITY, SCORE_POWERUP_VALUE,
    },
    debris::{DebrisType, SpawnDebris},
    explosion::{Explosion, SpawnExplosion},
    game_status::UpdateScore,
    missile::Missile,
    player_status::{PlayerStatus, SetPlayerExplosionSize, SetPlayerMissileSpeed},
    powerups::PowerupType,
    state::GameState,
    team::{EnemyTeam, PlayerTeam, Team},
    AssetHandles, Building, Silo,
};

pub struct CircleCollider(pub f32);

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(explosion_collisions.system())
                .with_system(missile_collisions.system())
                .with_system(enemy_missile_collisions.system())
                .with_system(missile_ground_collisions.system())
                .with_system(powerup_collisions.system()),
        );
    }
}

// Player explosions hit Enemy missiles and Powerups
fn explosion_collisions(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    player_explosions: Query<(&Explosion, &PlayerTeam, &CircleCollider, &Transform)>,
    enemy_missiles: Query<(Entity, &Missile, &EnemyTeam, &Transform)>,
    powerups: Query<(Entity, &PowerupType, &Transform, &CircleCollider)>,
    mut score_events: EventWriter<UpdateScore>,
    mut explosion_size_events: EventWriter<SetPlayerExplosionSize>,
    mut missile_speed_events: EventWriter<SetPlayerMissileSpeed>,
    mut audio_events: EventWriter<PlayAudio>,
) {
    for (_, _, p_collider, p_transform) in player_explosions.iter() {
        // TODO - Maybe merge the two queries into one?
        for (e_entity, _, _, e_transform) in enemy_missiles.iter() {
            let d = p_transform
                .translation
                .distance_squared(e_transform.translation);
            if d < p_collider.0.powi(2) {
                commands.entity(e_entity).despawn();
                score_events.send(UpdateScore(MISSILE_VALUE));
            }
        }

        for (pow_entity, pow_type, pow_transform, pow_collider) in powerups.iter() {
            let d = p_transform
                .translation
                .distance_squared(pow_transform.translation);

            if d < (p_collider.0 + pow_collider.0).powi(2) {
                commands.entity(pow_entity).despawn();

                match pow_type {
                    PowerupType::Score => score_events.send(UpdateScore(SCORE_POWERUP_VALUE)),
                    PowerupType::ExplosionSize => {
                        explosion_size_events.send(SetPlayerExplosionSize(EXPLOSION_SIZE_SCALE))
                    }
                    PowerupType::MissileSpeed => missile_speed_events.send(SetPlayerMissileSpeed(
                        PLAYER_MISSILE_VELOCITY * MISSILE_SPEED_BONUS,
                    )),
                };

                audio_events.send(PlayAudio {
                    handle: asset_handles.powerup_audio.clone(),
                });
            }
        }
    }
}

// Player missiles hit Enemy missiles
fn missile_collisions(
    mut commands: Commands,
    player_status: Res<PlayerStatus>,
    player_missiles: Query<(Entity, &Missile, &PlayerTeam, &Transform, &CircleCollider)>,
    enemy_missiles: Query<(Entity, &Missile, &EnemyTeam, &Transform, &CircleCollider)>,
    mut events: EventWriter<SpawnExplosion>,
    mut score_events: EventWriter<UpdateScore>,
) {
    for (p_entity, _, _, p_transform, p_collider) in player_missiles.iter() {
        for (e_entity, _, _, e_transform, e_collider) in enemy_missiles.iter() {
            let d = p_transform
                .translation
                .distance_squared(e_transform.translation);

            if d < (p_collider.0 + e_collider.0).powi(2) {
                commands.entity(p_entity).despawn();
                commands.entity(e_entity).despawn();
                events.send(SpawnExplosion {
                    position: p_transform.translation,
                    team: Team::Player,
                    size: player_status.explosion_size,
                });
                score_events.send(UpdateScore(MISSILE_HIT_VALUE));
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
    mut debris_events: EventWriter<SpawnDebris>,
) {
    for (missile, _, _, missile_transform) in enemy_missiles.iter() {
        let m_x = missile_transform.translation.x;
        let m_y = missile_transform.translation.y;

        for (structure_entity, structure_transform, b) in player_structures.iter() {
            let s_x = structure_transform.translation.x;
            let s_y = structure_transform.translation.y;

            if b.is_some() {
                // Hit a building
                let half_width = 16.0;
                let half_height = 20.0;
                let y_offset = -12.0;

                if check_collision(m_x, m_y, s_x, s_y, half_width, half_height, y_offset) {
                    commands.entity(missile).despawn();
                    commands.entity(structure_entity).despawn();
                    events.send(SpawnExplosion {
                        position: missile_transform.translation,
                        team: Team::Enemy,
                        size: 1.0,
                    });
                    debris_events.send(SpawnDebris {
                        x_position: structure_transform.translation.x,
                        debris_type: DebrisType::Building,
                    });
                }
            } else {
                // Hit a silo
                // TODO - Also need to despawn the corresponding SiloReloadUi entity
                //        Maybe just spawn SiloReloadUi as a child of the silo?
                let half_width = 32.0;
                let half_height = 16.0;
                let y_offset = 0.0;

                if check_collision(m_x, m_y, s_x, s_y, half_width, half_height, y_offset) {
                    commands.entity(missile).despawn();
                    commands.entity(structure_entity).despawn();
                    events.send(SpawnExplosion {
                        position: missile_transform.translation,
                        team: Team::Enemy,
                        size: 1.0,
                    });
                    debris_events.send(SpawnDebris {
                        x_position: structure_transform.translation.x,
                        debris_type: DebrisType::Silo,
                    });
                }
            }
        }
    }
}

fn missile_ground_collisions(
    mut commands: Commands,
    player_status: Res<PlayerStatus>,
    missiles: Query<(Entity, &Missile, &Transform, &Team)>,
    mut events: EventWriter<SpawnExplosion>,
) {
    for (entity, _, transform, team) in missiles.iter() {
        if transform.translation.y < -296.0 {
            commands.entity(entity).despawn();
            events.send(SpawnExplosion {
                position: transform.translation,
                team: *team,
                size: match team {
                    Team::Player => player_status.explosion_size,
                    Team::Enemy => 1.0,
                },
            })
        }
    }
}

// Player missiles hit Powerups
fn powerup_collisions(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    player_status: Res<PlayerStatus>,
    missiles: Query<(Entity, &Missile, &Transform, &Team)>,
    powerups: Query<(Entity, &PowerupType, &Transform, &CircleCollider)>,
    mut events: EventWriter<SpawnExplosion>,
    mut score_events: EventWriter<UpdateScore>,
    mut explosion_size_events: EventWriter<SetPlayerExplosionSize>,
    mut missile_speed_events: EventWriter<SetPlayerMissileSpeed>,
    mut audio_events: EventWriter<PlayAudio>,
) {
    for (m_entity, _, m_transform, m_team) in missiles.iter() {
        for (p_entity, p_type, p_transform, p_collider) in powerups.iter() {
            if *m_team == Team::Player {
                let distance = m_transform
                    .translation
                    .distance_squared(p_transform.translation);

                if distance < p_collider.0.powi(2) {
                    commands.entity(m_entity).despawn();
                    commands.entity(p_entity).despawn();

                    events.send(SpawnExplosion {
                        position: m_transform.translation,
                        team: Team::Player,
                        size: player_status.explosion_size,
                    });

                    match p_type {
                        PowerupType::Score => score_events.send(UpdateScore(SCORE_POWERUP_VALUE)),
                        PowerupType::ExplosionSize => {
                            explosion_size_events.send(SetPlayerExplosionSize(EXPLOSION_SIZE_SCALE))
                        }
                        PowerupType::MissileSpeed => missile_speed_events.send(
                            SetPlayerMissileSpeed(PLAYER_MISSILE_VELOCITY * MISSILE_SPEED_BONUS),
                        ),
                    };

                    audio_events.send(PlayAudio {
                        handle: asset_handles.powerup_audio.clone(),
                    });
                }
            }
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
