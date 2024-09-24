use crate::{controls::Action, ui::basic_choice::BasicChoice};

use super::{character_selection::CharacterSelection, music_room::MusicRoom, Scene};
use raylib::prelude::*;

#[derive(Debug)]
pub struct MainMenu {
    current_index: usize,
    choices: [BasicChoice; 8],
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            current_index: 0,

            choices: [
                BasicChoice::new("Start", false),
                BasicChoice::new("Extra Start", true),
                BasicChoice::new("Player Data", true),
                BasicChoice::new("Replay", true),
                BasicChoice::new("Music Room", false),
                BasicChoice::new("Replay", true),
                BasicChoice::new("Option", true),
                BasicChoice::new("Exit", false),
            ],
        }
    }
}

impl Scene for MainMenu {
    fn init(&mut self, state: &mut crate::state::State) {
        state.audio.play_bgm(0, state.bgm_volume);
    }

    fn clean_up(&mut self, state: &mut crate::state::State) {
        state.audio.stop_bgm();
    }

    fn draw(
        &self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        state: &crate::state::State,
    ) {
        let screen = (d.get_screen_width() as f32, d.get_screen_height() as f32);
        d.draw_texture(&state.assets.get("main_menu"), 0, 0, Color::WHITE);
        let width = d.measure_text("Touhou Project", 42) as f32;
        d.draw_text_pro(
            &state.assets.font,
            "Touhou Project",
            Vector2::new(screen.0 / 2. - width + 50., 50.),
            Vector2::new(0., 0.),
            0.,
            42.,
            0.,
            Color::WHITE,
        );

        let width = d.measure_text("Touhou Project", 28) as f32;
        d.draw_text_pro(
            &state.assets.font,
            "Unfinished Matrix Dream",
            Vector2::new(screen.0 / 2. - width - 80., 120.),
            Vector2::new(0., 0.),
            0.,
            28.,
            0.,
            Color::WHITE,
        );

        let position = Vector2::new(260., 250.);
        let font_size = 21.;
        for (i, val) in self.choices.iter().enumerate() {
            let position = Vector2::new(position.x, position.y + font_size * i as f32);
            val.draw(d, i == self.current_index, position, font_size, state);
        }

        let width = d.measure_text("UnknownRori © 2024", 16) as f32;
        d.draw_text_pro(
            &state.assets.font,
            "UnknownRori Copyright 2024",
            Vector2::new(screen.0 / 2. - width - 100., 480. - 24.),
            Vector2::new(0., 0.),
            0.,
            17.,
            0.,
            Color::GRAY,
        );
    }

    fn update(&mut self, d: &mut RaylibDrawHandle, state: &mut crate::state::State) {
        state.audio.update_bgm();

        if state.controls.is_pressed(Action::Down, d) {
            self.current_index = (self.current_index + 1) % self.choices.len() as usize;
            state.audio.select_sfx.play(state.sfx_volume);
        }

        if state.controls.is_pressed(Action::Up, d) {
            if self.current_index == 0 {
                self.current_index = self.choices.len() as usize;
            }
            self.current_index -= 1;
            state.audio.select_sfx.play(state.sfx_volume);
        }

        if state.controls.is_pressed(Action::Accept, d) {
            if self.current_index < 8 {
                state.audio.select_sfx.play(state.sfx_volume);
            }
            match self.current_index {
                0 => state.change_scene(Box::new(CharacterSelection::new())),
                4 => state.change_scene(Box::new(MusicRoom::new(&state))),

                7 => state.should_quit = true,
                _ => {}
            }
        }
    }
}
