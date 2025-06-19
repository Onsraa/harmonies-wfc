use crate::components::grid::hex::Hex;
use crate::components::grid::tile::tile_type::TileType;
use crate::components::grid::tile::{Tile, TileStack};
use crate::components::hex_coord::HexCoord;
use crate::resources::grid::{GridLayout, TileGrid};
use crate::resources::tile_weights::TileWeights;
use crate::wfc::cell::CellCoord;
use crate::wfc::solver::WfcSolver;
use bevy::prelude::*;

/// Événement pour déclencher une nouvelle génération
#[derive(Event)]
pub struct GenerateWorldEvent;

/// Système principal de génération WFC
pub fn wfc_generation_system(
    mut commands: Commands,
    mut solver: ResMut<WfcSolver>,
    mut grid: ResMut<TileGrid>,
    mut grid_layout: ResMut<GridLayout>,
    mut events: EventReader<GenerateWorldEvent>,
    tile_query: Query<Entity, With<Tile>>,
    mut hexes_query: Query<(&Hex, &mut TileStack)>,
) {
    for _event in events.read() {
        info!("Démarrage de la génération WFC avec pondération...");

        for entity in tile_query.iter() {
            commands.entity(entity).despawn();
        }
        grid.clear();

        *solver = WfcSolver::new(
            grid_layout.width * 2,
            grid_layout.length * 2,
            grid_layout.max_height,
        );
        match solver.solve() {
            Ok(()) => {
                info!("Génération WFC réussie!");
                for (coord, cell) in &solver.cells {
                    if let Some(tile_type) = cell.get_tile_type() {
                        if tile_type != TileType::Empty {
                            grid.set_tile(coord.clone(), tile_type);
                        }
                    }
                }
                for (hex, mut tile_stack) in hexes_query.iter_mut() {
                    tile_stack.tiles.clear();
                    for height in 0..=grid_layout.max_height {
                        let coord = CellCoord(hex.q(), hex.r(), height);
                        if let Some(tile_type) = grid.get_tile(&coord) {
                            let tile_entity = commands
                                .spawn(Tile {
                                    tile_type: *tile_type,
                                })
                                .id();
                            tile_stack.tiles.push(tile_entity);
                        }
                    }
                }
                info!("Nombre de tuiles générées: {}", grid.tiles.len());

                let river_count = grid.count_tiles_of_type(TileType::River);
                info!("Rivières générées: {} segments", river_count);
            }
            Err(e) => {
                error!("Erreur de génération WFC: {}", e);
            }
        }
    }
}

/// Système pour afficher les statistiques de génération (touche F1)
pub fn wfc_debug_system(
    solver: Res<WfcSolver>,
    grid: Res<TileGrid>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!("=== Statistiques de génération ===");
        info!("Cellules totales: {}", solver.cells.len());
        info!("Tuiles placées: {}", grid.tiles.len());

        // Compte par type
        for tile_type in TileType::all() {
            let count = grid.count_tiles_of_type(*tile_type);
            if count > 0 {
                info!("{:?}: {}", tile_type, count);
            }
        }

        // Statistiques rivières
        info!(
            "Rivières: {} segments",
            solver.river_constraints.river_positions.len()
        );
    }
}
