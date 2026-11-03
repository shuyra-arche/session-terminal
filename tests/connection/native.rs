use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::PieToken;

#[derive(Default)]
pub struct Native {
    pub(crate) next: usize,
    pub(crate) live: BTreeSet<usize>,
    pub(crate) replies: BTreeMap<usize, Vec<u8>>,
    pub(crate) loans: BTreeMap<usize, usize>,
    pub(crate) channels: [usize; 3],
}

thread_local! {
    pub(crate) static NATIVE: RefCell<Native> =
        RefCell::new(Native { next: 100, ..Native::default() });
}

pub fn token(n: usize) -> PieToken {
    PieToken::mint(n)
}

pub fn mint() -> PieToken {
    NATIVE.with(|n| {
        let mut n = n.borrow_mut();
        n.next += 1;
        token(n.next)
    })
}

pub fn reset() {
    NATIVE.with(|n| {
        *n.borrow_mut() = Native {
            next: 100,
            ..Native::default()
        }
    });
}
