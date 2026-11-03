use env::{PieToken, TaskId, pie};
use terminal_api::frame;

use super::Connection;

pub struct Foreground<'a> {
    pub(super) connection: &'a Connection,
    pub(super) task: TaskId,
    pub(super) token: PieToken,
    pub(super) active: bool,
}
impl Foreground<'_> {
    fn reclaim(&mut self) -> Result<(), ()> {
        // A completed task has already released the child capability.
        let _ = pie::revoke(self.task, self.token);
        self.connection
            .command(frame::FOREGROUND, env::unit::self_id())?;
        self.active = false;
        Ok(())
    }
    pub fn restore(mut self) -> Result<(), ()> {
        self.reclaim()
    }
}
impl Drop for Foreground<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.reclaim();
        }
    }
}
