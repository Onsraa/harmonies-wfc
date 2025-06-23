pub type TileId = u8;
pub type TilePossibilities = u16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WfcCell {
    pub possibilities: TilePossibilities,
    pub entropy: u8,
}

impl WfcCell {
    pub(crate) fn new_with_all_possibilities(tile_count: u8) -> Self {
        let possibilities = (1u16 << tile_count) - 1;
        Self {
            possibilities,
            entropy: tile_count,
        }
    }

    #[inline]
    pub(crate) fn is_possible(&self, tile_id: TileId) -> bool {
        self.possibilities & (1 << tile_id) != 0
    }

    fn remove_possibility(&mut self, tile_id: TileId) {
        if self.is_possible(tile_id) {
            self.possibilities &= !(1 << tile_id);
            self.entropy -= 1;
        }
    }

    #[inline]
    pub(crate) fn entropy(&self) -> u8 {
        self.entropy
    }

    #[inline]
    pub(crate) fn is_collapsed(&self) -> bool {
        self.entropy == 1
    }

    #[inline]
    pub fn is_contradiction(&self) -> bool {
        self.possibilities == 0
    }

    pub(crate) fn get_collapsed_tile(&self) -> Option<TileId> {
        if self.is_collapsed() {
            Some(self.possibilities.trailing_zeros() as TileId)
        } else {
            None
        }
    }
}
