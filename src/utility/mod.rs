use raylib::prelude::*;

pub mod timer;

pub fn get_sprite_coord(row: i32, height: i32, row_size: f32, height_size: f32) -> Rectangle {
    Rectangle::new(
        row as f32 * row_size,
        height as f32 * height_size,
        row_size,
        height_size,
    )
}
