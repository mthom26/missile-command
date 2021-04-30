use bevy::{app::AppExit, prelude::*};

use crate::{audio::PlayAudio, state::GameState, AssetHandles};

use super::{score_ui::ScoreUi, spawn_button, ButtonType};

struct PauseMenuUi;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(update_menu.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(despawn.system()));
    }
}

fn setup_menu(
    mut commands: Commands,
    query: Query<(Entity, &ScoreUi)>,
    asset_handles: Res<AssetHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawning the pause menu as a child of the ScoreUi so the NodeBundle has
    // a higher z-order and the darkened material displays properly.
    for (entity, _) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.8).into()),
                    ..Default::default()
                })
                .insert(PauseMenuUi)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "PAUSED".to_string(),
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
                            spawn_button(
                                parent,
                                &asset_handles,
                                "PLAY".to_string(),
                                ButtonType::PopState,
                            );
                            spawn_button(
                                parent,
                                &asset_handles,
                                "MAIN MENU".to_string(),
                                ButtonType::SetMainMenu,
                            );
                            spawn_button(
                                parent,
                                &asset_handles,
                                "QUIT".to_string(),
                                ButtonType::Quit,
                            );
                        });
                });
        });
    }
}

fn update_menu(
    asset_handles: Res<AssetHandles>,
    mut query: Query<(&Interaction, &mut Handle<ColorMaterial>, &ButtonType), Changed<Interaction>>,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<AppExit>,
    mut audio_events: EventWriter<PlayAudio>,
) {
    for (interaction, mut material, button) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = asset_handles.button_click.clone();
                audio_events.send(PlayAudio {
                    handle: asset_handles.button_click_audio.clone(),
                });
                match button {
                    ButtonType::PopState => state.pop().unwrap(),
                    ButtonType::SetMainMenu => state.replace(GameState::MainMenu).unwrap(),
                    ButtonType::Quit => events.send(AppExit),
                    _ => eprintln!("Button should not exist here."),
                }
            }
            Interaction::Hovered => {
                *material = asset_handles.button_hover.clone();
                audio_events.send(PlayAudio {
                    handle: asset_handles.button_hover_audio.clone(),
                });
            }
            Interaction::None => *material = asset_handles.button_normal.clone(),
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &PauseMenuUi)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
