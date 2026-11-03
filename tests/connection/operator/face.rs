use crate::{PieToken, TaskId, Wait};
use system_api::operator::{Entry, EntryId, EntryTarget, NamespaceKey, Path};

use super::Tile;

pub struct Face {
    pub service: Option<Entry>,
    pub attach: Option<PieToken>,
}
impl Face {
    pub fn resolve(&self, key: NamespaceKey, _: Wait) -> Result<Entry, ()> {
        if key == system_api::namespace::SERVICE {
            self.service.ok_or(())
        } else {
            Err(())
        }
    }
    pub fn tile_from(&self, base: Entry, road: &Path, _: Wait) -> Result<Tile, ()> {
        let service = self.service.ok_or(())?;
        if base != service || road.as_str() != terminal_api::ENTRY_PATH {
            return Err(());
        }
        self.attach.map(Tile).ok_or(())
    }
}
pub fn fixture_base() -> Entry {
    Entry {
        operator: TaskId::new(31),
        target: EntryTarget::Node(EntryId::new(0)),
    }
}
