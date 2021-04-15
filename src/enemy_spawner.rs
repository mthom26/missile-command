use bevy::{prelude::*, utils::Duration};

use crate::{missile::SpawnMissile, team::Team};

struct EnemyMissileSpawner {
    timer: Timer,
}

pub struct EnemySpawnerPlugin;
impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(EnemyMissileSpawner {
            timer: Timer::new(Duration::from_secs_f32(2.0), true),
        })
        .add_system(update_timer.system());
    }
}

fn update_timer(
    time: Res<Time>,
    mut spawner: ResMut<EnemyMissileSpawner>,
    mut events: EventWriter<SpawnMissile>,
) {
    if spawner.timer.tick(time.delta()).finished() {
        events.send(SpawnMissile {
            position: Vec3::new(0.0, 300.0, 0.0),
            target: Vec3::new(0.0, -300.0, 0.0),
            team: Team::Enemy,
        });
    }
}
