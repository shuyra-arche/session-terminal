use env::wire::Span as _;
use wire::message::Message;

use super::{DATA, EOF, MAX};

#[derive(Clone, env::Frame)]
pub struct Input {
    pub kind: u8,
    n: u32,
    #[frame(count = n, fill = 0)]
    bytes: [u8; MAX],
}
impl Input {
    pub fn data(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() || bytes.len() > MAX {
            return None;
        }
        let mut input = Self {
            kind: DATA,
            n: bytes.len() as u32,
            bytes: [0; MAX],
        };
        input.bytes[..bytes.len()].copy_from_slice(bytes);
        Some(input)
    }
    pub fn eof() -> Self {
        Self {
            kind: EOF,
            n: 0,
            bytes: [0; MAX],
        }
    }
    fn valid(&self) -> bool {
        match self.kind {
            DATA => self.n > 0 && self.n as usize <= MAX,
            EOF => self.n == 0,
            _ => false,
        }
    }
    pub fn clear(&mut self) {
        self.bytes.fill(0);
        self.n = 0;
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.n as usize]
    }
}
impl Message for Input {
    type In = Self;
    type Buf = [u8; Self::LEN];
    const EMPTY: Self::Buf = [0; Self::LEN];
    fn store(&self, out: &mut [u8]) -> Option<usize> {
        self.valid().then_some(())?;
        self.store_at(out, 0)
    }
    fn fetch(bytes: &[u8]) -> Option<Self> {
        let (input, end) = Self::fetch_at(bytes, 0)?;
        (end == bytes.len() && input.valid()).then_some(input)
    }
}
