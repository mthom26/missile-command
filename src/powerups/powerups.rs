use bevy::prelude::*;

use crate::collision::CircleCollider;

use super::PowerupType;

pub fn check_offscreen_powerups(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform, &PowerupType, &CircleCollider)>,
) {
    let half_width = windows.get_primary().unwrap().width() / 2.0;

    for (entity, transform, _, collider) in query.iter() {
        if transform.translation.x > half_width + collider.0
            || transform.translation.x < -half_width - collider.0
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_powerups(mut commands: Commands, query: Query<(Entity, &PowerupType)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}
