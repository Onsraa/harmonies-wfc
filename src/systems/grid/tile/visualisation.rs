use crate::components::grid::hex::Hex;
use crate::components::grid::tile::tile_type::TileType;
use crate::components::grid::tile::{Tile, TileStack};
use crate::components::hex_coord::HexCoord;
use crate::globals::{TILE_GAP, TILE_HEIGHT};
use bevy::prelude::*;
use egui::ahash::HashMap;

/// Marqueur pour les entités visuelles des tuiles
#[derive(Component)]
pub struct TileVisual;

/// Resource pour stocker le mesh partagé
#[derive(Resource)]
pub struct SharedMeshes {
    pub cylinder: Handle<Mesh>,
}

/// Système d'initialisation des meshes partagés
pub fn setup_shared_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cylinder = meshes.add(Cylinder::new(1., TILE_HEIGHT));
    commands.insert_resource(SharedMeshes { cylinder });
}

/// Système de rendu des tuiles optimisé - ne recrée que ce qui est nécessaire
pub fn render_tiles_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    shared_meshes: Res<SharedMeshes>,
    new_tiles: Query<(Entity, &Tile), Without<TileVisual>>,
    mut hexes: Query<(&Hex, &Transform, &mut TileStack)>,
    mut events: EventReader<crate::systems::wfc::GenerateWorldEvent>,
    visual_query: Query<Entity, With<TileVisual>>,
) {
    // Ne supprime les visuels QUE lors d'une régénération
    if !events.is_empty() {
        for entity in visual_query.iter() {
            commands.entity(entity).despawn();
        }
        events.clear();
    }

    let mut tile_to_material: HashMap<Entity, Handle<StandardMaterial>> = HashMap::default();
    for (entity, tile) in new_tiles.iter() {
        if tile.tile_type == TileType::Empty {
            continue;
        }
        let color = match tile.tile_type {
            TileType::City => Color::srgb(1.0, 0.0, 0.0),   // Rouge
            TileType::River => Color::srgb(0.0, 0.5, 1.0),  // Bleu
            TileType::Rock => Color::srgb(0.5, 0.5, 0.5),   // Gris
            TileType::Trunk => Color::srgb(0.4, 0.2, 0.0),  // Marron
            TileType::Leaves => Color::srgb(0.0, 0.8, 0.0), // Vert
            TileType::Field => Color::srgb(1.0, 1.0, 0.0),  // Jaune
            TileType::Empty => Color::NONE,
        };

        let material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        tile_to_material.insert(entity, material);
    }

    for (hex, transform, tile_stack) in hexes.iter_mut() {
        for (i, tile) in tile_stack.tiles.iter().enumerate() {
            if let Some(material) = tile_to_material.get(tile) {
                commands.entity(*tile).insert((
                    TileVisual,
                    Mesh3d(shared_meshes.cylinder.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(
                        transform.translation.x,
                        TILE_HEIGHT / 2.0 + i as f32 * (TILE_HEIGHT + TILE_GAP),
                        transform.translation.z,
                    ),
                ));
            }
        }
    }
}

/// Système pour gérer la touche G et régénérer
pub fn regenerate_world_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<crate::systems::wfc::GenerateWorldEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        info!("Régénération du monde...");
        events.write(crate::systems::wfc::GenerateWorldEvent);
    }
}
