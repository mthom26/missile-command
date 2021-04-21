use bevy::prelude::*;

use crate::{AssetHandles, GameState};

#[derive(Debug, Clone, Copy)]
pub enum DebrisType {
    Building,
    Silo,
}

pub struct SpawnDebris {
    pub x_position: f32,
    pub debris_type: DebrisType,
}

pub struct DebrisPlugin;
impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnDebris>()
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(spawn_debris.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn.system()));
    }
}

fn spawn_debris(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut events: EventReader<SpawnDebris>,
) {
    for e in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: match e.debris_type {
                    DebrisType::Building => asset_handles.debris_01.clone(),
                    DebrisType::Silo => asset_handles.silo_debris_01.clone(),
                },
                transform: Transform {
                    translation: Vec3::new(e.x_position, -280.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(e.debris_type);
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &DebrisType)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}
