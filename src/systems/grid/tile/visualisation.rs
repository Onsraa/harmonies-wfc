use crate::components::grid::hex::Hex;
use crate::globals::{TILE_GAP, TILE_HEIGHT};
use crate::systems::wfc::v1::GenerateWorldEvent;
use crate::wfc::v2::cell::TileId;
use crate::wfc::v2::controller::WfcController;
use crate::wfc::v2::{CITY, CITY_TOP, FIELD, RIVER, ROCK, ROCK_TOP, TRUNK};
use bevy::prelude::*;
use bevy::scene::Scene;
use std::collections::HashMap;

/// Marqueur pour les entités visuelles des tuiles
#[derive(Component)]
pub struct TileVisual;

/// Resource pour stocker les scènes partagées
#[derive(Resource)]
pub struct SharedMeshes {
    pub meshes: HashMap<TileId, Handle<Scene>>,
}

pub fn setup_shared_meshes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut meshes = HashMap::new();

    // City
    meshes.insert(
        CITY,
        asset_server.load("tiles/city/tiles_ville.gltf#Scene0"),
    );
    meshes.insert(
        CITY_TOP,
        asset_server.load("tiles/city/tile_city_2.gltf#Scene0"),
    );
    // meshes.insert(
    //     (TileType::City, 2),
    //     asset_server.load("tiles/city/tile_city_2.gltf#Scene0"),
    // );

    // Rock
    meshes.insert(
        ROCK_TOP,
        asset_server.load("tiles/rock/tile_rock.gltf#Scene0"),
    );
    meshes.insert(
        ROCK,
        asset_server.load("tiles/rock/tile_rock_2.gltf#Scene0"),
    );
    // meshes.insert(
    //     (TileType::Rock, 2),
    //     asset_server.load("tiles/rock/tile_rock_3.gltf#Scene0"),
    // );

    // River
    meshes.insert(
        RIVER,
        asset_server.load("tiles/water/tile_water.gltf#Scene0"),
    );

    // Trunk
    meshes.insert(
        TRUNK,
        asset_server.load("tiles/tronc/tile_tronc.gltf#Scene0"),
    );

    // Field
    meshes.insert(
        FIELD,
        asset_server.load("tiles/field/tile_champ.gltf#Scene0"),
    );

    commands.insert_resource(SharedMeshes { meshes });
}

pub fn render_tiles_system(
    mut commands: Commands,
    mut regenerate_event: EventReader<GenerateWorldEvent>,
    wfc_controller: Res<WfcController>,
    shared_meshes: Res<SharedMeshes>,
    hexes: Query<(Entity, &Hex, &Transform)>, // Include Entity in the query
    existing_visuals: Query<Entity, With<TileVisual>>, // Query for entities with TileVisual
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in regenerate_event.read() {
        for entity in existing_visuals.iter() {
            commands
                .entity(entity)
                .remove::<TileVisual>()
                .remove::<SceneRoot>();
        }

        // Add new visuals
        for (entity, hex, transform) in hexes.iter() {
            if let Some(cell) = wfc_controller.grid.get_hex_cell(hex) {
                if let Some(tile) = cell.get_collapsed_tile() {
                    if let Some(scene_handle) = shared_meshes.meshes.get(&tile) {
                        commands.entity(entity).insert((
                            TileVisual,
                            SceneRoot(scene_handle.clone()),
                            Transform::from_xyz(
                                transform.translation.x,
                                TILE_HEIGHT / 2.0 + hex.level as f32 * (TILE_HEIGHT + TILE_GAP),
                                transform.translation.z,
                            )
                            .with_scale(Vec3::splat(2.0)),
                        ));
                    }
                }
            }
        }
    }
}
