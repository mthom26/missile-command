use bevy::prelude::*;

use crate::{state::GameState, AssetHandles};

struct ScoreUi {
    score: usize,
}

struct ScoreUiText;

// Update score Event
pub struct UpdateScoreUi {
    pub value: usize,
}

pub struct ScoreUiPlugin;
impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UpdateScoreUi>()
            .add_system_set(
                SystemSet::on_enter(GameState::Game).with_system(setup_score_ui.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(update_score_ui.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn.system()));
    }
}

fn setup_score_ui(
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
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(ScoreUi { score: 0 })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font: asset_handles.font.clone(),
                                font_size: 28.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        ..Default::default()
                    },
                    style: Style {
                        margin: Rect {
                            top: Val::Px(15.0),
                            right: Val::Px(15.0),
                            bottom: Val::Px(15.0),
                            left: Val::Px(15.0),
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreUiText);
        });
}

fn update_score_ui(
    mut query: Query<&mut ScoreUi>,
    mut text_query: Query<(&ScoreUiText, &mut Text)>,
    mut events: EventReader<UpdateScoreUi>,
) {
    for e in events.iter() {
        let mut new_score = 0;

        for mut score_ui in query.iter_mut() {
            score_ui.score += e.value;
            new_score = score_ui.score;
        }

        for (_, mut text) in text_query.iter_mut() {
            text.sections[0].value = new_score.to_string();
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &ScoreUi)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
