use bevy::{prelude::*, render::pipeline::PipelineDescriptor, utils::Duration};
use bevy_kira_audio::{AudioPlugin as KiraAudioPlugin, AudioSource};
use rand::prelude::*;

mod actions;
mod audio;
mod collision;
mod consts;
mod debris;
mod enemy;
mod explosion;
mod game_status;
mod line_trail;
mod missile;
mod player_status;
mod powerups;
mod silo;
mod state;
mod team;
mod ui;

use actions::{Actions, ActionsPlugin};
use audio::AudioPlugin;
use collision::CollisionPlugin;
use consts::{SILO_MAX_MISSILES, SILO_RELOAD_TIME};
use debris::{DebrisPlugin, DebrisType};
use enemy::EnemySpawnerPlugin;
use explosion::{Explosion, ExplosionPlugin};
use game_status::GameStatusPlugin;
use line_trail::{LineMaterial, LineTrail, LineTrailPlugin};
use missile::{Missile, MissilePlugin, SpawnMissile};
use player_status::PlayerStatusPlugin;
use powerups::PowerupsPlugin;
use silo::{
    Silo, SiloLocation, SiloMissileCountUi, SiloMissileCountUpdate, SiloPlugin, SiloReloadUi,
};
use state::GameState;
use team::Team;
use ui::{GameOverPlugin, MainMenuPlugin, OptionsMenuPlugin, PauseMenuPlugin, ScoreUiPlugin};

struct Building;

struct Ground;

struct Velocity(Vec2);

#[derive(Default)]
struct MousePosition {
    position: Vec2,
}

#[derive(Default)]
pub struct AssetHandles {
    // Menu
    pub button_normal: Handle<ColorMaterial>,
    pub button_hover: Handle<ColorMaterial>,
    pub button_click: Handle<ColorMaterial>,
    pub font: Handle<Font>,
    pub simple_font: Handle<Font>,

    // Rebind Ui
    pub rebind_widget: Handle<ColorMaterial>,

    // Game
    pub missile_red: Handle<ColorMaterial>,
    pub missile_green: Handle<ColorMaterial>,
    pub explosion_red: Handle<ColorMaterial>,
    pub explosion_green: Handle<ColorMaterial>,
    pub building_01: Handle<ColorMaterial>,
    pub building_02: Handle<ColorMaterial>,
    pub building_03: Handle<ColorMaterial>,
    pub ground: Handle<ColorMaterial>,
    pub silo: Handle<ColorMaterial>,
    pub debris_01: Handle<ColorMaterial>,
    pub silo_debris_01: Handle<ColorMaterial>,
    pub silo_reload_loading: Handle<ColorMaterial>,
    pub silo_reload_ready: Handle<ColorMaterial>,

    // Powerups
    pub score_powerup: Handle<ColorMaterial>,
    pub explosion_size_powerup: Handle<ColorMaterial>,
    pub missile_speed_powerup: Handle<ColorMaterial>,

    // Line Trail
    pub line_trail: Handle<Mesh>,
    pub line_trail_pipeline: Handle<PipelineDescriptor>,
    pub player_line_trail_material: Handle<LineMaterial>,
    pub enemy_line_trail_material: Handle<LineMaterial>,

    // Audio
    pub button_hover_audio: Handle<AudioSource>,
    pub button_click_audio: Handle<AudioSource>,
    pub rebind_widget_open_audio: Handle<AudioSource>,
    pub rebind_widget_close_audio: Handle<AudioSource>,
    pub explosion_audio: Handle<AudioSource>,
    pub powerup_audio: Handle<AudioSource>,

