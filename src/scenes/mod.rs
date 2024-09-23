use std::fmt::Debug;

use raylib::prelude::*;

use crate::state::State;

pub mod main_menu;
pub mod music_room;

pub trait Scene: Debug {
    fn init(&mut self, _: &mut State);
    fn clean_up(&mut self, _: &mut State);
    fn update(&mut self, _: &RaylibDrawHandle, _: &mut State);
    fn draw(&self, _: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>, _: &State);
}
