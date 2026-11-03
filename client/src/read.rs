use terminal_api::frame::Input;

pub enum Read {
    Data(Input),
    Eof,
    Interrupt,
    Suspend,
}
