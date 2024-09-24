use raylib::prelude::*;

use crate::controls::Action;

use super::{main_menu::MainMenu, Scene};

#[derive(Debug)]
pub struct Instruction;

impl Scene for Instruction {
    fn init(&mut self, _: &mut crate::state::State) {
        //
    }

    fn clean_up(&mut self, _: &mut crate::state::State) {
        //
    }

    fn update(
        &mut self,
        d: &mut raylib::prelude::RaylibDrawHandle,
        state: &mut crate::state::State,
    ) {
        if state.controls.is_pressed(Action::Attack, d) {
            state.change_scene(Box::new(MainMenu::new()));
        }
    }

    fn draw(
        &mut self,
        d: &mut raylib::prelude::RaylibBlendMode<
            '_,
            raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
        >,
        state: &crate::state::State,
    ) {
        d.draw_text_pro(
            &state.assets.font,
            "Z  ->  Attack / Accept Selection / Continue Dialog",
            Vector2::new(40., 50.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "X  ->  Bomb",
            Vector2::new(40., 94.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "L Shift  ->  Focus / Slowdown",
            Vector2::new(40., 132.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "Arrow Key  ->  Movement",
            Vector2::new(40., 172.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "Esc  ->  Exit the current Menu",
            Vector2::new(40., 202.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "Enter  ->  Accept Selection",
            Vector2::new(40., 242.),
            Vector2::new(0., 0.),
            0.,
            21.,
            0.,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "Press Attack to Continue",
            Vector2::new(180., 400.),
            Vector2::new(0., 0.),
            0.,
            24.,
            0.,
            Color::WHITE,
        );
    }
}
