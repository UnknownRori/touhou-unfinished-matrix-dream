use std::fmt::Debug;

use hecs::World;
use raylib::prelude::*;

use super::Scene;

pub struct StageView {
    world: World,
    camera: Camera2D,
    bg: String,
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
        Self { world, bg, camera }
    }
}

impl Scene for StageView {
    fn clean_up(&mut self, _: &mut crate::state::State) {
        //
    }

    fn update(&mut self, _: &mut raylib::prelude::RaylibDrawHandle, _: &mut crate::state::State) {}

    fn draw(
        &self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        state: &crate::state::State,
    ) {
        d.draw_texture(&state.assets.get("stage_view"), 0, 0, Color::WHITE);
    }

    fn draw_stage(
        &self,
        d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
        state: &crate::state::State,
    ) {
        let mut md = d.begin_mode2D(self.camera);
        md.draw_texture(&state.assets.get(&self.bg), 0, 0, Color::WHITE);
    }

    fn init(&mut self, _: &mut crate::state::State) {
        //
    }
}
