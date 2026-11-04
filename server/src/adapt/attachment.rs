use super::endpoints::Endpoints;
use env::{Permission, PieToken, TaskId, pie};
use terminal_api::frame;

pub(super) struct Attachment {
    pub(super) foreground: TaskId,
    pub(super) endpoints: Endpoints,
    pub(super) authority: PieToken,
    pub(super) grants: [PieToken; 3],
}
impl Attachment {
    pub(super) fn open(authority: PieToken, owner: TaskId) -> Result<Self, ()> {
        let mut attachment = Self {
            foreground: owner,
            authority,
            endpoints: Endpoints {
                input: PieToken::NONE,
                output: PieToken::NONE,
                control: PieToken::NONE,
            },
            grants: [PieToken::NONE; 3],
        };
        attachment.endpoints.input =
            env::hole::unseal(frame::INPUT, env::HoleLimits::default()).map_err(|_| ())?;
        attachment.endpoints.output =
            env::hole::unseal(frame::OUTPUT, env::HoleLimits::default()).map_err(|_| ())?;
        attachment.endpoints.control =
            env::hole::unseal(frame::CONTROL, env::HoleLimits::default()).map_err(|_| ())?;
        attachment.grant(owner)?;
        Ok(attachment)
    }
    pub(super) fn revoke(&mut self) -> Result<(), ()> {
        for token in &mut self.grants {
            if *token == PieToken::NONE {
                continue;
            }
            match pie::revoke(self.foreground, *token) {
                Ok(()) => {}
                // No transfer rights were given, so a missing direct child is already gone.
                Err(error) if error.source == env::PieFail::Denied => {}
                Err(_) => return Err(()),
            }
            *token = PieToken::NONE;
        }
        Ok(())
    }
    pub(super) fn grant(&mut self, task: TaskId) -> Result<(), ()> {
        self.foreground = task;
        for (index, (token, rights, mark)) in [
            (self.endpoints.input, Permission::FETCH, frame::INPUT),
            (self.endpoints.output, Permission::STORE, frame::OUTPUT),
            (self.endpoints.control, Permission::FETCH, frame::CONTROL),
        ]
        .into_iter()
        .enumerate()
        {
            match pie::accord(token, task, rights, mark) {
                Ok(token) => self.grants[index] = token,
                Err(_) => {
                    let _ = self.revoke();
                    return Err(());
                }
            }
        }
        Ok(())
    }
}
impl Drop for Attachment {
    fn drop(&mut self) {
        for token in [
            self.endpoints.input,
            self.endpoints.output,
            self.endpoints.control,
            self.authority,
        ] {
            if token != PieToken::NONE {
                let _ = pie::release(token, env::ReleaseMode::Revoke);
            }
        }
    }
}
