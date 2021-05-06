use bevy::prelude::*;
use rand::prelude::*;

use crate::{missile::SpawnMissile, team::Team};

// TODO - Add more events and choose one at random

pub fn multiple_missiles(
    half_width: f32,
    half_height: f32,
    rng: &mut ThreadRng,
) -> Vec<SpawnMissile> {
    let mut to_spawn = vec![];

    for _ in 0..3 {
        let x_pos = rng.gen_range(-half_width..half_width);
        let x_tar = rng.gen_range(-half_width..half_width);

        to_spawn.push(SpawnMissile {
            position: Vec3::new(x_pos, half_height, 0.0),
            target: Vec3::new(x_tar, -half_height, 0.0),
            team: Team::Enemy,
        });
    }

    to_spawn
}
