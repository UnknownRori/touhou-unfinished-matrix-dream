use raylib::prelude::*;

use crate::state::State;

#[derive(Debug)]
pub struct BasicChoice {
    pub name: String,
    pub disabled: bool,
}

impl BasicChoice {
    pub fn new(name: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            disabled,
        }
    }

    pub fn draw(
        &self,
        d: &mut RaylibBlendMode<'_, RaylibTextureMode<'_, RaylibDrawHandle<'_>>>,
        active: bool,
        position: Vector2,
        font_size: f32,
        state: &State,
    ) {
        if active {
            d.draw_text_ex(
                &state.assets.font,
                ">",
                Vector2::new(position.x - 20., position.y),
                font_size,
                0.,
                Color::WHITE,
            );
        }
        if self.disabled {
            let color = if active {
                Color::new(88, 88, 88, 255)
            } else {
                Color::new(88, 88, 88, 128)
            };
            d.draw_text_ex(
                &state.assets.font,
                &self.name,
                position,
                font_size,
                0.,
                color,
            );
            return;
        }
        let color = if active { Color::WHITE } else { Color::GRAY };
        d.draw_text_ex(
            &state.assets.font,
            &self.name,
            position,
            font_size,
            0.,
            color,
        );
    }
}
