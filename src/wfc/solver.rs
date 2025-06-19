use super::cell::{CellCoord, WfcCell};
use crate::components::grid::tile::tile_type::TileType;
use crate::resources::river::RiverConstraints;
use crate::resources::tile_weights::TileWeights;
use bevy::prelude::*;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Resource)]
pub struct WfcSolver {
    pub cells: HashMap<CellCoord, WfcCell>,
    pub river_constraints: RiverConstraints,
    pub is_complete: bool,
}

impl WfcSolver {
    pub fn new(x: i32, y: i32, z: u8) -> Self {
        let mut cells = HashMap::new();
        for q in -x / 2..=x / 2 {
            for r in -y / 2..=y / 2 {
                if q.abs() + r.abs() + (-q - r).abs() <= x {
                    for level in 0..z {
                        let coord = CellCoord(q, r, level);
                        cells.insert(coord, WfcCell::new(level));
                    }
                }
            }
        }

        Self {
            cells,
            river_constraints: RiverConstraints::default(),
            is_complete: false,
        }
    }

    /// Lance la résolution complète
    pub fn solve(&mut self) -> Result<(), String> {
        // Boucle principale
        loop {
            // Trouve la cellule avec l'entropie minimale
            match self.find_min_entropy_cell() {
                Some(coord) => {
                    self.collapse_cell(&coord)?;
                    self.propagate(&coord)?;
                }
                None => {
                    // Plus de cellules à effondrer
                    self.is_complete = true;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Trouve la cellule non effondrée avec l'entropie minimale
    fn find_min_entropy_cell(&self) -> Option<CellCoord> {
        self.cells
            .iter()
            .filter(|(_, cell)| !cell.collapsed && !cell.possibilities.is_empty())
            .min_by(|(_, a), (_, b)| a.entropy().partial_cmp(&b.entropy()).unwrap())
            .map(|(coord, _)| coord.clone())
    }

    /// Effondre une cellule en choisissant une possibilité
    fn collapse_cell(&mut self, coord: &CellCoord) -> Result<(), String> {
        let cell = self.cells.get(coord).ok_or("Cellule introuvable")?;

        if cell.possibilities.is_empty() {
            return Err("Cellule sans possibilités".to_string());
        }

        // Filtre les possibilités selon les contraintes spéciales
        let valid_possibilities: Vec<_> = cell
            .possibilities
            .iter()
            .filter(|&&tile_type| {
                match tile_type {
                    TileType::River => {
                        // Vérifie les contraintes rivière
                        self.river_constraints.can_place_river(coord.0, coord.1)
                    }
                    _ => true,
                }
            })
            .cloned()
            .collect();

        if valid_possibilities.is_empty() {
            // Si aucune possibilité valide, on met Empty si on est en hauteur
            if coord.2 > 0 {
                let cell = self.cells.get_mut(&coord).unwrap();
                cell.possibilities.clear();
                cell.possibilities.insert(TileType::Empty);
                cell.collapsed = true;
                return Ok(());
            }
            return Err("Aucune possibilité valide".to_string());
        }

        // Choisit aléatoirement parmi les possibilités valides
        use rand::Rng;
        let mut rng = rand::rng();
        let chosen = valid_possibilities[rng.random_range(0..valid_possibilities.len())];

        // Met à jour la cellule
        let cell = self.cells.get_mut(&coord).unwrap();
        cell.possibilities.clear();
        cell.possibilities.insert(chosen);
        cell.collapsed = true;

        // Met à jour les contraintes globales
        if chosen == TileType::River {
            self.river_constraints
                .river_positions
                .insert((coord.0, coord.1));
        }

        Ok(())
    }

    /// Propage les contraintes après l'effondrement d'une cellule
    fn propagate(&mut self, start_coord: &CellCoord) -> Result<(), String> {
        let mut stack = VecDeque::new();
        stack.push_back(start_coord.clone());

        while let Some(current_coord) = stack.pop_front() {
            let current_cell = self.cells.get(&current_coord).unwrap();
            let current_type = if current_cell.collapsed && current_cell.possibilities.len() == 1 {
                *current_cell.possibilities.iter().next().unwrap()
            } else {
                continue;
            };

            // Propage aux voisins horizontaux
            for neighbor_coord in current_coord.neighbors() {
                if let Some(neighbor_cell) = self.cells.get(&neighbor_coord).cloned() {
                    if !neighbor_cell.collapsed {
                        let changed =
                            self.update_horizontal_possibilities(neighbor_coord.clone())?;

                        if changed && !stack.contains(&neighbor_coord) {
                            stack.push_back(neighbor_coord);
                        }
                    }
                }
            }

            // Propage verticalement
            if let Some(above_coord) = current_coord.above() {
                if let Some(above_cell) = self.cells.get(&above_coord).cloned() {
                    if !above_cell.collapsed {
                        self.update_vertical_possibilities(above_coord.clone(), current_type);

                        let cell = self.cells.get(&above_coord).unwrap();
                        if !cell.possibilities.is_empty() && !stack.contains(&above_coord) {
                            stack.push_back(above_coord);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Met à jour les possibilités selon ce qu'il y a en dessous
    fn update_vertical_possibilities(&mut self, coord: CellCoord, below_type: TileType) {
        if let Some(cell) = self.cells.get_mut(&coord) {
            let mut new_possibilities = HashSet::new();

            for tile_type in &cell.possibilities {
                if tile_type.can_be_placed_on(&below_type) {
                    new_possibilities.insert(*tile_type);
                }
            }

            cell.possibilities = new_possibilities;
        }
    }

    /// Met à jour les possibilités horizontales (rivières)
    fn update_horizontal_possibilities(&mut self, coord: CellCoord) -> Result<bool, String> {
        let cell = self.cells.get(&coord).cloned().unwrap();
        let old_count = cell.possibilities.len();

        let mut new_possibilities = HashSet::new();

        for &tile_type in &cell.possibilities {
            let valid = match tile_type {
                TileType::River => self.river_constraints.can_place_river(coord.0, coord.1),
                _ => true,
            };

            if valid {
                new_possibilities.insert(tile_type);
            }
        }

        if new_possibilities.is_empty() && coord.2 > 0 {
            new_possibilities.insert(TileType::Empty);
        }

        let cell = self.cells.get_mut(&coord).unwrap();
        cell.possibilities = new_possibilities;

        if cell.possibilities.is_empty() {
            return Err(format!("Contradiction à {:?}", coord));
        }

        Ok(old_count != cell.possibilities.len())
    }

    /// Obtient le type de tuile à une position (si effondrée)
    pub fn get_tile_at(&self, coord: &CellCoord) -> Option<TileType> {
        self.cells.get(coord).and_then(|cell| cell.get_tile_type())
    }

    fn collapse_cell_weighted(
        &mut self,
        coord: &CellCoord,
        tile_weights: &TileWeights,
    ) -> Result<(), String> {
        let cell = self.cells.get(&coord).ok_or("Cellule introuvable")?;

        if cell.possibilities.is_empty() {
            return Err("Cellule sans possibilités".to_string());
        }

        // Filtre les possibilités selon les contraintes spéciales
        let valid_possibilities: Vec<_> = cell
            .possibilities
            .iter()
            .filter(|&&tile_type| {
                match tile_type {
                    TileType::River => {
                        // Vérifie les contraintes rivière
                        self.river_constraints.can_place_river(coord.0, coord.1)
                    }
                    _ => true,
                }
            })
            .cloned()
            .collect();

        if valid_possibilities.is_empty() {
            // Si aucune possibilité valide, on met Empty si on est en hauteur
            if coord.2 > 0 {
                let cell = self.cells.get_mut(&coord).unwrap();
                cell.possibilities.clear();
                cell.possibilities.insert(TileType::Empty);
                cell.collapsed = true;
                return Ok(());
            }
            return Err("Aucune possibilité valide".to_string());
        }

        // Utilise le choix pondéré au lieu du choix aléatoire uniforme
        let chosen = tile_weights
            .weighted_choice(&valid_possibilities, coord.2)
            .ok_or("Erreur lors du choix pondéré")?;

        // Met à jour la cellule
        let cell = self.cells.get_mut(&coord).unwrap();
        cell.possibilities.clear();
        cell.possibilities.insert(chosen);
        cell.collapsed = true;

        // Met à jour les contraintes globales
        if chosen == TileType::River {
            self.river_constraints
                .river_positions
                .insert((coord.0, coord.1));
        }

        Ok(())
    }

    /// Lance la résolution avec pondération
    pub fn solve_weighted(&mut self, tile_weights: &TileWeights) -> Result<(), String> {
        // Boucle principale
        loop {
            // Trouve la cellule avec l'entropie minimale
            match self.find_min_entropy_cell() {
                Some(coord) => {
                    // Utilise la nouvelle méthode avec poids
                    self.collapse_cell_weighted(&coord, tile_weights)?;

                    // Propage les contraintes
                    self.propagate(&coord)?;
                }
                None => {
                    // Plus de cellules à effondrer
                    self.is_complete = true;
                    break;
                }
            }
        }

        Ok(())
    }
}
