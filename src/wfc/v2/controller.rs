use crate::components::grid::hex::Hex;
use crate::wfc::v2::grid::WfcHexGrid;
use crate::wfc::v2::solver::{idx, WfcSolver};
use crate::wfc::v2::{get_opposite_direction, Direction, WfcError, TILE_COUNT};
use bevy::prelude::Resource;
use rand::Rng;

#[derive(Resource)]
pub struct WfcController {
    solver: WfcSolver,
    pub grid: WfcHexGrid,
}

impl WfcController {
    pub fn from_hexes(hexes: Vec<Hex>) -> Self {
        let grid = WfcHexGrid::new(hexes);
        let solver = WfcSolver::new();
        Self { solver, grid }
    }

    pub fn step(&mut self) -> Result<bool, WfcError> {
        let (cell_idx, _) = self
            .grid
            .find_lowest_entropy_cell()
            .ok_or(WfcError::Complete)?;

        let chosen_tile = self.choose_random_tile(cell_idx)?;
        self.solver
            .collapse_cell(&mut self.grid.cells[cell_idx], chosen_tile);

        self.propagate_from(cell_idx)?;

        Ok(true)
    }

    fn propagate_from(&mut self, start_idx: usize) -> Result<(), WfcError> {
        let mut queue = vec![start_idx];

        while let Some(cell_index) = queue.pop() {
            let neighbors: Vec<(usize, Direction)> = self.grid.get_neighbors(cell_index).collect();

            for (neighbor_index, direction) in neighbors {
                let allowed_in_direction = self
                    .solver
                    .get_allowed_neighbor_tiles(&self.grid.get_cell(cell_index), direction);

                let opposite_dir = get_opposite_direction(direction);

                let mut compatible_tiles = 0;
                for tile_id in 0..=TILE_COUNT {
                    if self.grid.get_cell(neighbor_index).is_possible(tile_id) {
                        let accepts_from_opposite =
                            self.solver.adjacency[idx(tile_id, opposite_dir)];
                        if accepts_from_opposite & allowed_in_direction != 0 {
                            compatible_tiles |= 1 << tile_id;
                        }
                    }
                }

                // Check for contradiction before propagating
                if compatible_tiles == 0 && self.grid.cells[neighbor_index].possibilities != 0 {
                    eprintln!(
                        "Contradiction found at neighbor {}: no compatible tiles",
                        neighbor_index
                    );
                    return Err(WfcError::NoValidTiles);
                }

                if self
                    .solver
                    .propagate_to_cell(&mut self.grid.cells[neighbor_index], compatible_tiles)
                {
                    queue.push(neighbor_index);
                }
            }
        }

        Ok(())
    }

    fn choose_random_tile(&self, cell_idx: usize) -> Result<u8, WfcError> {
        let cell = &self.grid.cells[cell_idx];

        if cell.is_contradiction() {
            eprintln!("Contradiction found at cell {}: no valid tiles", cell_idx);
            return Err(WfcError::NoValidTiles);
        }

        if cell.is_collapsed() {
            println!("Cell {} is already collapsed", cell_idx);
            return cell.get_collapsed_tile().ok_or(WfcError::NoValidTiles);
        }

        let mut rng = rand::rng();
        let mut choices = Vec::with_capacity(8);

        for tile_id in 0..TILE_COUNT {
            if cell.is_possible(tile_id) {
                choices.push(tile_id);
            }
        }

        Ok(choices[rng.random_range(0..choices.len())])
    }
}
