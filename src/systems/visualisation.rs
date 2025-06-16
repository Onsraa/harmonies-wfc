use bevy::prelude::*;
use crate::components::{
    hex_coord::HexCoord,
    tile::{Tile, TileType},
};

/// Marqueur pour les entités visuelles des tuiles
#[derive(Component)]
pub struct TileVisual;

/// Resource pour stocker le mesh partagé
#[derive(Resource)]
pub struct SharedMeshes {
    pub cylinder: Handle<Mesh>,
}

/// Système d'initialisation des meshes partagés
pub fn setup_shared_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cylinder = meshes.add(Cylinder::new(0.4, 0.3));
    commands.insert_resource(SharedMeshes { cylinder });
}

/// Système de rendu des tuiles optimisé - ne recrée que ce qui est nécessaire
pub fn render_tiles_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    shared_meshes: Res<SharedMeshes>,
    new_tiles: Query<(Entity, &Tile), Without<TileVisual>>,
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

    // Crée uniquement les visuels pour les NOUVELLES tuiles
    for (entity, tile) in new_tiles.iter() {
        // Ne pas afficher les tuiles Empty
        if tile.tile_type == TileType::Empty {
            continue;
        }

        // Couleur selon le type de tuile
        let color = match tile.tile_type {
            TileType::City => Color::srgb(1.0, 0.0, 0.0),    // Rouge
            TileType::River => Color::srgb(0.0, 0.5, 1.0),   // Bleu
            TileType::Rock => Color::srgb(0.5, 0.5, 0.5),    // Gris
            TileType::Trunk => Color::srgb(0.4, 0.2, 0.0),   // Marron
            TileType::Leaves => Color::srgb(0.0, 0.8, 0.0),  // Vert
            TileType::Field => Color::srgb(1.0, 1.0, 0.0),   // Jaune
            TileType::Empty => Color::NONE,
        };

        let material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        // Position dans le monde
        let world_pos = tile.coord.to_world_pos();

        // Crée le cylindre - utilise le mesh partagé
        commands.spawn((
            TileVisual,
            Mesh3d(shared_meshes.cylinder.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos),
        ));

        // Marque l'entité tuile comme ayant un visuel
        commands.entity(entity).insert(TileVisual);
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