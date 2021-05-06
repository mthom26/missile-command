use bevy::{prelude::*, utils::Duration};

mod powerup_spawner;
mod powerups;

pub use self::{
    powerup_spawner::{run_powerup_spawner, spawn_powerups, PowerupSpawner, SpawnPowerup},
    powerups::{check_offscreen_powerups, despawn_powerups},
};

use crate::state::GameState;

#[derive(Copy, Clone, Debug)]
pub enum PowerupType {
    Score,
    ExplosionSize,
    MissileSpeed,
}

pub struct PowerupsPlugin;
impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnPowerup>()
            .insert_resource(PowerupSpawner {
                timer: Timer::new(Duration::from_secs_f32(10.0), true),
            })
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_powerups.system())
                    .with_system(run_powerup_spawner.system())
                    .with_system(check_offscreen_powerups.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_powerups.system()),
            );
    }
}
