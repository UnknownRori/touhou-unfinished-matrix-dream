use raylib::prelude::*;

use crate::state::State;

#[derive(Debug)]
pub struct OptionChoice {
    pub name: String,
    pub disabled: bool,
}

impl OptionChoice {
    pub fn new(name: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            disabled,
        }
    }

    pub fn draw(
        &self,
        d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
        active: bool,
        selected: bool,
        position: Vector2,
        font_size: f32,
        state: &State,
    ) {
        if active {
            d.draw_text_ex(
                &state.assets.font_bold,
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
                &state.assets.font_bold,
                &self.name,
                position,
                font_size,
                0.,
                color,
            );
            return;
        }
        let color = if active {
            Color::new(128, 128, 128, 255)
        } else {
            Color::GRAY
        };
        let color = if selected { Color::WHITE } else { color };
        d.draw_text_ex(
            &state.assets.font_bold,
            &self.name,
            position,
            font_size,
            0.,
            color,
        );
    }
}
