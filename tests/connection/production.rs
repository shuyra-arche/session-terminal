#[path = "../../client/src/terminal.rs"]
mod terminal;
#[path = "../../client/src/connection.rs"]
mod connection;
#[path = "../../client/src/foreground.rs"]
mod foreground;
#[path = "../../client/src/io.rs"]
mod io;
#[path = "../../client/src/read.rs"]
mod read;

pub use connection::Connection;
pub use foreground::Foreground;
pub use io::Io;
pub use read::Read;
pub use terminal::Terminal;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskId;
    fn open() -> Connection {
        crate::reset();
        Connection::open(Terminal {
            entry: crate::token(5),
            host: TaskId::new(31),
        })
        .unwrap()
    }
    #[test]
    fn find_resolves_service_key_then_terminal_relative_path() {
        crate::reset();
        let tree = crate::operator::Face {
            service: Some(crate::operator::fixture_base()),
            attach: Some(crate::token(5)),
        };
        let terminal = Terminal::find(&tree).unwrap();
        assert_eq!(terminal.host, TaskId::new(31));
    }
    #[test]
    fn find_rejects_missing_terminal_entry_after_resolving_service() {
        crate::reset();
        let tree = crate::operator::Face {
            service: Some(crate::operator::fixture_base()),
            attach: None,
        };
        assert!(Terminal::find(&tree).is_err());
    }
    #[test]
    fn attachment_uses_explicit_seeds_without_discovery() {
        let connection = open();
        connection.io().unwrap().write(b"attached").unwrap();
    }
    #[test]
    fn rejected_target_restores_fresh_seeds_and_revokes_old_io() {
        let mut connection = open();
        let old = connection.io().unwrap();
        assert!(connection.lend(TaskId::new(usize::MAX)).is_err());
        assert!(old.write(b"revoked").is_err());
        connection.io().unwrap().write(b"restored").unwrap();
    }
    #[test]
    fn foreground_restore_replaces_cached_seeds() {
        let mut connection = open();
        let old = connection.io().unwrap();
        let foreground = connection.lend(TaskId::new(9)).unwrap();
        assert!(old.write(b"revoked").is_err());
        foreground.restore().unwrap();
        connection.io().unwrap().write(b"restored").unwrap();
    }
}
