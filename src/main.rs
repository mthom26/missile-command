use bevy::prelude::*;

mod missile;

use missile::{MissilePlugin, SpawnMissile};

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

struct Velocity(Vec2);

#[derive(Default)]
struct MousePosition {
    position: Vec2,
}

#[derive(Default)]
pub struct AssetHandles {
    pub missile_red: Handle<ColorMaterial>,
    pub missile_green: Handle<ColorMaterial>,
    pub explosion_red: Handle<ColorMaterial>,
    pub explosion_green: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut asset_handles: ResMut<AssetHandles>, // textures: Res<Assets<Texture>>
) {
    let silo_tex = asset_server.load("missile_silo.png");
    let ground_tex = asset_server.load("ground.png");
    let building_tex = asset_server.load("building.png");
    let missile_red_tex: Handle<Texture> = asset_server.load("missile_red.png");
    let missile_green_tex: Handle<Texture> = asset_server.load("missile_green.png");
    let explosion_red_tex: Handle<Texture> = asset_server.load("explosion_red");
    let explosion_green_tex: Handle<Texture> = asset_server.load("explosion_green");

    asset_handles.missile_red = materials.add(missile_red_tex.into());
    asset_handles.missile_green = materials.add(missile_green_tex.into());
    asset_handles.explosion_red = materials.add(explosion_red_tex.into());
    asset_handles.explosion_green = materials.add(explosion_green_tex.into());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Ground
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(ground_tex.into()),
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
                        material: materials.add(silo_tex.clone().into()),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(silo);
            }
            _ => {
                let x = (step_size * i as f32) + half_step - half_width;
                let y = 32.0 - 328.0 + 32.0;

                commands.spawn_bundle(SpriteBundle {
                    material: materials.add(building_tex.clone().into()),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
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

    if keys.just_pressed(KeyCode::A) {
        event.send(SpawnMissile {
            position: positions[0],
            target,
        })
    }
    if keys.just_pressed(KeyCode::S) {
        event.send(SpawnMissile {
            position: positions[1],
            target,
        })
    }
    if keys.just_pressed(KeyCode::D) {
        event.send(SpawnMissile {
            position: positions[2],
            target,
        })
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
        .add_plugins(DefaultPlugins)
        .add_plugin(MissilePlugin)
        .init_resource::<MousePosition>()
        .init_resource::<AssetHandles>()
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(get_mouse_pos.system().label("get_mouse_position"))
        .add_system(shoot.system().after("get_mouse_position"))
        .add_system(apply_velocity.system())
        .run();
}
