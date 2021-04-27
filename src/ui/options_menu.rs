use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{actions::ActionsMap, AssetHandles, GameState};

struct OptionsMenuUi;

enum OptionsMenuButton {
    MainMenu,
}

// Event to update button text after RebindWidget runs
struct UpdateRebindButtonText {
    entity: Entity,
    new_keycode: KeyCode,
}

struct RebindButton {
    action: String,
    keycode: KeyCode,
}

// Child of `RebindButton`. Just giving the child the parent entity seems
// easier than messing around trying to get the child in the query.
struct RebindButtonChild {
    parent_entity: Entity,
}

// The widget that displays when a rebind is clicked on
struct RebindWidget {
    action: String,
    button_entity: Entity,
    previous_keycode: KeyCode,
}

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UpdateRebindButtonText>()
            .add_system_set(
                SystemSet::on_enter(GameState::OptionsMenu).with_system(setup_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::OptionsMenu)
                    .with_system(update_menu.system())
                    .with_system(run_rebind_widget.system())
                    .with_system(update_button_text.system())
                    .with_system(update_rebind_items.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::OptionsMenu).with_system(despawn.system()),
            );
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    action_map: Res<ActionsMap>,
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
        .insert(OptionsMenuUi)
        .with_children(|parent| {
            // Options Menu text
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "REBIND KEYS".to_string(),
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
                    material: materials.add(Color::rgb(0.2, 0.2, 0.3).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let actions = action_map
                        .keyboard
                        .iter()
                        .map(|(keycode, action)| (action.clone(), *keycode))
                        .collect::<Vec<(String, KeyCode)>>();

                    for (action, keycode) in actions.iter() {
                        spawn_rebind_item(parent, &asset_handles, action, keycode, &mut materials);
                    }
                });

            // Main Menu button
            spawn_button(
                parent,
                &asset_handles,
                "MAIN MENU".to_string(),
                OptionsMenuButton::MainMenu,
            );
        });
}

fn update_menu(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &OptionsMenuButton),
        Changed<Interaction>,
    >,
    mut state: ResMut<State<GameState>>,
) {
    // MainMenu button
    for (interaction, mut material, button) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = asset_handles.button_click.clone();
                match button {
                    OptionsMenuButton::MainMenu => state.set(GameState::MainMenu).unwrap(),
                }
            }
            Interaction::Hovered => *material = asset_handles.button_hover.clone(),
            Interaction::None => *material = asset_handles.button_normal.clone(),
        }
    }
}

fn update_rebind_items(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut rebind_query: Query<
        (
            Entity,
            &Interaction,
            &RebindButton,
            &mut Handle<ColorMaterial>,
        ),
        Changed<Interaction>,
    >,
) {
    // Rebind buttons
    for (entity, interaction, button, mut material) in rebind_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = asset_handles.button_click.clone();
                spawn_rebind_widget(
                    &mut commands,
                    &button.action,
                    entity,
                    &asset_handles,
                    button.keycode,
                );
            }
            Interaction::Hovered => {
                *material = asset_handles.button_hover.clone();
            }
            Interaction::None => {
                *material = asset_handles.button_normal.clone();
            }
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &OptionsMenuUi)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_button(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    text_value: String,
    button_type: OptionsMenuButton,
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

fn spawn_rebind_widget(
    commands: &mut Commands,
    action: &str,
    button_entity: Entity,
    asset_handles: &AssetHandles,
    previous_keycode: KeyCode,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(100.0)),
                ..Default::default()
            },
            material: asset_handles.rebind_widget.clone(),
            ..Default::default()
        })
        .insert(RebindWidget {
            action: action.to_string(),
            button_entity,
            previous_keycode,
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("Rebind {}", action),
                        style: TextStyle {
                            font: asset_handles.simple_font.clone(),
                            font_size: 24.0,
                            ..Default::default()
                        },
                    }],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}

fn run_rebind_widget(
    mut commands: Commands,
    mut events: EventWriter<UpdateRebindButtonText>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut action_map: ResMut<ActionsMap>,
    query: Query<(Entity, &RebindWidget)>,
) {
    // This assumes there is only one RebindWidget
    // and one keyboard input
    for (entity, rebind_widget) in query.iter() {
        for e in keyboard_events.iter() {
            action_map.update_action(
                &rebind_widget.action,
                rebind_widget.previous_keycode,
                e.key_code.unwrap(),
            );

            events.send(UpdateRebindButtonText {
                entity: rebind_widget.button_entity,
                new_keycode: e.key_code.unwrap(),
            });

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_button_text(
    mut query: Query<(&RebindButtonChild, &mut Text)>,
    mut events: EventReader<UpdateRebindButtonText>,
) {
    for e in events.iter() {
        for (rbt, mut text) in query.iter_mut() {
            if rbt.parent_entity == e.entity {
                text.sections[0].value = format!("{:?}", e.new_keycode);
            }
        }
    }
}

fn spawn_rebind_item(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    action: &str,
    keycode: &KeyCode,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Px(50.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: Rect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.4, 0.1, 0.1, 1.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Action text
            spawn_action_text(parent, asset_handles, action, materials);
            // Rebind Button
            spawn_rebind_button(parent, asset_handles, action, keycode, materials);
        });
}

fn spawn_action_text(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    action: &str,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                margin: Rect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.1, 0.1, 0.8, 1.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: action.to_string(),
                        style: TextStyle {
                            font: asset_handles.simple_font.clone(),
                            font_size: 18.0,
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn spawn_rebind_button(
    parent: &mut ChildBuilder,
    asset_handles: &AssetHandles,
    action: &str,
    keycode: &KeyCode,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: Rect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.8, 0.1, 0.8, 1.0).into()),
            ..Default::default()
        })
        .insert(RebindButton {
            action: action.to_string(),
            keycode: *keycode,
        })
        .with_children(|parent| {
            let parent_entity = parent.parent_entity();

            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{:?}", keycode),
                            style: TextStyle {
                                font: asset_handles.simple_font.clone(),
                                font_size: 18.0,
                                ..Default::default()
                            },
                        }],
                        alignment: TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .insert(RebindButtonChild { parent_entity });
        });
}
