use std::collections::HashMap;

use raylib::prelude::*;

#[derive(Hash, Eq, PartialEq)]
pub enum Action {
    Attack,
    Spell,
    Focus,

    Up,
    Right,
    Left,
    Down,

    Escape,
    Accept,
}

pub enum Combination {
    Single(KeyboardKey),
    Double(KeyboardKey, KeyboardKey),
}

pub struct Controls(HashMap<Action, Combination>);

pub fn init_controls() -> Controls {
    let mut controls = Controls::new();
    controls.add(Action::Attack, Combination::Single(KeyboardKey::KEY_Z));
    controls.add(
        Action::Focus,
        Combination::Single(KeyboardKey::KEY_LEFT_SHIFT),
    );
    controls.add(Action::Spell, Combination::Single(KeyboardKey::KEY_X));

    controls.add(Action::Up, Combination::Single(KeyboardKey::KEY_UP));
    controls.add(Action::Left, Combination::Single(KeyboardKey::KEY_LEFT));
    controls.add(Action::Right, Combination::Single(KeyboardKey::KEY_RIGHT));
    controls.add(Action::Down, Combination::Single(KeyboardKey::KEY_DOWN));

    controls.add(Action::Escape, Combination::Single(KeyboardKey::KEY_ESCAPE));
    controls.add(Action::Accept, Combination::Single(KeyboardKey::KEY_ENTER));

    controls
}

impl Controls {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, action: Action, combination: Combination) {
        self.0.insert(action, combination);
    }

    pub fn is_pressed(&self, action: Action, d: &RaylibDrawHandle) -> bool {
        self.0.get(&action).map_or(false, |a| match a {
            Combination::Single(key) => d.is_key_pressed(*key),
            Combination::Double(key, key2) => d.is_key_pressed(*key) && d.is_key_pressed(*key2),
        })
    }
}
