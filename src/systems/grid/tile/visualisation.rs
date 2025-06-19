use crate::components::grid::hex::Hex;
use crate::components::grid::tile::tile_type::TileType;
use crate::components::grid::tile::{Tile, TileStack};
use crate::globals::{TILE_GAP, TILE_HEIGHT};
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy::render::render_resource::TextureViewDimension::Cube;
use bevy::scene::Scene;
use egui::ahash::{HashMap, HashMapExt};

/// Marqueur pour les entités visuelles des tuiles
#[derive(Component)]
pub struct TileVisual;

/// Resource pour stocker les scènes partagées
#[derive(Resource)]
pub struct SharedMeshes {
    pub meshes: HashMap<(TileType, u8), Handle<Scene>>,
}

pub fn setup_shared_meshes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut meshes = HashMap::new();

    // City
    meshes.insert(
        (TileType::City, 0),
        asset_server.load("tiles/city/tiles_ville.gltf#Scene0"),
    );
    meshes.insert(
        (TileType::City, 1),
        asset_server.load("tiles/city/tile_city_2.gltf#Scene0"),
    );
    meshes.insert(
        (TileType::City, 2),
        asset_server.load("tiles/city/tile_city_3.gltf#Scene0"),
    );

    // Rock
    meshes.insert(
        (TileType::Rock, 0),
        asset_server.load("tiles/rock/tile_rock.gltf#Scene0"),
    );
    meshes.insert(
        (TileType::Rock, 1),
        asset_server.load("tiles/rock/tile_rock_2.gltf#Scene0"),
    );
    meshes.insert(
        (TileType::Rock, 2),
        asset_server.load("tiles/rock/tile_rock_3.gltf#Scene0"),
    );

    // River
    meshes.insert(
        (TileType::River, 0),
        asset_server.load("tiles/water/tile_water.gltf#Scene0"),
    );

    // Trunk
    meshes.insert(
        (TileType::Trunk, 0),
        asset_server.load("tiles/tronc/tile_tronc.gltf#Scene0"),
    );

    // Field
    meshes.insert(
        (TileType::Field, 0),
        asset_server.load("tiles/field/tile_champ.gltf#Scene0"),
    );

    commands.insert_resource(SharedMeshes { meshes });
}

/// Système de rendu des tuiles optimisé - ne recrée que ce qui est nécessaire
pub fn render_tiles_system(
    mut commands: Commands,
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

    let mut tile_to_mesh: HashMap<Entity, Handle<Scene>> = HashMap::default();
    for (entity, tile) in new_tiles.iter() {
        if tile.tile_type == TileType::Empty {
            continue;
        }
        let level = match tile.tile_type {
            TileType::City | TileType::Rock => {
                if tile.top_level {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        };
        if let Some(scene) = shared_meshes.meshes.get(&(tile.tile_type, level)) {
            tile_to_mesh.insert(entity, scene.clone());
        }
    }

    for (hex, transform, tile_stack) in hexes.iter_mut() {
        for (i, tile) in tile_stack.tiles.iter().enumerate() {
            if let Some(scene_handle) = tile_to_mesh.get(tile) {
                commands.entity(*tile).insert((
                    TileVisual,
                    SceneRoot(scene_handle.clone()),
                    Transform::from_xyz(
                        transform.translation.x,
                        TILE_HEIGHT / 2.0 + i as f32 * (TILE_HEIGHT + TILE_GAP),
                        transform.translation.z,
                    )
                    .with_scale(Vec3::splat(2.0)),
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
