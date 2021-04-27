mod game_over;
mod main_menu;
mod options_menu;
mod score_ui;

pub use self::{
    game_over::GameOverPlugin,
    main_menu::MainMenuPlugin,
    options_menu::OptionsMenuPlugin,
    score_ui::{ScoreUiPlugin, UpdateScoreUi},
};
