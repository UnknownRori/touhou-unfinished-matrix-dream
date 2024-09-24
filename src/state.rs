use raylib::{prelude::*, RaylibHandle, RaylibThread};

use crate::{
    assets::{Assets, AudioAssets},
    controls::{init_controls, Controls},
    scenes::Scene,
    score::ScoreData,
    window::{Resolution, WindowMode},
};

pub struct State<'a> {
    pub assets: Assets,
    pub audio: AudioAssets<'a>,

    pub resolution: Resolution,
    pub window_mode: WindowMode,
    pub should_quit: bool,

    pub bgm_volume: f32,
    pub sfx_volume: f32,

    current_scene: Option<Box<dyn Scene>>,
    old_scene: Option<Box<dyn Scene>>,

    pub score: ScoreData,
    pub controls: Controls,
}

impl<'a> State<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, audio: AudioAssets<'a>) -> Self {
        Self {
            assets: Assets::new(rl, thread),
            audio,

            resolution: Resolution::default(),
            window_mode: WindowMode::default(),
            should_quit: false,

            current_scene: None,
            old_scene: None,

            bgm_volume: 1.0,
            sfx_volume: 0.6,

            score: ScoreData::default(),
            controls: init_controls(),
        }
    }

    pub fn should_quit(&self) -> &bool {
        &self.should_quit
    }

    pub fn change_scene(&mut self, mut scene: Box<dyn Scene>) {
        // TODO : create transition
        if self.current_scene.is_some() {
            let mut old_scene = self.current_scene.take().unwrap();
            old_scene.clean_up(self);
        }
        scene.init(self);
        self.current_scene = Some(scene);
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        if self.current_scene.is_some() {
            let mut scene = self.current_scene.take().unwrap();
            scene.update(d, self);
            if self.current_scene.is_none() {
                self.current_scene = Some(scene);
            } else {
                scene.clean_up(self);
            }
        }
    }

    pub fn draw(
        &mut self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
    ) {
        d.clear_background(Color::BLACK);
        if self.current_scene.is_some() {
            let mut scene = self.current_scene.take().unwrap();
            scene.draw(d, self);
            self.current_scene = Some(scene);
        }
    }

    pub fn draw_stage(&mut self, d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>) {
        d.clear_background(Color::BLANK);
        if self.current_scene.is_some() {
            let scene = self.current_scene.take().unwrap();
            scene.draw_stage(d, self);
            self.current_scene = Some(scene);
        }
    }
}
