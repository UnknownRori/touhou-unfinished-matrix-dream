use raylib::prelude::*;

pub mod timer;

pub fn get_sprite_coord(row: i32, height: i32, row_size: f32, height_size: f32) -> Rectangle {
    const SPRITE_SIZE: (f32, f32) = (32., 32.);
    Rectangle::new(
        row as f32 * SPRITE_SIZE.0,
        height as f32 * SPRITE_SIZE.1,
        row_size,
        height_size,
    )
}
