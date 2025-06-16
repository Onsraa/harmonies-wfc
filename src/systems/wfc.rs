use bevy::prelude::*;
use crate::components::tile::{Tile, TileType};
use crate::resources::grid::HexGrid;
use crate::wfc::solver::WfcSolver;

/// Événement pour déclencher une nouvelle génération
#[derive(Event)]
pub struct GenerateWorldEvent;

/// Système principal de génération WFC
pub fn wfc_generation_system(
    mut commands: Commands,
    mut grid: ResMut<HexGrid>,
    mut solver: ResMut<WfcSolver>,
    mut events: EventReader<GenerateWorldEvent>,
    tile_query: Query<Entity, With<Tile>>,
) {
    for _event in events.read() {
        info!("Démarrage de la génération WFC...");

        // Supprime les anciennes tuiles
        for entity in tile_query.iter() {
            commands.entity(entity).despawn();
        }

        // Vide la grille
        grid.clear();

        // Réinitialise le solveur
        *solver = WfcSolver::new(grid.width, grid.height);

        // Lance la résolution
        match solver.solve() {
            Ok(()) => {
                info!("Génération WFC réussie!");

                // Transfère les tuiles du solveur vers la grille
                for (coord, cell) in &solver.cells {
                    if let Some(tile_type) = cell.get_tile_type() {
                        if tile_type != TileType::Empty {
                            grid.set_tile(*coord, tile_type);

                            // Crée l'entité Bevy pour la tuile
                            commands.spawn(Tile {
                                coord: *coord,
                                tile_type,
                            });
                        }
                    }
                }

                info!("Nombre de tuiles générées: {}", grid.tiles.len());
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
    grid: Res<HexGrid>,
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
        info!("Rivières: {} segments", solver.river_constraints.river_positions.len());
    }
}