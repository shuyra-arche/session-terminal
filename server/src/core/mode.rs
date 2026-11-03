//! Input interpretation and its bounded echo effect.

pub use super::{
    effect::{ECHO_MAX, Effect},
    input::Input,
};

use super::canonical::Canonical;

pub enum Mode {
    Canonical(Canonical),
}

impl Mode {
    pub fn canonical() -> Self {
        Self::Canonical(Canonical::new())
    }

    pub fn clear(&mut self) {
        match self {
            Self::Canonical(mode) => mode.clear(),
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::Canonical(mode) => mode.reset(),
        }
    }

    pub fn feed(&mut self, b: u8) -> Effect<'_> {
        match self {
            Self::Canonical(mode) => mode.feed(b),
        }
    }
}
