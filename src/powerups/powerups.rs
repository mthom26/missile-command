use bevy::prelude::*;

use super::PowerupType;

pub fn check_offscreen_powerups(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform, &PowerupType)>,
) {
    let half_width = windows.get_primary().unwrap().width() / 2.0;

    for (entity, transform, _) in query.iter() {
        // TODO - Add powerup width to this check so the whole sprite is offscreen
        if transform.translation.x > half_width || transform.translation.x < -half_width {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_powerups(mut commands: Commands, query: Query<(Entity, &PowerupType)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}
