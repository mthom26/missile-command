use bevy::prelude::*;

use crate::{state::GameState, AssetHandles, SILO_MAX_MISSILES};

pub struct Silo {
    pub location: SiloLocation,
    pub missiles: u8,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SiloLocation {
    Left,
    Middle,
    Right,
}

pub struct SiloReloadUi;

pub struct SiloPlugin;
impl Plugin for SiloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(check_timers.system())
                .with_system(update_reload_ui.system()),
        );
    }
}

fn check_timers(time: Res<Time>, mut query: Query<(&mut Silo, &mut Timer)>) {
    for (mut silo, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            silo.missiles += 1;
            if silo.missiles < SILO_MAX_MISSILES {
                timer.reset();
            }
        }
    }
}

fn update_reload_ui(
    asset_handles: Res<AssetHandles>,
    query: Query<(&Silo, &Timer, &Transform), Without<SiloReloadUi>>,
    mut ui_query: Query<(
        &SiloReloadUi,
        &SiloLocation,
        &mut Transform,
        &mut Handle<ColorMaterial>,
    )>,
) {
    for (silo, timer, silo_transform) in query.iter() {
        for (_, location, mut ui_transform, mut mat) in ui_query.iter_mut() {
            if silo.location == *location {
                ui_transform.translation.x =
                    silo_transform.translation.x - 25.0 * timer.percent_left();
                ui_transform.scale.x = timer.percent();
                // Changing the material every frame probably isn't necessary here,
                // can probably move this into the check `check_timers` system and
                // only trigger the material change when needed
                if timer.percent() == 1.0 {
                    *mat = asset_handles.silo_reload_ready.clone();
                } else {
                    *mat = asset_handles.silo_reload_loading.clone();
                }
            }
        }
    }
}
