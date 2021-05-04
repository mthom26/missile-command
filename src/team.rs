/*-----------------------------------------------------------------------------
    The `PlayerTeam` and `EnemyTeam` components below exist to make collision
    querying easier. However some entities need to be able to pass info to
    spawn events (e.g. Missile spawning an explosion) and require both a
    `Team` and `PlayerTeam`/`EnemyTeam` component.
-----------------------------------------------------------------------------*/

#[derive(Clone, Copy, PartialEq)]
pub enum Team {
    Player,
    Enemy,
}

pub struct PlayerTeam;

pub struct EnemyTeam;
