use bevy::{prelude::*, utils::Duration};

use crate::state::GameState;

mod enemy_events;
mod enemy_spawner;

use self::{
    enemy_events::multiple_missiles,
    enemy_spawner::{update_timer, EnemyMissileSpawner},
};

pub struct EnemySpawnerPlugin;
impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(EnemyMissileSpawner {
            timer: Timer::new(Duration::from_secs_f32(3.0), true),
            enemy_event_timer: Timer::new(Duration::from_secs_f32(12.0), true),
        })
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_timer.system()));
    }
}
