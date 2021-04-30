use bevy::prelude::*;

use crate::AssetHandles;

mod game_over;
mod main_menu;
mod options_menu;
mod pause_menu;
mod score_ui;

pub use self::{
    game_over::GameOverPlugin,
    main_menu::MainMenuPlugin,
    options_menu::OptionsMenuPlugin,
    pause_menu::PauseMenuPlugin,
    score_ui::{ScoreUiPlugin, UpdateScoreUi},
};

enum ButtonType {
    SetGame,
    SetMainMenu,
    SetOptions,
    PopState,
    Quit,
}

fn spawn_button(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    text_value: String,
    button_type: ButtonType,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                margin: Rect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: asset_handles.button_normal.clone(),
            ..Default::default()
        })
        .insert(button_type)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text_value,
                        style: TextStyle {
                            font: asset_handles.font.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
