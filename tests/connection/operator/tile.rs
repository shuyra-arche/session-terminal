use crate::{PieToken, Wait};

pub struct Tile(pub(crate) PieToken);

impl Tile {
    pub fn token(&self, _: Wait) -> Result<PieToken, ()> {
        Ok(self.0)
    }
}
