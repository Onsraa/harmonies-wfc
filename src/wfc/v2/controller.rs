use crate::components::grid::hex::Hex;
use crate::wfc::v2::cell::TileId;
use crate::wfc::v2::grid::WfcHexGrid;
use crate::wfc::v2::solver::{idx, WfcSolver};
use crate::wfc::v2::{get_opposite_direction, Direction, WfcError, TILE_COUNT};
use bevy::prelude::Resource;
use rand::Rng;
use std::collections::HashSet;

#[derive(Resource)]
pub struct WfcController {
    solver: WfcSolver,
    pub grid: WfcHexGrid,
    collapse_order: Vec<usize>,
}

impl WfcController {
    pub fn from_hexes(hexes: Vec<Hex>) -> Self {
        let grid = WfcHexGrid::new(hexes);
        let solver = WfcSolver::new();
        Self {
            solver,
            grid,
            collapse_order: Vec::new(),
        }
    }

    pub fn step(&mut self) -> Result<bool, WfcError> {
        let (cell_idx, _) = self
            .grid
            .find_lowest_entropy_cell()
            .ok_or(WfcError::Complete)?;

        let chosen_tile = self.choose_random_tile(cell_idx)?;

        // BEFORE collapsing, check if this choice would create contradictions
        // with already-collapsed neighbors
        if let Err(e) = self.check_collapsed_neighbors_compatibility(cell_idx, chosen_tile) {
            self.grid.cells[cell_idx].remove_possibility(chosen_tile);
            if self.grid.cells[cell_idx].is_contradiction() {
                eprintln!(
                    "Contradiction found at cell {} at level {}: no valid tiles",
                    cell_idx, self.grid.index_to_hex[cell_idx].level
                );
                return Err(WfcError::NoValidTiles);
            }
            return self.step();
        }

        self.solver
            .collapse_cell(&mut self.grid.cells[cell_idx], chosen_tile);

        self.collapse_order.push(cell_idx);
        self.propagate_from(cell_idx)?;

        Ok(true)
    }

    fn check_collapsed_neighbors_compatibility(
        &self,
        cell_idx: usize,
        chosen_tile: TileId,
    ) -> Result<(), WfcError> {
        let neighbors: Vec<(usize, Direction)> = self.grid.get_neighbors(cell_idx).collect();

        for (neighbor_idx, direction) in neighbors {
            let neighbor_cell = &self.grid.cells[neighbor_idx];

            if let Some(neighbor_tile) = neighbor_cell.get_collapsed_tile() {
                // What does our chosen tile require in this direction?
                let our_requirement = self.solver.adjacency[idx(chosen_tile, direction)];

                // Does the neighbor satisfy this requirement?
                if our_requirement & (1 << neighbor_tile) == 0 {
                    // eprintln!(
                    //     "Cannot place {} at {} because neighbor {} is {} but we require {:b}",
                    //     chosen_tile, cell_idx, neighbor_idx, neighbor_tile, our_requirement
                    // );
                    return Err(WfcError::NoValidTiles);
                }

                // Also check reverse: does the neighbor accept us?
                let opposite_dir = get_opposite_direction(direction);
                let neighbor_accepts = self.solver.adjacency[idx(neighbor_tile, opposite_dir)];

                if neighbor_accepts & (1 << chosen_tile) == 0 {
                    // eprintln!(
                    //     "Cannot place {} at {} because neighbor {} with {} doesn't accept us from direction {}",
                    //     chosen_tile, cell_idx, neighbor_idx, neighbor_tile, opposite_dir
                    // );
                    return Err(WfcError::NoValidTiles);
                }
            }
        }

        Ok(())
    }

    fn handle_contradiction(&mut self, failed_idx: usize) -> Result<(), WfcError> {
        let mut repair_zone = HashSet::new();
        repair_zone.insert(failed_idx);

        let repair_distance = 3;
        for dist in 1..=repair_distance {
            let cells_at_dist = self.get_cells_at_distance(failed_idx, dist);
            for cell_idx in cells_at_dist {
                if self.was_recently_collapsed(cell_idx) {
                    repair_zone.insert(cell_idx);
                }
            }
        }

        for &idx in &repair_zone {
            self.grid.get_cell_mut(idx).reset();
            self.collapse_order.retain(|&x| x != idx);
        }

        self.propagate_from_boundary(&repair_zone)?;

        Ok(())
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
                    self.handle_contradiction(neighbor_index)?;
                } else if self
                    .solver
                    .propagate_to_cell(&mut self.grid.cells[neighbor_index], compatible_tiles)
                {
                    queue.push(neighbor_index);
                }
            }
        }

        Ok(())
    }

    fn propagate_from_boundary(&mut self, repair_zone: &HashSet<usize>) -> Result<(), WfcError> {
        // Find boundary cells: cells outside repair zone that neighbor repair zone cells
        let mut boundary_cells = HashSet::new();

        for &repair_cell_idx in repair_zone {
            for (neighbor_idx, _) in self.grid.get_neighbors(repair_cell_idx) {
                if !repair_zone.contains(&neighbor_idx) {
                    boundary_cells.insert(neighbor_idx);
                }
            }
        }

        // Start propagation from all boundary cells
        let mut queue: Vec<usize> = boundary_cells.into_iter().collect();
        let mut visited = HashSet::new();

        while let Some(cell_index) = queue.pop() {
            if visited.contains(&cell_index) {
                continue;
            }
            visited.insert(cell_index);

            let neighbors: Vec<(usize, Direction)> = self.grid.get_neighbors(cell_index).collect();

            for (neighbor_index, direction) in neighbors {
                if !repair_zone.contains(&neighbor_index)
                    && self.grid.cells[neighbor_index].is_collapsed()
                {
                    continue;
                }

                let allowed_in_direction = self
                    .solver
                    .get_allowed_neighbor_tiles(&self.grid.cells[cell_index], direction);

                let opposite_dir = get_opposite_direction(direction);

                let mut compatible_tiles = 0;
                for tile_id in 0..=TILE_COUNT {
                    if self.grid.cells[neighbor_index].is_possible(tile_id) {
                        let accepts_from_opposite =
                            self.solver.adjacency[idx(tile_id, opposite_dir)];
                        if accepts_from_opposite & allowed_in_direction != 0 {
                            compatible_tiles |= 1 << tile_id;
                        }
                    }
                }

                if compatible_tiles == 0 && self.grid.cells[neighbor_index].possibilities != 0 {
                    // Still contradiction - repair zone might be too small
                    eprintln!(
                        "Contradiction persists after repair at cell {}",
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

    fn get_cells_at_distance(&self, center: usize, distance: usize) -> Vec<usize> {
        let mut current_ring = vec![center];
        let mut visited = HashSet::new();
        visited.insert(center);

        for _ in 0..distance {
            let mut next_ring = Vec::new();
            for &cell_idx in &current_ring {
                for (neighbor_idx, _) in self.grid.get_neighbors(cell_idx) {
                    if visited.insert(neighbor_idx) {
                        next_ring.push(neighbor_idx);
                    }
                }
            }
            current_ring = next_ring;
        }

        current_ring
    }

    fn was_recently_collapsed(&self, cell_idx: usize) -> bool {
        let recency_threshold = 100;

        if let Some(position) = self.collapse_order.iter().rposition(|&idx| idx == cell_idx) {
            let collapses_ago = self.collapse_order.len() - position;
            collapses_ago <= recency_threshold
        } else {
            false
        }
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
