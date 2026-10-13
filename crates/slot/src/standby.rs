use std::time::Duration;

pub const STANDBY_REST: Duration = Duration::from_millis(50);

pub const DARK_FRAMES: u8 = 2;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Step {
    Draw,
    Rest,
}

#[derive(Clone, Debug, Default)]
pub struct Standby {
    dark: u8,
}

impl Standby {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, dozing: bool) -> Step {
        if !dozing {
            self.dark = 0;
            return Step::Draw;
        }
        if self.dark < DARK_FRAMES {
            self.dark += 1;
            return Step::Draw;
        }
        Step::Rest
    }

    pub fn resting(&self) -> bool {
        self.dark >= DARK_FRAMES
    }
}
