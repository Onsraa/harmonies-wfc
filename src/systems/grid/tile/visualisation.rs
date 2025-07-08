use crate::components::boid::Obstacle;
use crate::components::grid::hex::Hex;
use crate::components::spatial::ObstacleInKDTree;
use crate::globals::{HEX_SIZE, TILE_GAP, TILE_HEIGHT};
use crate::systems::wfc::v2::wfc::WfcSharedState;
use crate::wfc::v2::cell::TileId;
use crate::wfc::v2::{CITY, CITY_TOP, FIELD, LEAVES, RIVER, ROCK, ROCK_TOP, TRUNK};
use bevy::prelude::*;
use bevy::scene::Scene;
use std::collections::HashMap;

#[derive(Component)]
pub struct TileVisual;

#[derive(Resource)]
pub struct SharedMeshes {
    pub meshes: HashMap<TileId, Handle<Scene>>,
}

#[derive(Event)]
pub struct ResetGridVisuals;

#[derive(Event)]
pub struct UpdateGridVisuals;

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

    // Rock
    meshes.insert(
        ROCK_TOP,
        asset_server.load("tiles/rock/tile_rock.gltf#Scene0"),
    );
    meshes.insert(
        ROCK,
        asset_server.load("tiles/rock/tile_rock_2.gltf#Scene0"),
    );

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
    // LEAVES
    meshes.insert(
        LEAVES,
        asset_server.load("tiles/leaves/tile_leaf.gltf#Scene0"),
    );

    // Field
    meshes.insert(
        FIELD,
        asset_server.load("tiles/field/tile_champ.gltf#Scene0"),
    );

    commands.insert_resource(SharedMeshes { meshes });
}

pub fn reset_grid_visualization(
    mut commands: Commands,
    mut reset_event: EventReader<ResetGridVisuals>,
    mut existing_visuals: Query<(Entity, &Children), With<TileVisual>>,
    wfc_shared: Option<Res<WfcSharedState>>,
) {
    for _ in reset_event.read() {
        println!(
            "Resetting grid visuals - found {} visual entities",
            existing_visuals.iter().count()
        );

        for (entity, children) in existing_visuals.iter_mut() {
            for child in children.iter() {
                commands.entity(child).despawn();
            }

            commands
                .entity(entity)
                .remove::<TileVisual>()
                .remove::<SceneRoot>()
                .remove::<Obstacle>()
                .remove::<ObstacleInKDTree>()
                .remove::<Children>(); // Also remove the Children component
        }
    }
}

pub fn render_new_tiles(
    mut commands: Commands,
    mut update_event: EventReader<UpdateGridVisuals>,
    wfc_shared: Option<Res<WfcSharedState>>,
    shared_meshes: Res<SharedMeshes>,
    hexes: Query<(Entity, &Hex, &Transform), Without<TileVisual>>,
) {
    for _ in update_event.read() {
        let Some(shared_state) = wfc_shared.as_ref() else {
            println!("No WFC shared state available yet");
            return;
        };

        let Ok(grid_guard) = shared_state.grid.try_lock() else {
            println!("Could not lock WFC grid (busy)");
            return;
        };

        let Some(grid) = grid_guard.as_ref() else {
            println!("No WFC grid available yet");
            return;
        };

        for (entity, hex, transform) in hexes.iter() {
            if let Some(cell) = grid.get_hex_cell(hex) {
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
                            Obstacle {
                                radius: HEX_SIZE * 1.5,
                            },
                            ObstacleInKDTree,
                        ));
                    }
                }
            }
        }
    }
}