    // Miscellaneous
    pub none: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_handles: ResMut<AssetHandles>, // textures: Res<Assets<Texture>>
) {
    let silo_tex: Handle<Texture> = asset_server.load("missile_silo.png");
    let ground_tex: Handle<Texture> = asset_server.load("ground.png");
    let building_01_tex: Handle<Texture> = asset_server.load("building_01.png");
    let building_02_tex: Handle<Texture> = asset_server.load("building_02.png");
    let building_03_tex: Handle<Texture> = asset_server.load("building_03.png");
    let missile_red_tex: Handle<Texture> = asset_server.load("missile_red.png");
    let missile_green_tex: Handle<Texture> = asset_server.load("missile_green.png");
    let explosion_red_tex: Handle<Texture> = asset_server.load("explosion_red.png");
    let explosion_green_tex: Handle<Texture> = asset_server.load("explosion_green.png");
    let debris_01: Handle<Texture> = asset_server.load("debris_01.png");
    let silo_debris_01: Handle<Texture> = asset_server.load("silo_debris_01.png");
    let score_powerup_tex = asset_server.load("score_powerup.png");
    let explosion_size_powerup_tex = asset_server.load("explosion_size_powerup.png");
    let missile_speed_powerup_tex = asset_server.load("missile_speed_powerup.png");

    asset_handles.font = asset_server.load("BlocTekRegular-gxEZ4.ttf");
    asset_handles.simple_font = asset_server.load("MontserratBold-DOWZd.ttf");
    asset_handles.button_normal = materials.add(Color::rgb(0.15, 0.15, 0.15).into());
    asset_handles.button_hover = materials.add(Color::rgb(0.35, 0.35, 0.35).into());
    asset_handles.button_click = materials.add(Color::rgb(0.35, 0.85, 0.35).into());
    asset_handles.missile_red = materials.add(missile_red_tex.into());
    asset_handles.missile_green = materials.add(missile_green_tex.into());
    asset_handles.explosion_red = materials.add(explosion_red_tex.into());
    asset_handles.explosion_green = materials.add(explosion_green_tex.into());
    asset_handles.building_01 = materials.add(building_01_tex.into());
    asset_handles.building_02 = materials.add(building_02_tex.into());
    asset_handles.building_03 = materials.add(building_03_tex.into());
    asset_handles.ground = materials.add(ground_tex.into());
    asset_handles.silo = materials.add(silo_tex.into());
    asset_handles.debris_01 = materials.add(debris_01.into());
    asset_handles.silo_debris_01 = materials.add(silo_debris_01.into());
    asset_handles.silo_reload_loading = materials.add(ColorMaterial {
        color: Color::rgb(0.9, 0.9, 0.3),
        texture: None,
    });
    asset_handles.silo_reload_ready = materials.add(ColorMaterial {
        color: Color::rgb(0.3, 0.9, 0.3),
        texture: None,
    });
    asset_handles.score_powerup = materials.add(score_powerup_tex.into());
    asset_handles.explosion_size_powerup = materials.add(explosion_size_powerup_tex.into());
    asset_handles.missile_speed_powerup = materials.add(missile_speed_powerup_tex.into());
    asset_handles.rebind_widget = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.9).into());
    asset_handles.none = materials.add(Color::NONE.into());
    asset_handles.button_hover_audio = asset_server.load("audio/Cursor Rollover 1.wav");
    asset_handles.button_click_audio = asset_server.load("audio/Warm Digital Accept Button HP.wav");
    asset_handles.rebind_widget_open_audio = asset_server.load("audio/Window Open 2.wav");
    asset_handles.rebind_widget_close_audio = asset_server.load("audio/Button Sound 8.wav");
    asset_handles.explosion_audio = asset_server.load("audio/explosionCrunch_000.ogg");
    asset_handles.powerup_audio = asset_server.load("audio/Main Window Open.mp3");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_game(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    windows: Res<Windows>,
    color_mats: Res<Assets<ColorMaterial>>,
    textures: Res<Assets<Texture>>,
) {
    let (window_width, window_half_width, window_half_height) = {
        let window = windows.get_primary().unwrap();
        (window.width(), window.width() / 2.0, window.height() / 2.0)
    };
    let (ground_y, ground_height) = {
        // This can fail if the texture hasn't loaded but it shouldn't happen
        // as long as the app isn't run immediately with GameState set.
        let mat = color_mats.get(asset_handles.ground.clone()).unwrap();
        let tex_handle = mat.texture.clone().unwrap();
        let tex_height = textures.get(tex_handle).unwrap().size.height as f32;
        (-window_half_height + tex_height / 2.0, tex_height)
    };
    let silo_height = {
        let mat = color_mats.get(asset_handles.silo.clone()).unwrap();
        let tex_handle = mat.texture.clone().unwrap();
        let tex_height = textures.get(tex_handle).unwrap().size.height as f32;
        tex_height
    };
    let building_height = {
        // All buildings are currently the same height...
        let mat = color_mats.get(asset_handles.building_01.clone()).unwrap();
        let tex_handle = mat.texture.clone().unwrap();
        let tex_height = textures.get(tex_handle).unwrap().size.height as f32;
        tex_height
    };

    // Ground
    commands
        .spawn_bundle(SpriteBundle {
            material: asset_handles.ground.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, ground_y, 0.0),
                scale: Vec3::new(2.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ground);

    // Silos and Buildings
    let step_size = window_width / 9.0; // 3 silos + 6 buildings
    let half_step = step_size / 2.0;

    for i in 0..9 {
        match i {
            0 | 4 | 8 => {
                let x = (step_size * i as f32) + half_step - window_half_width;
                let y = silo_height / 2.0 + ground_height - window_half_height;

                let silo_location = match i {
                    0 => SiloLocation::Left,
                    4 => SiloLocation::Middle,
                    8 => SiloLocation::Right,
                    _ => panic!("How the hell did this happen!?"),
                };

                let silo = Silo {
                    location: silo_location,
                    missiles: SILO_MAX_MISSILES - 1,
                };

                commands
                    .spawn_bundle(SpriteBundle {
                        material: asset_handles.silo.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(silo)
                    .insert(Timer::new(Duration::from_secs_f32(SILO_RELOAD_TIME), false));

                // Reload Ui
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            size: Vec2::new(50.0, 10.0),
                            ..Default::default()
                        },
                        material: asset_handles.silo_reload_loading.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y - 50.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(SiloReloadUi)
                    .insert(silo_location);

                // Missile Count Ui
                commands
                    .spawn()
                    .insert(GlobalTransform {
                        translation: Vec3::new(x, y - 30.0, 0.0),
                        ..Default::default()
                    })
                    .insert(Transform {
                        translation: Vec3::new(x, y - 30.0, 0.0),
                        ..Default::default()
                    })
                    .insert(SiloMissileCountUi)
                    .insert(silo_location)
                    .with_children(|parent| {
                        let space = 25.0;
                        for i in 0..3 {
                            parent.spawn_bundle(SpriteBundle {
                                material: asset_handles.missile_green.clone(),
                                transform: Transform {
                                    translation: Vec3::new(-space + (space * i as f32), 0.0, 0.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        }
                    });
            }
            _ => {
                let mut rng = thread_rng();
                let rand: usize = rng.gen_range(0..3);
                let building_material = match rand {
                    0 => asset_handles.building_01.clone(),
                    1 => asset_handles.building_02.clone(),
                    2 => asset_handles.building_03.clone(),
                    // Maybe a panic isn't really needed here...
                    _ => panic!("Error choosing building material."),
                };

                let x = (step_size * i as f32) + half_step - window_half_width;
                let y = building_height / 2.0 + ground_height - window_half_height;

                commands
                    .spawn_bundle(SpriteBundle {
                        material: building_material,
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Building);
            }
        }
    }
}

fn shoot(
    // keys: Res<Input<KeyCode>>,
    actions: Res<Actions>,
    mouse_pos: Res<MousePosition>,
    mut query: Query<(&mut Silo, &mut Timer, &Transform)>,
    mut events: EventWriter<SpawnMissile>,
    mut silo_ui_events: EventWriter<SiloMissileCountUpdate>,
) {
    let target = Vec3::new(mouse_pos.position.x, mouse_pos.position.y, 0.0);
    let team = Team::Player;

    for (mut silo, mut timer, transform) in query.iter_mut() {
        if silo.missiles > 0 {
            if silo.location == SiloLocation::Left && actions.just_pressed("Fire Left Silo")
                || silo.location == SiloLocation::Middle && actions.just_pressed("Fire Middle Silo")
                || silo.location == SiloLocation::Right && actions.just_pressed("Fire Right Silo")
            {
                silo.missiles -= 1;
                if timer.finished() {
                    timer.reset();
                }
                events.send(SpawnMissile {
                    position: transform.translation,
                    target,
                    team,
                });
                silo_ui_events.send(SiloMissileCountUpdate {
                    location: silo.location,
                    count: silo.missiles,
                });
            }
        }
    }
}

fn get_mouse_pos(mut cursor_evt: EventReader<CursorMoved>, mut mouse_pos: ResMut<MousePosition>) {
    for event in cursor_evt.iter() {
        let x = event.position.x - 1280.0 / 2.0;
        let y = event.position.y - 720.0 / 2.0;
        mouse_pos.position = Vec2::new(x, y);
        // println!("{:?}", event);
        // println!("x: {}, y: {}", x, y);
        // println!();
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let vel = Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
        transform.translation += vel;
    }
}

fn despawn_game(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<Building>,
            With<Silo>,
            With<DebrisType>,
            With<Missile>,
            With<Explosion>,
            With<LineTrail>,
            With<Ground>,
            With<SiloReloadUi>,
            With<SiloMissileCountUi>,
        )>,
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn check_game_over(query: Query<&Building>, mut state: ResMut<State<GameState>>) {
    let mut live_buildings = 0;

    for _ in query.iter() {
        live_buildings += 1;
    }

    if live_buildings == 0 {
        state.set(GameState::GameOver).unwrap();
    }
}

fn check_pause(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.push(GameState::Paused).unwrap();
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Missile Command".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.14)))
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(MissilePlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(EnemySpawnerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(LineTrailPlugin)
        .add_plugin(ScoreUiPlugin)
        .add_plugin(DebrisPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(SiloPlugin)
        .add_plugin(PowerupsPlugin)
        .add_plugin(OptionsMenuPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(KiraAudioPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(PlayerStatusPlugin)
        .add_plugin(GameStatusPlugin)
        .init_resource::<MousePosition>()
        .init_resource::<AssetHandles>()
        .add_startup_system(setup.system().label("setup"))
        .add_system(get_mouse_pos.system().label("get_mouse_position"))
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(shoot.system().after("get_mouse_position"))
                .with_system(apply_velocity.system())
                .with_system(check_game_over.system())
                .with_system(check_pause.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn_game.system()))
        .run();
}
