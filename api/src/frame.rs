pub use super::marks::AUTHORITY;
pub use super::marks::BACK;
pub use super::marks::CONTROL;
pub use super::marks::ENTRY;
pub use super::marks::INPUT;
pub use super::marks::OUTPUT;
pub const ATTACH: u8 = 1;
pub const FOREGROUND: u8 = 2;
pub const DETACH: u8 = 3;
pub const ECHO_OFF: u8 = 4;
pub const ECHO_ON: u8 = 5;
pub const INTERRUPT: u8 = 1;
pub const SUSPEND: u8 = 2;
pub const DATA: u8 = 1;
pub const EOF: u8 = 2;
pub const MAX: usize = 256;

mod command;
mod input;
mod reply;

pub use command::Command;
pub use input::Input;
pub use reply::Reply;
