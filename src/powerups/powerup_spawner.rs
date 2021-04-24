use bevy::prelude::*;
use rand::prelude::*;

use super::PowerupType;
use crate::{AssetHandles, Velocity};

pub struct PowerupSpawner {
    pub timer: Timer,
}

// Spawn Powerup Event
pub struct SpawnPowerup {
    powerup_type: PowerupType,
    position: Vec3,
    velocity: Vec2,
}

pub fn run_powerup_spawner(
    time: Res<Time>,
    windows: Res<Windows>,
    mut spawner: ResMut<PowerupSpawner>,
    mut events: EventWriter<SpawnPowerup>,
) {
    if spawner.timer.tick(time.delta()).finished() {
        let (half_width, half_height) = (
            windows.get_primary().unwrap().width() / 2.0,
            windows.get_primary().unwrap().height() / 2.0,
        );

        let mut rng = thread_rng();
        let y = rng.gen_range(0.0..half_height);
        let x = match rng.gen_bool(0.5) {
            // TODO - Add powerup width to this so the whole sprite is offscreen
            true => -half_width,
            false => half_width,
        };

        let velocity = match x {
            _ if x < 0.0 => Vec2::new(100.0, 0.0),
            _ if x >= 0.0 => Vec2::new(-100.0, 0.0),
            _ => panic!("Error getting powerup velocity."),
        };

        events.send(SpawnPowerup {
            powerup_type: PowerupType::Score,
            position: Vec3::new(x, y, 0.0),
            velocity,
        });
    }
}

pub fn spawn_powerups(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut events: EventReader<SpawnPowerup>,
) {
    for e in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: match e.powerup_type {
                    PowerupType::Score => asset_handles.score_powerup.clone(),
                },
                transform: Transform {
                    translation: e.position,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(e.powerup_type)
            .insert(Velocity(e.velocity));
    }
}
