use bevy::{app::AppExit, prelude::*};

use crate::{AssetHandles, GameState};

enum GameOverButton {
    MainMenu,
    Quit,
}

struct GameOverUi;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(setup_game_over.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(update_game_over.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(despawn.system()));
    }
}

fn setup_game_over(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(GameOverUi)
        .with_children(|parent| {
            // Game Over text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "GAME OVER".to_string(),
                        style: TextStyle {
                            font: asset_handles.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            // Score text
            // TODO - Display score achieved in game
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "SCORE: 1337".to_string(),
                        style: TextStyle {
                            font: asset_handles.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            // Container for buttons
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Main Menu button
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
                        .insert(GameOverButton::MainMenu)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "MAIN MENU".to_string(),
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
                    // Quit button
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
                        .insert(GameOverButton::Quit)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "QUIT".to_string(),
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
                });
        });
}

fn update_game_over(
    asset_handles: Res<AssetHandles>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &GameOverButton),
        Changed<Interaction>,
    >,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<AppExit>,
) {
    for (interaction, mut material, button) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = asset_handles.button_click.clone();
                match button {
                    GameOverButton::MainMenu => state.set(GameState::MainMenu).unwrap(),
                    GameOverButton::Quit => events.send(AppExit),
                }
            }
            Interaction::Hovered => *material = asset_handles.button_hover.clone(),
            Interaction::None => *material = asset_handles.button_normal.clone(),
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &GameOverUi)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
