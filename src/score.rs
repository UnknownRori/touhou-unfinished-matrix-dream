use crate::difficulty::{self, Difficulty};

#[derive(Debug)]
pub struct ScoreData {
    pub graze: i32,
    pub score: u64,
    pub power: f32,
    pub value: u64,
    pub life: i8,
    pub spell: i8,
    pub difficulty: Difficulty,
}

impl Default for ScoreData {
    fn default() -> Self {
        Self {
            graze: 0,
            score: 0,
            power: 1.0,
            value: 10000,
            life: 3,
            spell: 3,
            difficulty: Difficulty::Normal,
        }
    }
}

impl ScoreData {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            graze: 0,
            score: 0,
            power: 1.0,
            value: 10000,
            life: 3,
            spell: 3,
            difficulty,
        }
    }
}
