use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    // textures: Res<Assets<Texture>>
) {
    let silo_tex = asset_server.load("missile_silo.png");
    let ground_tex = asset_server.load("ground.png");
    let building_tex = asset_server.load("building.png");
    // let missile_red_tex = asset_server.load("missile_red.png");
    // let missile_green_tex = asset_server.load("missile_green.png");
    // let explosion_red = asset_server.load("explosion_red");
    // let explosion_green = asset_server.load("explosion_green");

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

                commands.spawn_bundle(SpriteBundle {
                    material: materials.add(silo_tex.clone().into()),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
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

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Missile Command".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.14)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
