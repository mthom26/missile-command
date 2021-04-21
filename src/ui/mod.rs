mod game_over;
mod main_menu;
mod score_ui;

pub use self::{
    game_over::GameOverPlugin,
    main_menu::MainMenuPlugin,
    score_ui::{ScoreUiPlugin, UpdateScoreUi},
};
