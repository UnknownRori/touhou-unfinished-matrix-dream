use std::fmt::Debug;

use raylib::prelude::*;

use crate::state::State;

pub mod character_selection;
pub mod instruction;
pub mod main_menu;
pub mod music_room;
pub mod stage_view;

pub trait Scene: Debug {
    fn init(&mut self, _: &mut State);
    fn clean_up(&mut self, _: &mut State);
    fn update(&mut self, _: &mut RaylibDrawHandle, _: &mut State);
    // Draw entire screen
    fn draw(
        &mut self,
        _: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        _: &State,
    );

    // Draw on the stage
    fn draw_stage(&self, _: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>, _: &State) {}
}
