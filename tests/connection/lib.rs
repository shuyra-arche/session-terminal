//! Exercise the production Connection with a service that revokes and replaces data grants.
#![allow(dead_code)]
extern crate alloc;
extern crate self as env;
extern crate self as ipc;
extern crate self as resource;
extern crate self as system_client;
pub use abi::{
    HoleCondition, HoleFail, HoleLimits, HoleResult, Mark, Permission, PieToken, ReleaseMode,
    TaskId, ToleEventSource, ToleResult, Wait,
};
pub mod wire {
    pub use abi::wire::*;
}

mod error;
mod native;
mod production;

pub use error::Error;
pub(crate) use error::denied;
pub(crate) use native::{mint, reset, token, NATIVE};

pub mod debug {
    pub fn put(_: &str) {}
}
pub mod unit {
    pub fn self_id() -> super::TaskId {
        super::TaskId::new(32)
    }
}
pub mod hole {
    pub fn unseal(_: crate::Mark, _: crate::HoleLimits) -> crate::HoleResult<crate::PieToken> {
        Ok(crate::mint())
    }
}
pub mod tole {
    pub fn unseal(_: bool) -> crate::ToleResult<crate::PieToken> {
        Ok(crate::token(1))
    }
}
pub mod pie {
    use crate::{mint, Mark, Permission, PieToken, TaskId, NATIVE};

    pub fn accord(source: PieToken, _: TaskId, _: Permission, _: Mark) -> Result<PieToken, ()> {
        let remote = mint();
        NATIVE.with(|n| n.borrow_mut().loans.insert(remote.get(), source.get()));
        Ok(remote)
    }
    pub fn release(_: PieToken, _mode: crate::ReleaseMode) -> Result<(), ()> {
        Ok(())
    }
    pub fn revoke(_: TaskId, _: PieToken) -> Result<(), ()> {
        Ok(())
    }
}
pub mod session {
    pub mod establish {
        pub fn find(
            _: super::super::TaskId,
            _: super::super::Mark,
        ) -> Result<super::super::PieToken, ()> {
            panic!("Connection must use the acknowledged seeds, not indexed discovery")
        }
    }
}

pub mod operator;
pub mod raw;
pub mod pile;

pub mod chrono {
    pub fn clock() -> u64 {
        0
    }
}
#[path = "time.rs"]
pub mod time;
