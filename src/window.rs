pub enum Resolution {
    Resolution640x480,
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Resolution640x480
    }
}

pub enum WindowMode {
    Fullscreen,
    Windowed,
}

impl Default for WindowMode {
    fn default() -> Self {
        Self::Windowed
    }
}
