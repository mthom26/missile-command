use bevy::prelude::*;

use crate::{state::GameState, AssetHandles};

enum MainMenuButton {
    Play,
    Quit,
}

struct MainMenuUi;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu.system()),
        );
    }
}

fn setup_menu(
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
        .insert(MainMenuUi)
        .with_children(|parent| {
            // Missile Command text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "MISSILE COMMAND".to_string(),
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
                        // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        display: Display::Flex,
                        // padding: Rect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Play button
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
                        .insert(MainMenuButton::Play)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "PLAY".to_string(),
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
                        .insert(MainMenuButton::Quit)
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
