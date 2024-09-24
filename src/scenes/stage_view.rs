use std::fmt::Debug;

use hecs::World;
use raylib::prelude::*;

use super::Scene;

pub struct StageView {
    world: World,
    camera: Camera2D,
    bg: String,

    bg_pos: Vector2,
    bg_movement: Vector2,
}

impl Debug for StageView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self.bg))
    }
}

impl StageView {
    pub fn new(bg: String) -> Self {
        let world = World::new();
        let camera = Camera2D {
            target: Vector2 { x: 0.0, y: 0.0 },
            offset: Vector2 { x: 0., y: 0. },
            rotation: 0.0,
            zoom: 1.0,
        };
        let bg_pos = Vector2::new(0., 0.);
        let bg_movement = Vector2::new(0., 100.);
        Self {
            world,
            bg,
            camera,
            bg_pos,
            bg_movement,
        }
    }
}

impl Scene for StageView {
    fn clean_up(&mut self, _: &mut crate::state::State) {
        //
    }

    fn update(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _: &mut crate::state::State) {
        self.bg_pos += self.bg_movement * d.get_frame_time();
        if self.bg_pos.y >= 448. {
            self.bg_pos.y = 0.;
        }
    }

    fn draw(
        &self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        state: &crate::state::State,
    ) {
        d.draw_texture(&state.assets.get("stage_view"), 0, 0, Color::WHITE);
        d.draw_text_pro(
            &state.assets.font,
            state.score.difficulty.as_ref(),
            Vector2::new(480., 20.),
            Vector2::new(0., 0.),
            0.,
            24.,
            0.,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &state.assets.get("title"),
            Vector2::new(460., 250.),
            0.,
            0.65,
            Color::WHITE,
        );

        // INFO : Score
        d.draw_text_pro(
            &state.assets.font,
            "Hi-Score",
            Vector2::new(420., 54.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );
        let score_text = format!("{:08}", 00100000);
        d.draw_text_pro(
            &state.assets.font,
            &score_text,
            Vector2::new(550., 54.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.5,
            Color::WHITE,
        );

        d.draw_text_pro(
            &state.assets.font,
            "Score",
            Vector2::new(420., 74.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );
        let score_text = format!("{:08}", state.score.score);

        d.draw_text_pro(
            &state.assets.font,
            &score_text,
            Vector2::new(550., 74.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.5,
            Color::WHITE,
        );

        // INFO : Life
        d.draw_text_pro(
            &state.assets.font,
            "Life",
            Vector2::new(420., 104.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );

        // INFO : Spell Cards
        d.draw_text_pro(
            &state.assets.font,
            "Spell Cards",
            Vector2::new(420., 128.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );

        // INFO : Power
        d.draw_text_pro(
            &state.assets.font,
            "Power",
            Vector2::new(420., 154.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );

        // INFO : Value
        d.draw_text_pro(
            &state.assets.font,
            "Value",
            Vector2::new(420., 174.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );

        let score_text = format!("{:08}", state.score.value);
        d.draw_text_pro(
            &state.assets.font,
            &score_text,
            Vector2::new(550., 174.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.5,
            Color::WHITE,
        );

        // INFO : Graze
        d.draw_text_pro(
            &state.assets.font,
            "Graze",
            Vector2::new(420., 194.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.,
            Color::WHITE,
        );
        let score_text = format!("{:08}", state.score.graze);
        d.draw_text_pro(
            &state.assets.font,
            &score_text,
            Vector2::new(550., 194.),
            Vector2::new(0., 0.),
            0.,
            18.,
            0.5,
            Color::WHITE,
        );
    }

    fn draw_stage(
        &self,
        d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
        state: &crate::state::State,
    ) {
        let mut md = d.begin_mode2D(self.camera);

        for i in 0..2 {
            md.draw_texture_v(
                &state.assets.get(&self.bg),
                Vector2::new(0., self.bg_pos.y - 448. * i as f32),
                Color::WHITE,
            );
        }
    }

    fn init(&mut self, _: &mut crate::state::State) {
        //
    }
}
