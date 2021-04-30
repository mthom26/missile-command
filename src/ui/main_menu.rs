use bevy::{app::AppExit, prelude::*};

use crate::{audio::PlayAudio, state::GameState, AssetHandles};

use super::{spawn_button, ButtonType};

struct MainMenuUi;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update_menu.system()))
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn.system()));
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
                    spawn_button(
                        parent,
                        &asset_handles,
                        "PLAY".to_string(),
                        ButtonType::SetGame,
                    );
                    // Options button
                    spawn_button(
                        parent,
                        &asset_handles,
                        "OPTIONS".to_string(),
                        ButtonType::SetOptions,
                    );
                    // Quit button
                    spawn_button(parent, &asset_handles, "QUIT".to_string(), ButtonType::Quit);
                });
        });
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
                    ButtonType::SetGame => state.set(GameState::Game).unwrap(),
                    ButtonType::SetOptions => state.set(GameState::OptionsMenu).unwrap(),
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

fn despawn(mut commands: Commands, query: Query<(Entity, &MainMenuUi)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
