use crate::components::clouds::{Cloud, CloudBillboard};
use crate::globals::BOID_ZONE_SIZE;
use crate::resources::clouds::{CloudAssets, CloudSettings};
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_clouds(
    mut commands: Commands,
    cloud_settings: Res<CloudSettings>,
    cloud_assets: Res<CloudAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    // Créer le matériau partagé pour tous les nuages
    let cloud_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, cloud_settings.opacity),
        base_color_texture: Some(cloud_assets.cloud_texture.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    // Distribuer les nuages uniformément sur la map
    let grid_size = (cloud_settings.cloud_count as f32).sqrt().ceil() as i32;
    let spacing = BOID_ZONE_SIZE / grid_size as f32;

    for i in 0..grid_size {
        for j in 0..grid_size {
            // Position de base en grille avec variation aléatoire
            let base_x = -BOID_ZONE_SIZE / 2.0 + (i as f32 + 0.5) * spacing;
            let base_z = -BOID_ZONE_SIZE / 2.0 + (j as f32 + 0.5) * spacing;

            let position = Vec3::new(
                base_x + rng.random_range(-spacing * 0.3..spacing * 0.3),
                rng.random_range(cloud_settings.min_height..cloud_settings.max_height),
                base_z + rng.random_range(-spacing * 0.3..spacing * 0.3),
            );

            // Variation de taille pour plus de naturel
            let scale_variation = rng.random_range(0.7..1.3);
            let scale = cloud_settings.cloud_scale * scale_variation;

            // Vitesse de dérive individuelle
            let drift_speed = cloud_settings.wind_speed * rng.random_range(0.8..1.2)
                + Vec3::new(
                rng.random_range(-0.5..0.5),
                0.0,
                rng.random_range(-0.2..0.2),
            );

            commands.spawn((
                Mesh3d(cloud_assets.cloud_mesh.clone()),
                MeshMaterial3d(cloud_material.clone()),
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(scale))
                    .with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))),
                Cloud {
                    base_position: position,
                    drift_speed,
                    oscillation_offset: rng.random_range(0.0..std::f32::consts::TAU),
                },
                CloudBillboard,
            ));
        }
    }

    info!("Spawned {} clouds", grid_size * grid_size);
}

pub fn setup_cloud_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Créer un mesh de plan pour les nuages
    let cloud_mesh = meshes.add(Rectangle::new(1.0, 0.6));

    let cloud_assets = CloudAssets {
        cloud_texture: asset_server.load("textures/clouds/cloud.png"),
        cloud_mesh,
    };

    commands.insert_resource(cloud_assets);
}