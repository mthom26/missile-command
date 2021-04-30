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

pub struct SiloMissileCountUi;

// Event
pub struct SiloMissileCountUpdate {
    pub location: SiloLocation,
    pub count: u8,
}

pub struct SiloPlugin;
impl Plugin for SiloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SiloMissileCountUpdate>().add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(check_timers.system())
                .with_system(update_reload_ui.system())
                .with_system(update_missile_count_ui.system()),
        );
    }
}

fn check_timers(
    time: Res<Time>,
    mut query: Query<(&mut Silo, &mut Timer)>,
    mut events: EventWriter<SiloMissileCountUpdate>,
) {
    for (mut silo, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            silo.missiles += 1;
            if silo.missiles < SILO_MAX_MISSILES {
                timer.reset();
            }
            events.send(SiloMissileCountUpdate {
                location: silo.location,
                count: silo.missiles,
            });
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

fn update_missile_count_ui(
    query: Query<(&SiloMissileCountUi, &SiloLocation, &Children)>,
    mut visible_query: Query<&mut Visible>,
    mut events: EventReader<SiloMissileCountUpdate>,
) {
    for e in events.iter() {
        for (_, location, children) in query.iter() {
            if *location == e.location {
                match e.count {
                    0 => {
                        visible_query.get_mut(children[0]).unwrap().is_visible = false;
                        visible_query.get_mut(children[1]).unwrap().is_visible = false;
                        visible_query.get_mut(children[2]).unwrap().is_visible = false;
                    }
                    1 => {
                        visible_query.get_mut(children[0]).unwrap().is_visible = true;
                        visible_query.get_mut(children[1]).unwrap().is_visible = false;
                        visible_query.get_mut(children[2]).unwrap().is_visible = false;
                    }
                    2 => {
                        visible_query.get_mut(children[0]).unwrap().is_visible = true;
                        visible_query.get_mut(children[1]).unwrap().is_visible = true;
                        visible_query.get_mut(children[2]).unwrap().is_visible = false;
                    }
                    3 => {
                        visible_query.get_mut(children[0]).unwrap().is_visible = true;
                        visible_query.get_mut(children[1]).unwrap().is_visible = true;
                        visible_query.get_mut(children[2]).unwrap().is_visible = true;
                    }
                    _ => {}
                }
            }
        }
    }
}
