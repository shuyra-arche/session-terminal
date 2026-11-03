use ::resource::{pile::Pile, raw::Hole};
use env::wire::Span as _;
use env::{HoleCondition, PieToken, TaskId, Wait, pie};
use terminal_api::frame::{self, Input};
use wire::message::Message;

use super::{terminal::MS, Read};

pub struct Io {
    input: PieToken,
    output: PieToken,
    control: PieToken,
    pile: Pile,
}
impl Io {
    pub(super) fn of(input: PieToken, output: PieToken, control: PieToken) -> Result<Self, ()> {
        let pile = Pile::unseal(false).map_err(|_| ())?;
        let io = Self {
            input,
            output,
            control,
            pile,
        };
        io.pile
            .attach(env::ToleEventSource::Hole {
                token: input,
                condition: HoleCondition::Pull,
            })
            .map_err(|_| ())?;
        io.pile
            .attach(env::ToleEventSource::Hole {
                token: control,
                condition: HoleCondition::Pull,
            })
            .map_err(|_| ())?;
        Ok(io)
    }
    pub fn injected(owner: TaskId) -> Result<Self, ()> {
        let find = |mark| ipc::session::establish::find(owner, mark).map_err(|_| ());
        Self::of(
            find(frame::INPUT)?,
            find(frame::OUTPUT)?,
            find(frame::CONTROL)?,
        )
    }
    /// Task-local data handles. Foreground transitions revoke previously returned handles.
    pub fn raw_channels(&self) -> [PieToken; 3] {
        [self.input, self.output, self.control]
    }
    pub fn read(&self) -> Result<Read, ()> {
        self.read_with(Wait::Forever)?.ok_or(())
    }
    pub fn read_with(&self, within: Wait) -> Result<Option<Read>, ()> {
        let deadline = ipc::time::Deadline::new(within);
        let mut bytes = [0; Input::LEN];
        loop {
            match Hole::from_raw(self.control).pull(&mut bytes, Wait::POLL) {
                Ok((1, _)) if bytes[0] == frame::INTERRUPT => return Ok(Some(Read::Interrupt)),
                Ok((1, _)) if bytes[0] == frame::SUSPEND => return Ok(Some(Read::Suspend)),
                Err(error) if error.source.is_busy() => {}
                _ => return Err(()),
            }
            match Hole::from_raw(self.input).pull(&mut bytes, Wait::POLL) {
                Ok((n, _)) => {
                    let input = Input::fetch(&bytes[..n]).ok_or(())?;
                    return Ok(Some(if input.kind == frame::EOF {
                        Read::Eof
                    } else {
                        Read::Data(input)
                    }));
                }
                Err(error) if error.source.is_busy() => {}
                Err(_) => return Err(()),
            }
            let left = deadline.remaining();
            if left == Wait::POLL {
                return Ok(None);
            }
            self.pile.await_(left).map_err(|_| ())?;
        }
    }
    pub fn try_write(&self, bytes: &[u8]) -> Result<bool, ()> {
        if bytes.len() > frame::MAX || bytes.is_empty() {
            return Err(());
        }
        match Hole::from_raw(self.output).push(bytes, Wait::POLL) {
            Ok(()) => Ok(true),
            Err(error) if error.source.is_busy() => Ok(false),
            Err(_) => Err(()),
        }
    }
    pub fn write(&self, bytes: &[u8]) -> Result<(), ()> {
        for chunk in bytes.chunks(frame::MAX) {
            Hole::from_raw(self.output)
                .push(chunk, MS)
                .map_err(|_| ())?;
        }
        Ok(())
    }
    pub fn drain(&self) -> Result<(), ()> {
        Hole::from_raw(self.output)
            .wait(env::HoleCondition::Empty, MS)
            .map_err(|_| ())?
            .then_some(())
            .ok_or(())
    }
}
impl Drop for Io {
    fn drop(&mut self) {
        let _ = pie::release(self.pile.token(), env::ReleaseMode::Revoke);
    }
}
