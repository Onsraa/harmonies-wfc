use crate::wfc::v2::cell::{TileId, TilePossibilities, WfcCell};
use crate::wfc::v2::{
    Direction, CITY, CITY_TOP, DOWN, EAST, EMPTY, FIELD, LEAVES, NORTHEAST, NORTHWEST, RIVER, ROCK,
    ROCK_TOP, SOUTHEAST, SOUTHWEST, TILE_COUNT, TRUNK, UP, WEST,
};

pub(crate) struct WfcSolver {
    pub(crate) adjacency: [TilePossibilities; 72], // 9 tiles * 8 directions
    tile_count: u8,
}

impl WfcSolver {
    pub(crate) fn new() -> Self {
        Self {
            adjacency: Self::setup_hexagonal_rules(),
            tile_count: TILE_COUNT,
        }
    }

    pub fn get_allowed_neighbor_tiles(
        &self,
        cell: &WfcCell,
        direction: Direction,
    ) -> TilePossibilities {
        let mut allowed_tiles = 0;
        for tile_id in 0..=self.tile_count {
            if cell.is_possible(tile_id) {
                allowed_tiles |= self.adjacency[idx(tile_id, direction)];
            }
        }

        allowed_tiles
    }

    pub fn collapse_cell(&self, cell: &mut WfcCell, chosen_tile: TileId) {
        cell.possibilities = 1 << chosen_tile;
        cell.entropy = 1;
    }

    pub fn propagate_to_cell(&self, cell: &mut WfcCell, constraints: TilePossibilities) -> bool {
        let old_possibilities = cell.possibilities;
        cell.possibilities &= constraints;

        if cell.possibilities != old_possibilities {
            cell.entropy = cell.possibilities.count_ones() as u8;
            true
        } else {
            false
        }
    }

    fn setup_hexagonal_rules() -> [TilePossibilities; 72] {
        let mut adjacency = [0u16; 72];
        // FIX: Include all 9 tiles (0-8) in the mask
        const ALL_TILES: TilePossibilities = 0x1FF; // 511 in decimal, bits for tiles 0-8

        for tile_id in 0..=TILE_COUNT {
            for hex_dir in 0..6 {
                adjacency[idx(tile_id, hex_dir)] = ALL_TILES;
            }
        }
        // EMPTY tile rules
        adjacency[idx(EMPTY, UP)] = 1 << EMPTY;
        adjacency[idx(EMPTY, DOWN)] = ALL_TILES;
        for dir in 0..6 {
            adjacency[idx(EMPTY, dir)] = ALL_TILES;
        }

        // RIVER
        adjacency[idx(RIVER, UP)] = 1 << EMPTY;
        adjacency[idx(RIVER, DOWN)] = 0;
        adjacency[idx(RIVER, EAST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);
        adjacency[idx(RIVER, WEST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);
        adjacency[idx(RIVER, NORTHEAST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);
        adjacency[idx(RIVER, NORTHWEST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);
        adjacency[idx(RIVER, SOUTHWEST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);
        adjacency[idx(RIVER, SOUTHEAST)] = (1 << FIELD) | (1 << RIVER) | (1 << ROCK_TOP);

        // FIELD
        adjacency[idx(FIELD, UP)] = 1 << EMPTY;
        adjacency[idx(FIELD, DOWN)] = 0;

        // TRUNK
        adjacency[idx(TRUNK, UP)] = (1 << TRUNK) | (1 << LEAVES) | (1 << CITY);
        adjacency[idx(TRUNK, DOWN)] = 1 << TRUNK;
        adjacency[idx(TRUNK, EAST)] &= !(1 << RIVER);
        // LEAVES
        adjacency[idx(LEAVES, UP)] = 1 << EMPTY;
        adjacency[idx(LEAVES, DOWN)] = 1 << TRUNK;
        adjacency[idx(LEAVES, EAST)] &= !(1 << RIVER);

        // CITY
        adjacency[idx(CITY, UP)] = (1 << CITY) | (1 << CITY_TOP);
        adjacency[idx(CITY, DOWN)] = (1 << CITY) | (1 << TRUNK) | (1 << ROCK);
        adjacency[idx(CITY, EAST)] &= !(1 << RIVER);
        // CITY TOP
        adjacency[idx(CITY_TOP, UP)] = 1 << EMPTY;
        adjacency[idx(CITY_TOP, DOWN)] = 1 << CITY;
        adjacency[idx(CITY_TOP, EAST)] &= !(1 << RIVER);

        // ROCK
        adjacency[idx(ROCK, UP)] = (1 << ROCK) | (1 << ROCK_TOP) | (1 << CITY);
        adjacency[idx(ROCK, DOWN)] = 1 << ROCK;
        adjacency[idx(ROCK, EAST)] &= !(1 << RIVER);
        // ROCK TOP
        adjacency[idx(ROCK_TOP, UP)] = 1 << EMPTY;
        adjacency[idx(ROCK_TOP, DOWN)] = 1 << ROCK;

        adjacency
    }
}

pub fn idx(tile_id: TileId, direction: Direction) -> usize {
    ((tile_id * 8) + direction) as usize
}
