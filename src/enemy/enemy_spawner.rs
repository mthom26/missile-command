use bevy::prelude::*;
use rand::prelude::*;

use crate::{missile::SpawnMissile, team::Team};

use super::multiple_missiles;

pub struct EnemyMissileSpawner {
    pub timer: Timer,
    pub enemy_event_timer: Timer,
}

pub fn update_timer(
    time: Res<Time>,
    windows: Res<Windows>,
    mut spawner: ResMut<EnemyMissileSpawner>,
    mut events: EventWriter<SpawnMissile>,
) {
    let (half_width, half_height) = if let Some(window) = windows.get_primary() {
        (window.width() / 2.0, window.height() / 2.0)
    } else {
        panic!("Could not get primary window!");
    };
    let mut rng = thread_rng();

    if spawner.timer.tick(time.delta()).finished() {
        let x_pos = rng.gen_range(-half_width..half_width);
        let x_tar = rng.gen_range(-half_width..half_width);

        events.send(SpawnMissile {
            position: Vec3::new(x_pos, half_height, 0.0),
            target: Vec3::new(x_tar, -half_height, 0.0),
            team: Team::Enemy,
        });
    }

    if spawner.enemy_event_timer.tick(time.delta()).finished() {
        let to_spawn = multiple_missiles(half_width, half_height, &mut rng);
        events.send_batch(to_spawn.into_iter());
    }
}
