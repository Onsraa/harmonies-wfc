use bevy::prelude::*;
use crate::components::{
    hex_coord::HexCoord,
    tile::{Tile, TileType},
};

/// Marqueur pour les entités visuelles des tuiles
#[derive(Component)]
pub struct TileVisual;

/// Système de rendu des tuiles sous forme de cylindres
pub fn render_tiles_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tile_query: Query<&Tile, Without<TileVisual>>,
    visual_query: Query<Entity, With<TileVisual>>,
) {
    // Supprime les anciennes visualisations quand on régénère
    for entity in visual_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Crée un cylindre pour chaque tuile
    let cylinder_mesh = meshes.add(Cylinder::new(0.4, 0.3));

    for tile in tile_query.iter() {
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

        // Crée le cylindre
        commands.spawn((
            TileVisual,
            Mesh3d(cylinder_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos),
        ));
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