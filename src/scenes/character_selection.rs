use crate::{
    controls::Action,
    difficulty::Difficulty,
    score::ScoreData,
    stage::stage1,
    state::State,
    ui::character_selection::{Character, DifficultyChoice, ShotType},
};
use raylib::prelude::*;

use super::{main_menu::MainMenu, stage_view::StageView, Scene};

#[derive(Debug)]
enum CurrentSelection {
    Difficulty,
    Character,
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
                    ShotType::new("Unknown", "Unknown", true),
                    ShotType::new("Unknown", "Unknown", true),
                    "dummy_char",
                    true,
                ),
            ],
        }
    }
}

impl Scene for CharacterSelection {
    fn init(&mut self, state: &mut State) {
        //
    }

    fn clean_up(&mut self, state: &mut State) {
        state.audio.stop_bgm();
    }

    fn update(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        state.audio.bgm[0].update_stream();

        if state.controls.is_pressed(Action::Escape, d) {
            state.audio.select_sfx.play(state.sfx_volume);
            match self.current_menu {
                CurrentSelection::Difficulty => state.change_scene(Box::new(MainMenu::new())),
                CurrentSelection::Character => self.current_menu = CurrentSelection::Difficulty,
            }
        }

        if state.controls.is_pressed(Action::Accept, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {
                    if self.difficulty_selected == 1 {
                        self.current_menu = CurrentSelection::Character;
                    }
                }
                CurrentSelection::Character => {
                    if self.character_select[0] == 0 && self.character_select[1] == 0 {
                        state.score = ScoreData::new(Difficulty::Normal);
                        state.change_scene(Box::new(StageView::new("stg1".to_owned(), stage1())));
                    }
                }
            }
        }

        if state.controls.is_pressed(Action::Left, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {}
                CurrentSelection::Character => {
                    if self.character_select[0] == 0 {
                        self.character_select[0] = self.character_select.len() as usize;
                    }
                    self.character_select[0] -= 1;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
            }
        }

        if state.controls.is_pressed(Action::Right, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {}
                CurrentSelection::Character => {
                    self.character_select[0] =
                        (self.character_select[0] + 1) % self.character_select.len() as usize;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
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
                CurrentSelection::Character => {
                    if self.character_select[1] == 0 {
                        self.character_select[1] = 2;
                    }
                    self.character_select[1] -= 1;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
            }
        }

        if state.controls.is_pressed(Action::Down, d) {
            match self.current_menu {
                CurrentSelection::Difficulty => {
                    self.difficulty_selected =
                        (self.difficulty_selected + 1) % self.difficulty_choices.len() as usize;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
                CurrentSelection::Character => {
                    self.character_select[1] = (self.character_select[1] + 1) % 2;
                    state.audio.select_sfx.play(state.sfx_volume);
                }
            }
        }
    }

    fn draw(
        &self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        state: &State,
    ) {
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
            CurrentSelection::Character => {
                for (i, val) in self.character_choices.iter().enumerate() {
                    let color = if !val.disabled {
                        Color::WHITE
                    } else {
                        Color::GRAY
                    };
                    let position = Vector2::new(300., 100.);
                    if i == self.character_select[0] {
                        d.draw_text_pro(
                            &state.assets.font,
                            &val.name,
                            Vector2::new(30., 100.),
                            Vector2::new(0., 0.),
                            0.,
                            28.,
                            0.,
                            color,
                        );
                        d.draw_texture_ex(
                            state.assets.get(&val.char),
                            position,
                            0.,
                            1.5,
                            Color::WHITE,
                        );
                    }

                    if i == self.character_select[0] {
                        val.type_a.draw(
                            d,
                            0 == self.character_select[1],
                            Vector2::new(30., 200.),
                            16.,
                            state,
                        );

                        val.type_b.draw(
                            d,
                            1 == self.character_select[1],
                            Vector2::new(30., 260.),
                            16.,
                            state,
                        );
                    }
                }
            }
        }
    }
}
