use env::{Permission, PieToken, TaskId, pie};
use terminal_api::frame::{self, Command};

use super::{Foreground, Io, Terminal};

/// An exclusive control capability; the service owns and revokes all data grants.
pub struct Connection {
    terminal: Terminal,
    root: PieToken,
    authority: PieToken,
    channels: core::cell::Cell<[PieToken; 3]>,
}
impl Connection {
    pub fn open(terminal: Terminal) -> Result<Self, ()> {
        let root = env::tole::unseal(false).map_err(|_| ())?;
        let mut connection = Self {
            terminal,
            root,
            authority: PieToken::NONE,
            channels: core::cell::Cell::new([PieToken::NONE; 3]),
        };
        let anchor = pie::accord(
            root,
            connection.terminal.host,
            Permission::FETCH | Permission::VEST | Permission::ONLY,
            frame::AUTHORITY,
        )
        .map_err(|_| ())?;
        let reply = connection.terminal.call(Command {
            op: frame::ATTACH,
            task: env::unit::self_id(),
            authority: anchor,
            back: PieToken::NONE,
        })?;
        connection.authority = reply.authority;
        connection
            .channels
            .set([reply.input, reply.output, reply.control]);
        if connection.authority == PieToken::NONE {
            return Err(());
        }
        Ok(connection)
    }
    pub fn host(&self) -> TaskId {
        self.terminal.host
    }
    pub fn io(&self) -> Result<Io, ()> {
        let [input, output, control] = self.channels.get();
        Io::of(input, output, control)
    }
    pub(super) fn command(&self, op: u8, task: TaskId) -> Result<(), ()> {
        // The service returns this exclusive proof before acknowledging the command.
        let proof = pie::accord(
            self.authority,
            self.host(),
            Permission::FETCH | Permission::ONLY,
            frame::AUTHORITY,
        )
        .map_err(|_| ())?;
        let result = self.terminal.call(Command {
            op,
            task,
            authority: proof,
            back: PieToken::NONE,
        });
        if result.is_err() {
            let _ = pie::revoke(self.host(), proof);
        }
        result.map(|reply| {
            self.channels
                .set([reply.input, reply.output, reply.control]);
        })
    }
    /// The target remains held until control and data grants have both moved.
    pub fn lend(&mut self, task: TaskId) -> Result<Foreground<'_>, ()> {
        if task.get() == 0 || task == env::unit::self_id() {
            return Err(());
        }
        if self.command(frame::FOREGROUND, task).is_err() {
            let _ = self.command(frame::FOREGROUND, env::unit::self_id());
            return Err(());
        }
        match pie::accord(
            self.authority,
            task,
            Permission::FETCH | Permission::VEST | Permission::ONLY,
            frame::AUTHORITY,
        ) {
            Ok(token) => Ok(Foreground {
                connection: self,
                task,
                token,
                active: true,
            }),
            Err(_) => {
                let _ = self.command(frame::FOREGROUND, env::unit::self_id());
                Err(())
            }
        }
    }
    pub fn set_echo(&self, enabled: bool) -> Result<(), ()> {
        self.command(
            if enabled {
                frame::ECHO_ON
            } else {
                frame::ECHO_OFF
            },
            env::unit::self_id(),
        )
    }
    pub fn close(&self) -> Result<(), ()> {
        self.command(frame::DETACH, env::unit::self_id())
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        let _ = pie::release(self.root, env::ReleaseMode::Revoke);
    }
}
