use crate::components::grid::hex::Hex;
use crate::wfc::v2::cell::WfcCell;
use crate::wfc::v2::HEX_3D_DIRECTIONS;
use crate::wfc::v2::{Direction, TILE_COUNT};
use std::collections::HashMap;

pub struct WfcHexGrid {
    pub(crate) cells: Vec<WfcCell>,
    pub hex_to_index: HashMap<Hex, usize>,
    pub index_to_hex: Vec<Hex>,
}

impl WfcHexGrid {
    pub fn new(hex_positions: Vec<Hex>) -> Self {
        let cell_count = hex_positions.len();
        let cells = vec![WfcCell::new_with_all_possibilities(TILE_COUNT + 1); cell_count];

        let mut hex_to_index = HashMap::new();
        for (index, hex) in hex_positions.iter().enumerate() {
            hex_to_index.insert(*hex, index);
        }

        Self {
            cells,
            hex_to_index,
            index_to_hex: hex_positions,
        }
    }

    #[inline]
    pub fn get_cell(&self, cell_idx: usize) -> &WfcCell {
        &self.cells[cell_idx]
    }

    #[inline]
    pub fn get_cell_mut(&mut self, cell_idx: usize) -> &mut WfcCell {
        &mut self.cells[cell_idx]
    }

    pub fn get_neighbors(&self, cell_idx: usize) -> impl Iterator<Item = (usize, Direction)> + '_ {
        let hex = self.index_to_hex[cell_idx];
        HEX_3D_DIRECTIONS
            .iter()
            .filter_map(move |(offset, direction)| {
                let neighbor_hex = hex + *offset;
                self.hex_to_index
                    .get(&neighbor_hex)
                    .map(|&idx| (idx, *direction))
            })
    }

    pub fn find_lowest_entropy_cell(&self) -> Option<(usize, u8)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.entropy() > 1)
            .min_by_key(|(_, cell)| cell.entropy())
            .map(|(idx, cell)| (idx, cell.entropy()))
    }

    pub fn get_hex_cell(&self, hex: &Hex) -> Option<&WfcCell> {
        self.hex_to_index
            .get(hex)
            .and_then(|&idx| self.cells.get(idx))
    }
}
