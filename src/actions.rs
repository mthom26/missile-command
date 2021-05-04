use std::fs::File;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use ron::{
    de::from_reader,
    ser::{to_writer_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionsMap {
    pub keyboard: HashMap<KeyCode, String>,
}

impl ActionsMap {
    pub fn update_action(
        &mut self,
        new_action: &str,
        previous_keycode: KeyCode,
        new_keycode: KeyCode,
    ) {
        if self.keyboard.contains_key(&previous_keycode) {
            // TODO - Check if the new keycode is already assigned to a different action
            //        Will want to display error message in rebind widget somehow...
            self.keyboard.remove(&previous_keycode);
        }

        // Remove previous action, this assumes that each action will have no
        // more than one binding.
        let mut to_remove = None;
        for (keycode, action) in self.keyboard.iter_mut() {
            if action == new_action {
                to_remove = Some(*keycode);
            }
        }
        match to_remove {
            Some(keycode) => {
                self.keyboard.remove(&keycode).unwrap();
            }
            None => {}
        };

        self.keyboard.insert(new_keycode, new_action.to_string());

        self.write_config_to_file();
    }

    pub fn reset_bindings(&mut self) {
        let path = format!("./config/default_bindings.ron");
        let f = File::open(&path).expect("Could not open file");

        *self = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };

        self.write_config_to_file();
    }

    fn write_config_to_file(&self) {
        let path = format!("./config/bindings.ron");
        let file = File::create(&path).expect("Could not create bindings file.");
        let config = PrettyConfig::new();
        to_writer_pretty(file, &self, config).unwrap();
    }
}

#[derive(Default)]
pub struct Actions {
    pressed: HashSet<String>,
    just_pressed: HashSet<String>,
    just_released: HashSet<String>,
}

impl Actions {
    pub fn update_sets(
        &mut self,
        action: &str,
        pressed: bool,
        just_pressed: bool,
        just_released: bool,
    ) {
        if pressed {
            self.pressed.insert(action.to_string());
        }
        if just_pressed {
            self.just_pressed.insert(action.to_string());
        }
        if just_released {
            self.just_released.insert(action.to_string());
        }
    }

    fn clear(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn _pressed(&self, action: &str) -> bool {
        match self.pressed.get(action) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn just_pressed(&self, action: &str) -> bool {
        match self.just_pressed.get(action) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn _just_released(&self, action: &str) -> bool {
        match self.just_released.get(action) {
            Some(_) => true,
            None => false,
        }
    }
}

fn get_input(
    input: Res<Input<KeyCode>>,
    actions_map: Res<ActionsMap>,
    mut actions: ResMut<Actions>,
) {
    actions.clear();
    for (keycode, action) in &actions_map.keyboard {
        let (pressed, just_pressed, just_released) = (
            input.pressed(*keycode),
            input.just_pressed(*keycode),
            input.just_released(*keycode),
        );
        actions.update_sets(action, pressed, just_pressed, just_released);
    }
}

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let path = format!("./config/bindings.ron");
        let f = File::open(&path).expect("Could not open file");

        // Load ActionMap from bindings.ron
        let config: ActionsMap = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };

        app.insert_resource(ActionsMap {
            keyboard: config.keyboard,
        })
        .init_resource::<Actions>()
        .add_system(get_input.system().label("get_input"));
    }
}
