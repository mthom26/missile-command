use bevy::prelude::*;

use crate::{consts::EXPLOSION_SIZE_TIME, GameState};

// Event
pub struct SetPlayerExplosionSize(pub f32);

// Resource to store player missile velocity, explosion size, etc...
// TODO - Add ui indicators to show powerup status?
pub struct PlayerStatus {
    pub explosion_size: f32,
    pub explosion_timer: Timer,
}

impl PlayerStatus {
    fn reset(&mut self) {
        self.explosion_size = 1.0;
        self.explosion_timer = Timer::from_seconds(EXPLOSION_SIZE_TIME, false);
    }
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            explosion_size: 1.0,
            explosion_timer: Timer::from_seconds(EXPLOSION_SIZE_TIME, false),
        }
    }
}

pub struct PlayerStatusPlugin;
impl Plugin for PlayerStatusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SetPlayerExplosionSize>()
            .init_resource::<PlayerStatus>()
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(run_timers.system())
                    .with_system(handle_events.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(reset_player_status.system()),
            );
    }
}

fn run_timers(time: Res<Time>, mut player_status: ResMut<PlayerStatus>) {
    if player_status.explosion_timer.tick(time.delta()).finished() {
        player_status.explosion_size = 1.0;
    }
}

fn handle_events(
    mut player_status: ResMut<PlayerStatus>,
    mut explosion_size_events: EventReader<SetPlayerExplosionSize>,
) {
    for e in explosion_size_events.iter() {
        player_status.explosion_size = e.0;
        player_status.explosion_timer.reset();
    }
}

fn reset_player_status(mut player_status: ResMut<PlayerStatus>) {
    player_status.reset();
}
