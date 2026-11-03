use ::resource::raw::{Hole, reserve};
use env::wire::Span as _;
use env::{Permission, PieToken, TaskId, Wait, pie};
use system_client::operator::Face;
use terminal_api::frame::{self, Command, Reply};
use wire::message::Message;

pub(super) const MS: Wait = Wait::AtMost(1000);

pub struct Terminal {
    pub(super) entry: PieToken,
    pub(super) host: TaskId,
}
impl Terminal {
    pub fn find(tree: &Face) -> Result<Self, ()> {
        let base = tree
            .resolve(system_api::namespace::SERVICE, MS)
            .map_err(|_| ())?;
        let entry = tree
            .tile_from(
                base,
                system_api::operator::Path::new(terminal_api::ENTRY_PATH),
                MS,
            )
            .and_then(|tile| tile.token(MS))
            .map_err(|_| ())?;
        let host = match reserve(entry) {
            Ok((_, host, _)) => host,
            Err(_) => {
                let _ = pie::release(entry, env::ReleaseMode::Revoke);
                return Err(());
            }
        };
        Ok(Self { entry, host })
    }
    pub(super) fn call(&self, mut command: Command) -> Result<Reply, ()> {
        let back = env::hole::unseal(frame::BACK, env::HoleLimits::default()).map_err(|_| ())?;
        let result = (|| {
            command.back =
                pie::accord(back, self.host, Permission::STORE, frame::BACK).map_err(|_| ())?;
            let mut bytes = [0; Command::LEN];
            let n = command.store_at(&mut bytes, 0).ok_or(())?;
            Hole::from_raw(self.entry)
                .push(&bytes[..n], MS)
                .map_err(|error| {
                    crate::debug::put(&alloc::format!("terminal client: command push {:?}", error));
                    ()
                })?;
            let mut bytes = [0; Reply::LEN];
            let (n, from) = Hole::from_raw(back).pull(&mut bytes, MS).map_err(|_| ())?;
            let (reply, end) = Reply::fetch_at(&bytes[..n], 0).ok_or(())?;
            if from == self.host && n == end && reply.status == 0 {
                Ok(reply)
            } else {
                Err(())
            }
        })();
        let _ = pie::release(back, env::ReleaseMode::Revoke);
        result
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = pie::release(self.entry, env::ReleaseMode::Revoke);
    }
}
