use raylib::audio::{Music, RaylibAudio};

use crate::state::State;

pub struct BGM<'a> {
    pub name: String,
    pub description: String,
    pub author: String,
    pub bgm: Music<'a>,
}

impl<'a> BGM<'a> {
    pub fn new(
        name: &str,
        author: &str,
        description: &str,
        path: &str,
        audio: &'a RaylibAudio,
    ) -> Self {
        let bgm = audio.new_music(path).unwrap();

        Self {
            name: name.to_owned(),
            author: author.to_owned(),
            description: description.to_owned(),
            bgm,
        }
    }

    pub fn play_stream(&mut self, volume: f32) {
        self.bgm.set_volume(volume);
        self.bgm.play_stream();
    }

    pub fn stop_stream(&mut self) {
        self.bgm.stop_stream();
    }

    pub fn update_stream(&mut self) {
        self.bgm.update_stream();
    }
}
