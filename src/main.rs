use bevy::prelude::*;

const MISSILE_VELOCITY: f32 = 200.0;

struct Silo;

struct Velocity(Vec2);

#[derive(Default)]
struct MousePosition {
    position: Vec2,
}

#[derive(Default)]
struct AssetHandles {
    missile_red: Handle<ColorMaterial>,
    missile_green: Handle<ColorMaterial>,
    explosion_red: Handle<ColorMaterial>,
    explosion_green: Handle<ColorMaterial>,
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

                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(silo_tex.clone().into()),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Silo);
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
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePosition>,
    asset_handles: Res<AssetHandles>,
    query: Query<(&Silo, &Transform)>,
) {
    for (_, transform) in query.iter() {
        if keys.just_pressed(KeyCode::A) {
            let mut spawn_point = transform.translation;
            spawn_point.y += 16.0; // Should replace this with Texture half height

            // Rotate missile towards mouse position
            let a = Vec2::new(0.0, 1.0);
            let b = mouse_pos.position - Vec2::new(spawn_point.x, spawn_point.y);
            let angle = a.angle_between(b);

            // Missile velocity
            let velocity = b.normalize() * MISSILE_VELOCITY;

            commands
                .spawn_bundle(SpriteBundle {
                    material: asset_handles.missile_green.clone(),
                    transform: Transform {
                        translation: spawn_point,
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity(velocity));
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

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Missile Command".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.14)))
        .add_plugins(DefaultPlugins)
        .init_resource::<MousePosition>()
        .init_resource::<AssetHandles>()
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(get_mouse_pos.system().label("get_mouse_position"))
        .add_system(shoot.system().after("get_mouse_position"))
        .add_system(apply_velocity.system())
        .run();
}
