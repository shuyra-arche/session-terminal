use crate::wire::Span as _;
use crate::{denied, unit, Error, HoleCondition, PieToken, TaskId, Wait, NATIVE};
use terminal_api::frame::{Command, Reply};

pub struct Hole(PieToken);

impl Hole {
    pub fn from_raw(t: PieToken) -> Self {
        Self(t)
    }
    pub fn push(&self, bytes: &[u8], _: Wait) -> Result<(), Error> {
        NATIVE.with(|n| {
            let mut n = n.borrow_mut();
            if self.0 != crate::token(5) {
                return n
                    .live
                    .contains(&self.0.get())
                    .then_some(())
                    .ok_or_else(denied);
            }
            let (command, _) = Command::fetch_at(bytes, 0).unwrap();
            let rejected = command.task.get() == usize::MAX;
            let channels = if rejected {
                [0; 3]
            } else {
                let old = n.channels;
                for t in old {
                    n.live.remove(&t);
                }
                if command.task == unit::self_id() {
                    let mut fresh = [0; 3];
                    for t in &mut fresh {
                        n.next += 1;
                        *t = n.next;
                        n.live.insert(*t);
                    }
                    n.channels = fresh;
                    fresh
                } else {
                    n.channels = [0; 3];
                    [0; 3]
                }
            };
            let reply = Reply {
                status: u8::from(rejected),
                authority: crate::token(50),
                input: crate::token(channels[0]),
                output: crate::token(channels[1]),
                control: crate::token(channels[2]),
            };
            let mut bytes = vec![0; Reply::LEN];
            reply.store_at(&mut bytes, 0).unwrap();
            let back = n.loans[&command.back.get()];
            n.replies.insert(back, bytes);
            Ok(())
        })
    }
    pub fn pull(&self, bytes: &mut [u8], _: Wait) -> Result<(usize, TaskId), Error> {
        NATIVE.with(|n| {
            let reply = n
                .borrow_mut()
                .replies
                .remove(&self.0.get())
                .ok_or_else(denied)?;
            bytes[..reply.len()].copy_from_slice(&reply);
            Ok((reply.len(), TaskId::new(31)))
        })
    }
    pub fn wait(&self, _: HoleCondition, _: Wait) -> Result<bool, Error> {
        Ok(true)
    }
}
