use bevy::{prelude::*, utils::Duration};
use rand::prelude::*;

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
    windows: Res<Windows>,
    mut spawner: ResMut<EnemyMissileSpawner>,
    mut events: EventWriter<SpawnMissile>,
) {
    if spawner.timer.tick(time.delta()).finished() {
        let (half_width, half_height) = if let Some(window) = windows.get_primary() {
            (window.width() / 2.0, window.height() / 2.0)
        } else {
            panic!("Could not get primary window!");
        };
        let mut rng = thread_rng();
        let x_pos = rng.gen_range(-half_width..half_width);
        let x_tar = rng.gen_range(-half_width..half_width);

        events.send(SpawnMissile {
            position: Vec3::new(x_pos, half_height, 0.0),
            target: Vec3::new(x_tar, -half_height, 0.0),
            team: Team::Enemy,
        });
    }
}
