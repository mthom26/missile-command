use bevy::prelude::*;

use crate::{state::GameState, ui::UpdateScoreUi};

// Event
pub struct UpdateScore(pub usize);

#[derive(Debug, Default)]
pub struct GameStatus {
    pub score: usize,
}

pub struct GameStatusPlugin;
impl Plugin for GameStatusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UpdateScore>()
            .init_resource::<GameStatus>()
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset.system()))
            .add_system(update_game_status.system());
    }
}

fn update_game_status(
    mut game_status: ResMut<GameStatus>,
    mut score_events: EventReader<UpdateScore>,
    mut score_ui_events: EventWriter<UpdateScoreUi>,
) {
    for e in score_events.iter() {
        game_status.score += e.0;
        score_ui_events.send(UpdateScoreUi(game_status.score));
    }
}

fn reset(mut game_status: ResMut<GameStatus>) {
    game_status.score = 0;
}
