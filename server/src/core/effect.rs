use super::input::Input;

/// Largest echo from one input byte: erasing a two-column control character.
pub const ECHO_MAX: usize = 6;

pub struct Effect<'a> {
    echo: [u8; ECHO_MAX],
    n: usize,
    pub input: Input<'a>,
}

impl<'a> Effect<'a> {
    pub(super) fn new(echo: &[u8], input: Input<'a>) -> Self {
        let mut bytes = [0; ECHO_MAX];
        bytes[..echo.len()].copy_from_slice(echo);
        Self {
            echo: bytes,
            n: echo.len(),
            input,
        }
    }

    pub fn echo(&self) -> &[u8] {
        &self.echo[..self.n]
    }
}
