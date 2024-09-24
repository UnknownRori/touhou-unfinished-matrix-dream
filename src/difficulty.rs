#[derive(Debug)]
pub enum Difficulty {
    Easy,    // Plebs
    Normal,  // Good
    Hard,    // Master
    Lunatic, // Master of Master
}

impl Difficulty {
    #[inline(always)]
    pub fn difficulty_value<T>(&self, easy: T, normal: T, hard: T, lunatic: T) -> T {
        match self {
            Difficulty::Easy => easy,
            Difficulty::Normal => normal,
            Difficulty::Hard => hard,
            Difficulty::Lunatic => lunatic,
        }
    }
}

impl AsRef<str> for Difficulty {
    fn as_ref(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
            Difficulty::Lunatic => "Lunatic",
        }
    }
}
