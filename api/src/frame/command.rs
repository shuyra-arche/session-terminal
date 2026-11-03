use env::{PieToken, TaskId};

#[derive(Clone, Copy, env::Frame)]
pub struct Command {
    pub op: u8,
    pub task: TaskId,
    pub authority: PieToken,
    pub back: PieToken,
}
