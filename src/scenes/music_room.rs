use raylib::prelude::*;

use crate::{controls::Action, state::State, ui::option_choice::OptionChoice};

use super::{main_menu::MainMenu, Scene};

#[derive(Debug)]
pub struct MusicRoom {
    pub selection_index: usize,
    pub current_music: usize,
    pub choices: Vec<OptionChoice>,
}

impl MusicRoom {
    pub fn new(state: &State) -> Self {
        let choices = state
            .audio
            .bgm
            .iter()
            .map(|bgm| OptionChoice::new(&bgm.name, false))
            .collect();

        Self {
            selection_index: 0,
            current_music: 0,
            choices,
        }
    }
}

impl Scene for MusicRoom {
    fn init(&mut self, _: &mut crate::state::State) {
        //
    }

    fn clean_up(&mut self, state: &mut crate::state::State) {
        state.audio.bgm[self.current_music].stop_stream();
    }

    fn update(&mut self, d: &raylib::prelude::RaylibDrawHandle, state: &mut crate::state::State) {
        state.audio.bgm[self.current_music].update_stream();

        if state.controls.is_pressed(Action::Escape, d) {
            state.change_scene(Box::new(MainMenu::new()));
        }

        if state.controls.is_pressed(Action::Down, d) {
            self.selection_index = (self.selection_index + 1) % self.choices.len() as usize;
        }

        if state.controls.is_pressed(Action::Up, d) {
            if self.selection_index == 0 {
                self.selection_index = self.choices.len() as usize;
            }
            self.selection_index -= 1;
        }

        if state.controls.is_pressed(Action::Accept, d) {
            state.audio.bgm[self.current_music].stop_stream();

            self.current_music = self.selection_index;
            state.audio.bgm[self.current_music].play_stream(state.bgm_volume);
        }
    }

    fn draw(
        &self,
        d: &mut raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
        state: &crate::state::State,
    ) {
        let screen = (d.get_screen_width() as f32, d.get_screen_height() as f32);
        d.draw_texture(&state.assets.main_menu, 0, 0, Color::WHITE);
        d.draw_text_pro(
            &state.assets.font_bold,
            "Music Room",
            Vector2::new(200., 50.),
            Vector2::new(0., 0.),
            0.,
            42.,
            0.,
            Color::WHITE,
        );
        let width = d.measure_text("UnknownRori © 2024", 16) as f32;
        d.draw_text_pro(
            &state.assets.font_bold,
            "UnknownRori Copyright 2024",
            Vector2::new(screen.0 / 2. - width - 100., 480. - 24.),
            Vector2::new(0., 0.),
            0.,
            17.,
            0.,
            Color::GRAY,
        );

        let position = Vector2::new(40., 100.);
        let font_size = 21.;
        for (i, val) in self.choices.iter().enumerate() {
            let position = Vector2::new(position.x, position.y + font_size * i as f32);
            val.draw(
                d,
                i == self.selection_index,
                i == self.current_music,
                position,
                font_size,
                state,
            );
        }

        d.draw_text_pro(
            &state.assets.font_bold,
            &state.audio.bgm[self.current_music].description,
            Vector2::new(40., 380.),
            Vector2::new(0., 0.),
            0.,
            17.,
            0.,
            Color::WHITE,
        );
    }
}
