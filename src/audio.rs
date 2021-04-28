use bevy::prelude::*;
use bevy_kira_audio::{Audio as KiraAudio, AudioSource};

// Event
pub struct PlayAudio {
    pub handle: Handle<AudioSource>,
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlayAudio>().add_system(play_sfx.system());
    }
}

fn play_sfx(
    audio: Res<KiraAudio>,
    mut events: EventReader<PlayAudio>,
) {
    for e in events.iter() {
        audio.play(e.handle.clone());
    }
}
