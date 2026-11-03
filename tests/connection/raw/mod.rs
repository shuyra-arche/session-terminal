mod hole;

use crate::{Mark, PieToken, TaskId};

pub use hole::Hole;

pub fn reserve(token: PieToken) -> Result<(TaskId, TaskId, Mark), ()> {
    if token == crate::token(5) {
        Ok((TaskId::new(31), TaskId::new(31), terminal_api::marks::ENTRY))
    } else {
        Err(())
    }
}
