use crate::adapt::{self, E_NO_CONSOLE, MS};
use env::{Wait, unit};
use ipc::{rack::Mode, session::Session};
use system_client::operator;
use system_client::operator::Face;
use uart_client as client;

pub fn run() -> Result<(), env::Reason> {
    let session =
        Session::open(unit::sire(), operator::BERTH, Wait::AtMost(MS)).map_err(|_| E_NO_CONSOLE)?;
    let road = client::road().ok_or(E_NO_CONSOLE)?;
    let mut console = client::find(&Face::of(session), &road, Mode::Oldest, Wait::AtMost(MS))
        .ok_or(E_NO_CONSOLE)?;
    adapt::run::run(&mut console)
}
