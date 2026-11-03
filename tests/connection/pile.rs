use crate::{PieToken, ToleEventSource, Wait, NATIVE};

pub struct Pile;

impl Pile {
    pub fn unseal(_: bool) -> Result<Self, ()> {
        Ok(Self)
    }
    pub fn attach(&self, source: ToleEventSource) -> Result<(), ()> {
        let ToleEventSource::Hole { token, .. } = source else {
            return Err(());
        };
        NATIVE.with(|n| n.borrow().live.contains(&token.get()).then_some(()).ok_or(()))
    }
    pub fn await_(&self, _: Wait) -> Result<(), ()> {
        unreachable!()
    }
    pub fn token(&self) -> PieToken {
        crate::token(1)
    }
}
