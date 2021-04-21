use bevy::{prelude::*, render::pipeline::PipelineDescriptor};
use rand::prelude::*;

mod collision;
mod debris;
mod enemy_spawner;
mod explosion;
mod line_trail;
mod main_menu;
mod missile;
mod score_ui;
mod state;
mod team;

use collision::CollisionPlugin;
use debris::DebrisPlugin;
use enemy_spawner::EnemySpawnerPlugin;
use explosion::ExplosionPlugin;
use line_trail::{LineMaterial, LineTrailPlugin};
use main_menu::MainMenuPlugin;
use missile::{MissilePlugin, SpawnMissile};
use score_ui::ScoreUiPlugin;
use state::GameState;
use team::Team;

const MISSILE_VELOCITY: f32 = 200.0;

struct Silo {
    location: SiloLocation,
}

#[derive(PartialEq, Debug)]
enum SiloLocation {
    Left,
    Middle,
    Right,
}

struct Building;

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

    // Line Trail
    pub line_trail: Handle<Mesh>,
    pub line_trail_pipeline: Handle<PipelineDescriptor>,
    pub player_line_trail_material: Handle<LineMaterial>,
    pub enemy_line_trail_material: Handle<LineMaterial>,
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

    asset_handles.font = asset_server.load("BlocTekRegular-gxEZ4.ttf");
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_game(mut commands: Commands, asset_handles: Res<AssetHandles>, windows: Res<Windows>) {
    // Ground
    commands.spawn_bundle(SpriteBundle {
        material: asset_handles.ground.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -328.0, 0.0),
            scale: Vec3::new(2.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Silos and Buildings
    let (width, half_width) = if let Some(window) = windows.get_primary() {
        (window.width(), window.width() / 2.0)
    } else {
        eprintln!("Could not find primary window!");
        std::process::exit(1);
    };
    let step_size = width / 9.0; // 3 silos + 6 buildings
    let half_step = step_size / 2.0;

    for i in 0..9 {
        match i {
            0 | 4 | 8 => {
                let x = (step_size * i as f32) + half_step - half_width;
                // Can't get the height as Texture it is still loading so
                // just hard code it for now...
                // let y = textures.get(silo_tex.clone()).unwrap().size.height as f32 - 328.0 + 32.0;

                // TODO - Lots of hardcoded values here that should be put into
                //        proper variables for clarity
                let y = 16.0 - 328.0 + 32.0;

                let silo = match i {
                    0 => Silo {
                        location: SiloLocation::Left,
                    },
                    4 => Silo {
                        location: SiloLocation::Middle,
                    },
                    8 => Silo {
                        location: SiloLocation::Right,
                    },
                    _ => panic!("How the hell did this happen!?"),
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
                    .insert(silo);
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

                let x = (step_size * i as f32) + half_step - half_width;
                let y = 32.0 - 328.0 + 32.0;

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
    keys: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePosition>,
    query: Query<(&Silo, &Transform)>,
    mut event: EventWriter<SpawnMissile>,
) {
    // Could probably just put the silo positions in a resource on startup,
    // they should only change on screen resize
    let mut positions = [Vec3::ZERO; 3];
    for (i, (_, transform)) in query.iter().enumerate() {
        positions[i] = transform.translation;
        positions[i].y += 10.0;
    }

    let target = Vec3::new(mouse_pos.position.x, mouse_pos.position.y, 0.0);
    let team = Team::Player;

    if keys.just_pressed(KeyCode::A) {
        event.send(SpawnMissile {
            position: positions[0],
            target,
            team,
        });
    }
    if keys.just_pressed(KeyCode::S) {
        event.send(SpawnMissile {
            position: positions[1],
            target,
            team,
        });
    }
    if keys.just_pressed(KeyCode::D) {
        event.send(SpawnMissile {
            position: positions[2],
            target,
            team,
        });
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

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Missile Command".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.14)))
        .add_state(GameState::Game)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(MissilePlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(EnemySpawnerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(LineTrailPlugin)
        .add_plugin(ScoreUiPlugin)
        .add_plugin(DebrisPlugin)
        .init_resource::<MousePosition>()
        .init_resource::<AssetHandles>()
        .add_startup_system(setup.system().label("setup"))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(get_mouse_pos.system().label("get_mouse_position"))
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_game.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(shoot.system().after("get_mouse_position"))
                .with_system(apply_velocity.system()),
        )
        .run();
}
