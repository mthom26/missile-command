use bevy::{app::AppExit, prelude::*};

use crate::{AssetHandles, GameState};

use super::{spawn_button, ButtonType};

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
                    spawn_button(
                        parent,
                        &asset_handles,
                        "MAIN MENU".to_string(),
                        ButtonType::SetMainMenu,
                    );
                    // Quit button
                    spawn_button(parent, &asset_handles, "QUIT".to_string(), ButtonType::Quit);
                });
        });
}

fn update_game_over(
    asset_handles: Res<AssetHandles>,
    mut query: Query<(&Interaction, &mut Handle<ColorMaterial>, &ButtonType), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<AppExit>,
) {
    for (interaction, mut material, button) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = asset_handles.button_click.clone();
                match button {
                    ButtonType::SetMainMenu => state.set(GameState::MainMenu).unwrap(),
                    ButtonType::Quit => events.send(AppExit),
                    _ => eprintln!("Button should not exist here."),
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
