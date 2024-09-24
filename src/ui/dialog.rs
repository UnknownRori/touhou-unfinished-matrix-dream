use raylib::prelude::*;
use std::collections::VecDeque;

use crate::state::State;

pub struct Dialog {
    pub player: &'static str,
    pub opponent: &'static str,

    pub dialog_list: VecDeque<DialogItem>,
}

pub enum DialogItem {
    Player(&'static str),
    Opponent(&'static str),
}

impl Dialog {
    pub fn new(
        player: &'static str,
        opponent: &'static str,
        dialog_list: VecDeque<DialogItem>,
    ) -> Self {
        Self {
            player,
            opponent,
            dialog_list,
        }
    }

    pub fn next(&mut self) {
        self.dialog_list.pop_front();
    }

    pub fn done(&self) -> bool {
        self.dialog_list.is_empty()
    }

    pub fn draw(
        &self,
        state: &State,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
    ) {
        let alpha = match self.dialog_list.front() {
            Some(dialog) => match dialog {
                DialogItem::Player(_) => (255, 128),
                DialogItem::Opponent(_) => (128, 255),
            },
            None => (128, 128),
        };

        let text = match self.dialog_list.front() {
            Some(dialog) => match dialog {
                DialogItem::Player(a) => a,
                DialogItem::Opponent(a) => a,
            },
            None => "",
        };
        d.draw_texture(
            &state.assets.get(&self.player),
            0,
            200,
            Color::new(255, 255, 255, alpha.0),
        );
        d.draw_texture(
            &state.assets.get(&self.opponent),
            200,
            250,
            Color::new(255, 255, 255, alpha.1),
        );
        d.draw_rectangle(0, 400, 640, 480 - 400, Color::new(0, 0, 0, 128));
        d.draw_text_pro(
            &state.assets.font,
            text,
            Vector2::new(0., 410.),
            Vector2::new(0., 0.),
            0.,
            16.,
            0.,
            Color::WHITE,
        );
    }
}
