use bevy::prelude::*;

use crate::{
    explosion::SpawnExplosion,
    line_trail::SpawnLineTrail,
    state::GameState,
    team::{EnemyTeam, PlayerTeam, Team},
    AssetHandles, Velocity, MISSILE_VELOCITY,
};

pub struct Missile;

// Position the missile should explode at if it doesn't hit anything
struct Target(Vec3);

// Spawn missile event
pub struct SpawnMissile {
    pub position: Vec3,
    pub target: Vec3,
    pub team: Team,
}

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnMissile>().add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(spawn_missiles.system())
                .with_system(check_target_reached.system()),
        );
    }
}

fn spawn_missiles(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnMissile>,
    mut line_events: EventWriter<SpawnLineTrail>,
) {
    for e in events.iter() {
        let a = Vec2::new(0.0, 1.0);
        let b = e.target - e.position;
        let b = Vec2::new(b.x, b.y);
        let angle = a.angle_between(b);

        let velocity = b.normalize() * MISSILE_VELOCITY;

        let missile_material = match e.team {
            Team::Player => asset_handles.missile_green.clone(),
            Team::Enemy => asset_handles.missile_red.clone(),
        };

        // let team = match e.team {
        //     Team::Player => PlayerTeam,
        //     Team::Enemy => EnemyTeam,
        // };
        // It seems like there isn't a way to conditionally choose which component
        // to insert without just duplicating the whole commands sequence...
        let missile_id = if e.team == Team::Player {
            commands
                .spawn_bundle(SpriteBundle {
                    material: missile_material,
                    transform: Transform {
                        translation: e.position,
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity(velocity))
                .insert(Target(e.target))
                .insert(PlayerTeam)
                .insert(Team::Player)
                .insert(Missile)
                .id()
        } else {
            commands
                .spawn_bundle(SpriteBundle {
                    material: missile_material,
                    transform: Transform {
                        translation: e.position,
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity(velocity))
                .insert(Target(e.target))
                .insert(EnemyTeam)
                .insert(Team::Enemy)
                .insert(Missile)
                .id()
        };

        // Spawn Line Trail
        // TODO - Send appropriate color
        line_events.send(SpawnLineTrail {
            position: e.position,
            rotation: angle,
            velocity,
            owner: missile_id,
            team: e.team,
        });
    }
}

fn check_target_reached(
    mut commands: Commands,
    mut events: EventWriter<SpawnExplosion>,
    query: Query<(Entity, &Transform, &Target, &Team)>,
) {
    for (entity, transform, target, team) in query.iter() {
        if transform.translation.distance_squared(target.0) < 10.0 {
            commands.entity(entity).despawn();
            events.send(SpawnExplosion {
                position: transform.translation,
                team: *team,
            });
        }
    }
}
