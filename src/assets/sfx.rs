use raylib::audio::{RaylibAudio, Sound};

use crate::state::State;

pub struct Sfx<'a>(Sound<'a>);

impl<'a> Sfx<'a> {
    pub fn new(path: &str, audio: &'a RaylibAudio) -> Self {
        let sfx = audio.new_sound(path).unwrap();

        Self(sfx)
    }

    pub fn play(&mut self, volume: f32) {
        if !self.0.is_playing() {
            self.0.set_volume(volume);
            self.0.play();
        }
    }
}
