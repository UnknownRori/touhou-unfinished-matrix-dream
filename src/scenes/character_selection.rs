use crate::{
    controls::Action,
    state::State,
    ui::character_selection::{Character, DifficultyChoice, ShotType},
};
use raylib::prelude::*;

use super::{main_menu::MainMenu, Scene};

#[derive(Debug)]
enum CurrentSelection {
    Difficulty,
    Character,
    ShootType,
}

#[derive(Debug)]
pub struct CharacterSelection {
    current_menu: CurrentSelection,
    pub difficulty_selected: usize,
    pub character_select: [usize; 2],

    difficulty_choices: [DifficultyChoice; 4],
    character_choices: [Character; 2],
}

impl CharacterSelection {
    pub fn new() -> Self {
        Self {
            current_menu: CurrentSelection::Difficulty,
            difficulty_selected: 0,
            character_select: [0, 0],
            difficulty_choices: [
                DifficultyChoice::new("Easy", "New to STG", true),
                DifficultyChoice::new("Normal", "Most People", false),
                DifficultyChoice::new("Hard", "Arcade Difficulty", true),
                DifficultyChoice::new("Lunatic", "Weird People", true),
            ],
            character_choices: [
                Character::new(
                    "Hakurei Reimu",
                    ShotType::new("Percission Needle", "Fantasy Seal", false),
                    ShotType::new("Homing Amulet", "Evil Sealing Circle", true),
                    "reimu_char",
                    false,
                ),
                Character::new(
                    "Unknown",
                    ShotType::new("Unknown", "Unknown", false),
                    ShotType::new("Unknown", "Unknown", false),
                    "dummy_char",
                    true,
                ),
            ],
        }
    }
}

impl Scene for CharacterSelection {
    fn init(&mut self, state: &mut State) {
        state.audio.bgm[0].play_stream(state.bgm_volume);
    }

    fn clean_up(&mut self, _: &mut State) {
        //
    }

    fn update(&mut self, d: &RaylibDrawHandle, state: &mut State) {
        state.audio.bgm[0].update_stream();

        if state.controls.is_pressed(Action::Escape, d) {
            state.audio.select_sfx.play(state.sfx_volume);
            match self.current_menu {
                CurrentSelection::Difficulty => state.change_scene(Box::new(MainMenu::new())),
                CurrentSelection::Character => self.current_menu = CurrentSelection::Difficulty,
                CurrentSelection::ShootType => self.current_menu = CurrentSelection::ShootType,
            }
        }

        if state.controls.is_pressed(Action::Up, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {
                    if self.difficulty_selected == 0 {
                        self.difficulty_selected = self.difficulty_choices.len() as usize;
                    }
                    self.difficulty_selected -= 1;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
                CurrentSelection::Character => todo!(),
                CurrentSelection::ShootType => todo!(),
            }
        }

        if state.controls.is_pressed(Action::Down, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {
                    self.difficulty_selected =
                        (self.difficulty_selected + 1) % self.difficulty_choices.len() as usize;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
                CurrentSelection::Character => todo!(),
                CurrentSelection::ShootType => todo!(),
            }
        }
    }

    fn draw(&self, d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>, state: &State) {
        let screen = (d.get_screen_width() as f32, d.get_screen_height() as f32);
        d.draw_texture(&state.assets.get("main_menu"), 0, 0, Color::WHITE);

        match self.current_menu {
            CurrentSelection::Difficulty => {
                let position = Vector2::new(250., 150.);
                let font_size = 24.;
                let skip = 50.;

                for (i, val) in self.difficulty_choices.iter().enumerate() {
                    let position = Vector2::new(position.x, position.y + skip * i as f32);

                    val.draw(d, self.difficulty_selected == i, position, font_size, state);
                }
            }
            CurrentSelection::Character => {}
            CurrentSelection::ShootType => {}
        }
    }
}
