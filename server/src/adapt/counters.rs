use uart_client::Console;

pub(super) struct Counters {
    pub(super) seen: u64,
    pub(super) lost: u64,
    pub(super) cr: bool,
}
impl Counters {
    pub fn of(console: &Console) -> Self {
        Self {
            seen: console.rx.skipped(),
            lost: console.tx.lost(),
            cr: false,
        }
    }
}
