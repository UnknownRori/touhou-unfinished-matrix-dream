use raylib::prelude::*;

use crate::state::State;

// Description
#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub type_a: ShotType,
    pub type_b: ShotType,
    pub disabled: bool,
    pub char: String,
}

#[derive(Debug)]
pub struct ShotType {
    bullet_type: String,
    spell_card: String,
    disabled: bool,
}

impl ShotType {
    pub fn new(bullet_type: &str, spell_card: &str, disabled: bool) -> Self {
        Self {
            bullet_type: bullet_type.to_owned(),
            spell_card: spell_card.to_owned(),
            disabled,
        }
    }
}

impl ShotType {
    pub fn draw(
        &self,
        d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
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
                &format!("Bullet Type : {}", &self.bullet_type),
                position,
                font_size,
                0.,
                color,
            );

            d.draw_text_ex(
                &state.assets.font,
                &format!("Spells : {}", &self.spell_card),
                Vector2::new(position.x, position.y + font_size + 4.),
                font_size,
                0.,
                color,
            );
            return;
        }

        let color = if active { Color::WHITE } else { Color::GRAY };
        d.draw_text_ex(
            &state.assets.font,
            &format!("Bullet Type : {}", &self.bullet_type),
            position,
            font_size,
            0.,
            color,
        );

        d.draw_text_ex(
            &state.assets.font,
            &format!("Spells : {}", &self.spell_card),
            Vector2::new(position.x, position.y + font_size + 4.),
            font_size,
            0.,
            color,
        );
    }
}

impl Character {
    pub fn new(name: &str, type_a: ShotType, type_b: ShotType, char: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            type_a,
            type_b,
            char: char.to_owned(),
            disabled,
        }
    }
}

#[derive(Debug)]
pub struct DifficultyChoice {
    pub name: String,
    pub comment: String,
    pub disabled: bool,
}

impl DifficultyChoice {
    pub fn new(name: &str, comment: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            comment: comment.to_owned(),
            disabled,
        }
    }

    pub fn draw(
        &self,
        d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
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

            d.draw_text_pro(
                &state.assets.font,
                &self.name,
                position,
                Vector2::new(0., 0.),
                0.,
                font_size,
                0.,
                color,
            );

            let color = if active {
                Color::new(128, 128, 128, 255)
            } else {
                Color::new(100, 100, 100, 128)
            };

            d.draw_text_pro(
                &state.assets.font,
                &self.comment,
                Vector2::new(position.x, position.y + font_size + 4.),
                Vector2::new(0., 0.),
                0.,
                16.,
                0.,
                color,
            );
            return;
        }

        let color = if active { Color::WHITE } else { Color::GRAY };
        d.draw_text_pro(
            &state.assets.font,
            &self.name,
            position,
            Vector2::new(0., 0.),
            0.,
            font_size,
            0.,
            color,
        );

        let color = if active {
            Color::new(128, 128, 128, 255)
        } else {
            Color::new(88, 88, 88, 128)
        };

        d.draw_text_pro(
            &state.assets.font,
            &self.comment,
            Vector2::new(position.x, position.y + font_size + 4.),
            Vector2::new(0., 0.),
            0.,
            16.,
            0.,
            color,
        );
    }
}
